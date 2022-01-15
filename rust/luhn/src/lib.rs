fn dbl_and_check(x: u32) -> u32 {
    let i = x * 2;
    if i > 9 {
        i - 9
    } else {
        i
    }
}

#[derive(Debug, Clone)]
struct Input(Vec<char>, usize);

impl From<&str> for Input {
    fn from(x: &str) -> Self {
        Input(x.chars().rev().collect(), 0)
    }
}

impl Iterator for Input {
    type Item = Parsed;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        match self.1 {
            x if x < len => {
                let this = self.0.get(x);
                match this {
                    Some(sp) if (*sp).is_whitespace() => {
                        self.1 += 1;
                        Some(Parsed::Space)
                    }
                    Some(t) => (|x| {
                        self.1 += 1;
                        match x {
                            Some(v) => Some(Parsed::Digit(v)),
                            None => Some(Parsed::SomethingElse),
                        }
                    })(t.to_digit(10)),
                    None => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Parsed {
    Digit(u32),
    Space,
    SomethingElse,
}

impl Parsed {
    fn is_something_else(&self) -> bool {
        match self {
            Parsed::SomethingElse => true,
            _ => false,
        }
    }

    fn is_digit(&self) -> bool {
        match self {
            Parsed::Digit(_) => true,
            _ => false,
        }
    }

    fn get_digit(&self) -> Option<u32> {
        match self {
            Parsed::Digit(x) => Some(*x),
            Parsed::Space => None,
            _ => None,
        }
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let inp: Input = Input::from(code);
    let has_non_digit_non_space = inp.clone().any(|x| x.is_something_else());
    let only_digits = inp
        .clone()
        .filter(|x| x.is_digit())
        .map_while(|x| x.get_digit());

    if has_non_digit_non_space || only_digits.clone().count() <= 1 {
        false
    } else {
        let y: u32 = only_digits
            .clone()
            .enumerate()
            .map(|(id, el)| match id {
                _ if id % 2 != 0 => dbl_and_check(el),
                _ => el,
            })
            .sum();
        y % 10 == 0
    }
}
