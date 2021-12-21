use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + (10_i64.pow(9)).seconds()
}
