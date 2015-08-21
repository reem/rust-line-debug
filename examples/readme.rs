#[macro_use(inspect)]
extern crate inspect;

fn main() {
    let a = 7u32;
    let b = 4u64;
    inspect!(a, b, a as u64 + b);
    // examples/readme.rs - 7: a = [u32] 7, b = [u64] 4, a as u64 + b = [u64] 11,
}
