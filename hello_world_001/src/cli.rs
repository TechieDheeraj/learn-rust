pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "dheeraj";
    let status = "100%";

    println!("Args: {:?}", command);
    if command == "hello" {
        println!("Hi {}, How are you ?", name); 
    } else if command == "status" {
        println!("Status is {}", status); 
    }
}
