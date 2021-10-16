pub fn run() {
    println!("Hello World from print.rs file");
    println!("Number : {}", 1);

    // positinal arguments
    println!("Name: {0}, LastName: {1}, Name: {0}", "Dheeraj", "Kakkar");
    
    // Named Arguments
    println!("{name} likes to play {activity}", name="Dheeraj", activity="Badminton");

    // placeholder Arguments
    println!("Binary {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    
    // placeholder for debug triats
    println!("{:?}", (12, true, "Hello"));

    // Math operations 
    println!("10 + 10 = {}", 10+10);
}
