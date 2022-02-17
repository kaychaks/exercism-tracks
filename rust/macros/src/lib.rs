#[macro_export]
macro_rules! hashmap {
    (@internal $($key:expr => $value:expr),*) => {
        {
            let mut hm = crate::HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
    ($($key:expr => $value:expr),+,) => {
        crate::hashmap!(@internal $($key => $value),*);
    };
    ($($key:expr => $value:expr),*) => {
        crate::hashmap!(@internal $($key => $value),*);
    };
}
