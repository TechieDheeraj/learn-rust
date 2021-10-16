// Premitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated DS - use when you need to modify

pub fn run() {
    let hello = "Hello"; // immutable

    // let hello_t = String::from("Hello "); // By default immutable variable
    let mut hello_t = String::from("Hello "); // By default immutable variable
    // hello.push('W'); // can't push as fixed length
   
    // Push Char
    hello_t.push('W');

    // Push String 
    hello_t.push_str("orld!");

    // capacity in bytes 
    println!("Capacity {}", hello_t.capacity());
    println!("IsEmpty {}", hello_t.is_empty());
    println!("Contains:  {}", hello_t.contains("World"));
    println!("Replace:  {}", hello_t.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_t.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{:?}", (hello, hello.len(), hello_t.len(), hello_t, &s));

    // Assertion
    assert_eq!(2, s.len()); // assert if s.len() is not equal to 2
}
