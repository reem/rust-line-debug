#![deny(missing_docs)]

//! A lightweight library for quickly debugging rust code.

/// Logs the file, line number, and expressions along with their `Show` value.
///
/// # Examples
///
/// ```
/// # #[macro_use(inspect)]
/// # extern crate inspect;
/// fn main() {
///     let a = 7;
///     inspect!(a, a + 4); //=> file.rs - X: a = 7, a + 4 = 11
/// }
/// ```
///
#[macro_export]
macro_rules! inspect(
    ($($a:expr),*) => {
        println!(
            "{} - {}: {}",
            file!(), line!(),
            format!(
                concat!($(stringify!($a), " = {}, "),*), $($a),*
            )
        );
    }
);
