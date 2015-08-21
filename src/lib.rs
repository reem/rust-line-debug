#![feature(core_intrinsics)]
#![deny(missing_docs)]

//! A lightweight library for quickly debugging rust code.

use std::fmt::Debug;

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
                concat!($(stringify!($a), " = {}, "),*), $(::inspect::Inspect::inspect(&$a)),*
            )
        );
    }
);

/// Helpful function for printing a value's inspect string.
///
/// # Examples
///
/// ```
/// use inspect::{p, Inspect};
///
/// p(43);  // stdout=> [i32] 43
/// ```
pub fn p<T: Inspect>(value: T) {
    println!("{}", value.inspect());
}

/// Inspect trait for reflecting on values.
pub trait Inspect {
    /// Returns a string describing a value and it's type.
    ///
    /// # Examples
    ///
    /// ```
    /// use inspect::Inspect;
    ///
    /// assert_eq!(7.inspect(), "[i32] 7");
    /// ```
    fn inspect(&self) -> String;
}

impl<T: Debug> Inspect for T {
    fn inspect(&self) -> String {
        let type_name = unsafe { std::intrinsics::type_name::<T>() };
        format!("[{}] {:?}", type_name, self)
    }
}
