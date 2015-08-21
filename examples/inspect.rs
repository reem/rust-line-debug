#[macro_use(inspect)]
extern crate inspect;

fn main() {
    let a = 7;
    inspect!(a, a + 4, a - 3);
}
