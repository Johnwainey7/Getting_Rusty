mod if_else;

fn main() {
    let x = 2.1;  //Immutable by default and f64 by default
    let mut y: f32 = 3.0;  //Explicityy mutable and f32 by default (single percision)
    let a: i8 = -128; //8 bit signed integer
    let b: u8 = 255; //8 bit unsigned integer
    let c: i32 = -50_000; //32 bit signed integer
    let d: u64 = 1_000_000_000_000_000_000; //64 bit unsigned integer

    let t = true;
    let f: bool = false; //with explicit type annotation

    //Boolean operations
    let is_morning = true;
    let is_weekday = false;
    let go_to_work = is_weekday && is_morning; //&& only checks if both are true

    println!("Go to work: {}", go_to_work);
    
    let n ='z';
    let z: char = 'ℤ'; //Unicode character
    let heart_eyed_cat = '😻'; //Unicode character
    println!("n: {}, z: {}, heart_eyed_cat: {}", n, z, heart_eyed_cat);

    //Rust can infer types from context
    let mut jessie = String::new();  //string type inferred

    let s1: &str = "Hello, Johnny"; // string slice, fixed size, lives in binary
    let mut s2: String = String::from("Hello, World"); // Owned string, Heap-allocated, can grow
    let name: String = String::from("Rust");
    s2.push_str(", Rust is great!");
    println!("name: {}", name);
    println!("s1: {}, s2: {}", s1, s2);
    println!("name: {}", name);
    //Sometimes context helps with inference
    let mut numbers: Vec<f32> = Vec::new(); //Vector of i32s
    let parsed: u32 = "42".parse().expect("Not a number");
    numbers.push(1.0);
    numbers.push(200.0);
    println!("parsed: {}", parsed);
    println!("numbers: {:?}", numbers);

    //Type inference -Rust can often determine types automatically
    let guess = 42; //Rust infers that guess is an i32
    println!("a: {}, b: {}, c: {}, d: {}, guess: {}", a, b, c, d, guess);

    //Constants are always immutable and must have type annotations
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // Can be shadowed (redeclared)
    let sum = x + y as f64; //Type casting required
    let difference = 95.5 - 4.3;
    let product = 4.0 *30.0;
    let quotient = 56.7 / 32.2;

    println!("sum: {}, difference: {}, product: {}, quotient: {}",
     sum, difference, product, quotient);

    println!("x + y = {}", x + y as f64);
    println!("It's getting a little rusty in here");

    println!("x: {}, y: {}", x, y);


    println!("I am Lejohn D'Dun Wainford");
    let greet_user = |name: &str| {
        println!("Hello, {}", name);
    };
    greet_user("Lejohn");


    let (sum, product) = calculate(4, 6);
    println!("Sum: {}, Product: {}", sum, product);

    // Calls to functions in the `if_else` module
    if_else::if_else();
    if_else::if_as_expression();

    // Infinite loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //Return value from loop
        }
    };
    println!("Result: {}", result);

    //while loop
    let mut number = 3;
    while number != 0 {
        println!( "{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //For loop with range
    for number in 1..4 { // 1, 2, 3 (4 is excluded)
        println!("{}", number);
    }

    // for loop with arry
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is : {}", element);
    }

    //for loop with index
    for (index, value) in a.iter().enumerate() {
        println!("the value at index {} is: {}", index, value);
    }

    //this is a line comment
    /*
    *This is a block comment
    *spanning multiple lines
     */
    println!("Result: {}", add(2, 3));
}

fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
///This function adds two numbers together
/// 
/// #Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result,5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}