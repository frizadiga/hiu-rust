pub fn run(name: &'static str) {
    // Formatting
    println!("Hello, world! {} do you like {}", name, object = "Soto Padang");

    // Positional
    println!("{0} dont like be interfere with {1}, be like {0}", "Cats", "Human");

    // Placeholder Trait
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debugging
    println!("Debug: {:?}", (10, true));

    // Eval Expression
    println!("10 x 10 = {}", 10 * 10);
}