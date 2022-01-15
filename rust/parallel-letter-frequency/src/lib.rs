use std::{collections::HashMap, sync::mpsc::channel, thread};

fn split_evenly<T>(n: usize, xs: &[T]) -> impl Iterator<Item = &[T]> {
    struct _Iter<'a, T>(usize, &'a [T]);

    impl<'a, T> Iterator for _Iter<'a, T> {
        type Item = &'a [T];

        fn next(&mut self) -> Option<Self::Item> {
            if self.1.len() == 0 {
                None
            } else if self.0 == 0 {
                Some(self.1)
            } else {
                let extra = if self.1.len() % self.0 == 0 { 0 } else { 1 };
                let mid = self.1.len() / self.0 + extra;
                let (first, next) = self.1.split_at(mid);
                self.1 = next;
                self.0 -= 1;
                Some(first)
            }
        }
    }

    _Iter(n, xs)
}

fn process_str(inp: Vec<String>) -> Vec<(char, usize)> {
    inp.iter()
        .flat_map(|xs| {
            xs.to_lowercase()
                .chars()
                .filter_map(|c| {
                    if c.is_alphabetic() {
                        Some((c, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        HashMap::new()
    } else {
        let vss: Vec<Vec<String>> = split_evenly(worker_count, input)
            .collect::<Vec<&[&str]>>()
            .iter()
            .map(|&a| a.iter().map(|&a| String::from(a)).collect())
            .collect();

        let vss_c = vss.clone();
        let (tx, rx) = channel();

        vss.into_iter().for_each(|s| {
            let tx = tx.clone();
            thread::spawn(move || {
                let cs: Vec<(char, usize)> = process_str(s);
                tx.send(cs).unwrap();
            });
        });
    

    let mut r: HashMap<char, usize> = HashMap::new();
    for _ in vss_c {
        r = match rx.recv() {
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
