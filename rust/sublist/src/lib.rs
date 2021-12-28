use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_contained<T: PartialEq>(fl: &[T], sl: &[T], sz: usize) -> bool {
    let l = fl.len();
    if l == 0 || sz == 0 {
        true
    } else {
        sl.windows(sz).any(|xs| xs == fl)
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let l = first_list.len();
    let m = second_list.len();

    match l.cmp(&m) {
        Ordering::Equal => {
            if is_contained(first_list, second_list, l) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if is_contained(first_list, second_list, l) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if is_contained(second_list, first_list, m) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
