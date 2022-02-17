use std::{cmp::min, collections::HashMap, sync::mpsc::channel, thread, vec};

fn process(s: &str) -> Vec<(char, usize)> {
    s.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some((c, 1))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let len = input.len();
    let wc = min(len, worker_count);

    if input.is_empty() {
        HashMap::new()
    } else {
        let (super_tx, super_rx) = channel();

        let thread_pool = (0..wc)
            .map(|_| {
                let super_tx = super_tx.clone();
                let (child_tx, child_rx) = channel();
                thread::spawn(move || {
                    let mut p_cs: Vec<(char, usize)> = vec![];
                    for s in child_rx {
                        match s {
                            Some(x) => {
                                p_cs.append(&mut process(x));
                            }
                            None => {
                                super_tx.send(p_cs).unwrap();
                                break;
                            }
                        }
                    }
                });
                child_tx
            })
            .collect::<Vec<_>>();

        input
            .iter()
            .zip(thread_pool.iter().cycle())
            .for_each(|(&s, child_tx)| {
                child_tx.send(Some(s)).unwrap();
            });

        let mut r: HashMap<char, usize> = HashMap::new();
        for child_tx in thread_pool.iter() {
            child_tx
                .send(None)
                .expect("stop sending any more values to child channel");
            r = match super_rx.recv() {
                Ok(v) => v.iter().fold(r, |mut acc, (c, count)| {
                    acc.entry(*c).and_modify(|x| *x += 1).or_insert(*count);
                    acc
                }),
                _ => r,
            }
        }
        r
    }
}
