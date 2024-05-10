use std::io;
use std::env;
use rand::prelude::*;
use std::fs;
use std::io::prelude::*;

struct S {
    x: i32,
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

fn get_y(p: Point) -> u8 {
    p.1
}

#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

const S:S = S{x:2};

fn run() {
    let v = &mut S;
    v.x +=1;
    S.x +=1;

    println!("{}, {}", v.x, S.x);
}

fn sum_and_mul(a: u8, b: u8) -> (u8, u8) {
    let sum = a + b;  
    let mul = a*b;
    return (sum, mul);
}

fn print_in_loop(mut count :u8) {
    // Infinte loop statement without condition in expression 
    let result = loop {
        if count == 1 {
            break count * 5;
        }
        println!("printing value {}", count);
        count-=1;
    };
    println!("After the loop!\nresult is {}", result);


    let message = ['a', 'b', 'c', 'd', 'e', 'f'];
    for (index, &item) in message.iter().enumerate() {
        println!("index is {}, item is {}", index, item);
        if item == 'd' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }
}

fn print_matrix() {
    let mut matrix = [[1, 2, 3],
                                [4, 5, 6],
                                [7, 8, 9]];
    
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num+=10;
            print!("{}\t", num);
        }
        print!("\n");
    }

    let planet = "Earth";
    println!("Planet is {}", planet);
    let mut planet = 4; // shadow the variable with new type
    planet += 1;
    println!("Planet is {}", planet);
}

fn explain_ownership(mut test_var: i32, test_str: String, new_str: &mut String) -> String{
    let mut mess = String::from("Earth"); // stored in heap
    println!("mess is: {}", mess);
    mess.push_str(" is home");
    println!("mess is: {}", mess);

    let mut var1 : u8;
    {
        let mut var2 = 10;
        var1 = var2;
        println!("Var1 is {}", var1);
        println!("Var2 is {}", var1);
        // For Stack only data types, integer, float-points -> copy occurs implictly; cloning must be done explicitly, no concept of ownership 
    }
    let outer_p: String;
    {
        let mut inner_p = String::from("hello");
        outer_p = inner_p.clone(); // allocates new space on heap and copy content
        inner_p.clear();
        /*
        outer_p = inner_p;
            // Can't use because ownership is passed to outer_p as move implicit, and in rust only one owner should be there
        println!("inner_p is {}", inner_p); 
        */
        println!("inner_p is {}", inner_p); 
    }
    println!("outer_p is {}", outer_p);
    test_var+=1; // t_var and test_var have diff ownership because data is copied as variable resides on stack
    println!("test var is {} and test str is {}, new_str is {}", test_var, test_str, new_str);
    new_str.push_str(" Adding...");
    test_str
}

fn explain_slice(s: &str) -> &str {
    let input = String::from("Hello World !");
    let slice_var = &input[6..6+3];
    println!("slice var is {}", slice_var);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets : &[i32] = &planets[..4];
    println!("inner planets are: {:?}", inner_planets);

    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    return &s;
}

fn take_input() {
    let mut buffer = String::new();
    println!("Enter the message");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number :i32 = buffer.trim().parse().unwrap();
    println!("number is {}", number);
}

fn main() {
    if env::args().len() <= 2 {
        println!("program requires atleast 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("index {}, argument is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap(); 
    println!("2nd argument is {}", arg2);

    let mut x: u8 = 254;
    x +=1;
    let fl: f64 = 10.22;
    println!("x is {}, {}", x, fl);
    println!("Hello, world!");

    let a: i32 = 10;
    let b: f64  = 10.2;

    let c = a as f64 /b; 
    println!("c is {0:08.3}, a is {1}", c, a);

    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    value  = !value;
    println!("value is {:08b}", value);
    value  = value & 0b1111_0111;
    println!("value is {:08b}", value);

    let finger  = '\u{261D}';
    println!("finger : {}", finger);
    run();
    let letters = ['a', 'b', 'c'];
    println!("first letter {}", letters[0]);

    let numbers: [i32; 5];
    numbers = [0;5];
    println!("last number is {}", numbers[4]);

    let mut stuff:(u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first item is {}", first_item);
    let (a, b, c) = stuff;
    println!("first item is {}, {}, {}", a, b, c);

    let mut v = vec![0, 1];
    let mut_ref_v = &mut v;
    mut_ref_v.push(2);
    println!("{}", v[2]);

    let x = 10; 
    let y = 13;
    /* compiler knows that we are passing x, y to sum;
       So declaring x,y as u8
    */
    println!("result is {:?}", sum_and_mul(x, y)); // way to print tuple

    let count:u8 = 5;
    print_in_loop(count);
    print_matrix();
    let t_var = 1;
    let t_string = String::from("Hello");
    let mut new_str = String::from("World");
    // Below String is stored on heap so to transfer ownership and get it back need to return, & is used for borrow reference
    // If we have mutable reference to variable (new_str), rust will not allow us to create any other reference (neither mutable nor unmutable)
    // We can have as many as unmutable references to a variable
    let t_string = explain_ownership(t_var, t_string, &mut new_str);
    println!("t_var is {} and t_string is {}, new_str is {}", t_var, t_string, new_str);
    let input_st = String::from("Hello world, how are you ?");
    let first_word = explain_slice(&input_st[..10]); // slice can accept both slice and string inside explain_slice func
    println!("first word is {}", first_word);
    take_input();

    let number  = random::<f64>();
    let guess = thread_rng().gen_range(1..11);
    println!("number is {}, guess is {}", number, guess);

    let file_read = fs::read_to_string("src/input.txt").unwrap(); 
    println!("contents is {}", file_read);

    for line in file_read.lines() {
        println!("line is {}", line);
    }
    let file_read = fs::read("src/input.txt").unwrap(); 
    println!("contents is {:?}", file_read);

    let mut speech = String::new();
    speech.push_str("We choose to go to moon this decade\n");
    speech.push_str("Is there any challenge in that ?\n");

    fs::write("src/speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("src/input.txt").unwrap();
    file.write(b"\nTony");

    let mut vehicle = Shuttle {
        name: String::from("Endaevour"),
        crew_size: 7,
        propellant: 83599.0
    };

    println!("shuttle name is {}", vehicle.name);
    println!("vehicle object is {:?}", vehicle);

    // Move from existing object apart from name
    // If first object vehicle tries to modify the string which stores on heap
    // will get error as the ownership is changed
    let vehicle2 = Shuttle {
        ..vehicle.clone()
    };
    vehicle.crew_size = 7; // this can be updated since the fixed size is on the stack
    // Moved ownership to vehicle2 object (use clone to overcome this)
    // println!("vehicle object is {:?}", vehicle);
    println!("vehicle object is {:?}", vehicle);
    println!("vehicle2 object is {:?}", vehicle2);

    let vehicle_name = vehicle.get_name();
    println!("vehicle name is {}", vehicle_name);
    vehicle.add_fuel(1000.0);
    println!("new propellent is {}", vehicle.propellant);

    let vehicle3 = Shuttle::new("Ford");
    println!("new vehicle3 is {:?}", vehicle3);

    let red = Color(255,0,0);
    let coord = Point(1,2,3);

    let y = get_y(coord);
    println!("y is {}", y);

}
