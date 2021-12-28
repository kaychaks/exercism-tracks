#[macro_export]
macro_rules! space_age {
  (
      $($planet:ident -> $distance:tt),+
  ) => {

    $(
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $distance
            }
        }
    )+
  }
}
