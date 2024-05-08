struct S {
    x: i32,
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

fn main() {
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
}
