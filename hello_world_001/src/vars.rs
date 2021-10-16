// Variables are immutable by default
// Rust is block-scoped language

pub fn run() {
    let name = "Dheeraj";
    //let age = 26; // immutable variable
    let mut age = 26; // mutable variable
    println!("Name is : {} and I am {}", name, age);
    age = 28;
    println!("Name is : {} and I am {}", name, age);

    // Define Constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Dheeraj", 26);
    println!("My Name: {}, My Age: {}", my_name, my_age);
}
