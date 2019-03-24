// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run () {
    // String like
    let text = "Some value";
    println!("value of text: {}", text);

    // Number like (i32 actually)
    let mut number = 70;
    println!("value of number: {:?}", number);
    number = 100;
    println!("value of number: {:?}", number);

    // Constant
    const ID: i32 = 123;
    println!("value of ID: {}", ID);
    
}