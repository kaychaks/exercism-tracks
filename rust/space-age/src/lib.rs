// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

mod space_age_macro;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(((s as f64 / 31557600_f64) * 100.0).round() / 100.0)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0
    }
}

space_age! (
    Mercury -> 0.2408467,
    Venus -> 0.61519726,
    Earth -> 1.0,
    Mars -> 1.8808158,
    Jupiter -> 11.862615,
    Saturn -> 29.447498,
    Uranus -> 84.016846,
    Neptune -> 164.79132
);
