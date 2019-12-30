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

    println!("{} {} {}", ONE, TWO, three);
}
