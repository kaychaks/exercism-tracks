use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Clock(i32, i32);

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

fn adjust_hour(hrs: i32) -> i32 {
    match hrs.abs() {
        24 => 0,
        0..=23 => hrs,
        _ => hrs % 24,
    }
}

fn adjust_min(mins: i32) -> (i32, i32) {
    match mins.abs() {
        60 => (mins / 60, 0),
        0..=59 => (0, mins),
        _ => (adjust_hour(mins / 60), mins % 60),
    }
}

fn check_hr_neg(hr: i32) -> i32 {
    if hr.is_negative() {
        24 - hr.abs()
    } else {
        hr
    }
}

fn check_min_neg(min: i32, hr: i32) -> (i32, i32) {
    if min.is_negative() {
        let h = check_hr_neg(adjust_hour(hr - 1));
        let m = 60 - min.abs();
        (h, m)
    } else {
        (hr, min)
    }
}

fn proc(hr: i32, min: i32) -> (i32, i32) {
    let (mut h, m) = adjust_min(min);
    h = check_hr_neg(adjust_hour(hr + h));

    check_min_neg(m, h)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut x = Clock(0, 0);

        x.0 = check_hr_neg(adjust_hour(hours));

        let (a, b) = proc(x.0, minutes);

        x.0 = a;
        x.1 = b;
        x
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut x = *self;

        let (h, m) = adjust_min(minutes);

        x.0 += h;
        x.1 += m;

        let (a, b) = proc(x.0, x.1);

        x.0 = a;
        x.1 = b;
        x
    }
}
