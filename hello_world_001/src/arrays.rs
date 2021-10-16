// Arrays - Fixed List where elements are the same data types

use std::mem;

pub fn run() {
 //   let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // immutable
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    numbers[2] = 20;
    println!("numbers[2]: {}", numbers[2]);

    //Arrays len 
    println!("Array len: {}", numbers.len());

    //Arrays Occupies 
    println!("Array occupies: {} bytes", mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
