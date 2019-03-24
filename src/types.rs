/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run () {
    println!("types");

    // Default to i32
    let int = 1;
    // Default to f64
    let float = 2.7;

    println!("int: {} float: {}", int, float);

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_open: bool = true;
    println!("is_open: {}", is_open);

    // From Expression
    let is_greater_than_100 = 18 > 100;
    println!("is_greater_than_100: {}", is_greater_than_100);
}