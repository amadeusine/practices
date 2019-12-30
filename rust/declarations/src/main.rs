/// multiple declaration statements possible
/// `let`, `const` and `static`
/// and opt-in `mut`ability
/// Here's a small example.

// This dwells in a global scope and lifetime
static ONE: &'static str = "Hello world";
// Caution when mutating static variables.
fn main() {
    // Static values hold static addresses, whereas const values are inlined in use.
    // The lifetime parameter is good, here's a primitive string reference.
    const TWO: &str = "can wait";
    // And here's a heap-allocated string.
    let three: _ = String::from("just a little bit longer. :)");

    println!("{} {} {}\n", ONE, TWO, three);

    // And more with `mut`
    let range = 1 << 10;
    let mut four: i32 = 0;
    // Accumulation
    for i in 1..=range {
        if i % 2 == 0 {
            four += i;
        }
    }
    // Or something much nicer... and faster.
    let five = (1..=range)
                .filter(|x| x % 2 == 0)
                .fold(0, |acc, x| acc + x);

    println!("Evens summed from 1 through {} are {} (by loop) and {} (by folding).\n", range, four, five);
}
