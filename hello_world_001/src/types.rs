/*
 * Primitive types:
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Typles
 * Arrays
 */

pub fn run() { 
    // By default it will be i32
    let x = 1;

    // By default it will be f64
    let y = 2.5;

    let z:i64 = 34423423432;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 4;

    let a1 = 'a';
    let face = '\u{1F600}'; // unicode characters

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
