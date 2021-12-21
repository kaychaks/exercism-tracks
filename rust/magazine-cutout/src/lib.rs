// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

type Count = u8;

#[derive(Eq, PartialEq, Debug)]
struct MyHash<'a>(HashMap<&'a str, Count>);

impl<'a> MyHash<'a> {
    fn from(h: HashMap<&'a str, Count>) -> Self {
        MyHash(h)
    }
}

fn parse_text<'a>(xs: &[&'a str]) -> MyHash<'a> {
    MyHash::from(xs.iter().zip(std::iter::repeat(1).take(xs.len())).fold(
        HashMap::new(),
        |mut m, (k, v)| {
            m.entry(k).and_modify(|e| *e += 1).or_insert(v);
            m
        },
    ))
}

impl<'a> PartialOrd for MyHash<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.0.iter().find(|(x, c)| match self.0.get(**x) {
            Some(d) => d < c,
            _ => true,
        }) {
            Some(_) => Some(std::cmp::Ordering::Less),
            _ => Some(std::cmp::Ordering::Greater),
        }
    }
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    parse_text(magazine) >= parse_text(note)
}
