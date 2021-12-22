use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock(i32);

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}

const DAY: i32 = 24 * 60;
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock((hours * 60 + minutes).rem_euclid(DAY))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock((self.0 + minutes).rem_euclid(DAY))
    }
}
