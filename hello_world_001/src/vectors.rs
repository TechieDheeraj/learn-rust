// Vectors: Resizable Arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    numbers.push(5);
    numbers.push(6);
    numbers.pop(); // pop the last value

    numbers[2] = 20;
    println!("Vector Length: {}", numbers.len());

    println!("Vector uses: {} bytes", mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[1..5];
    println!("Slice: {:?}", slice);

    // Loop through vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers: {:?}", numbers);

}
