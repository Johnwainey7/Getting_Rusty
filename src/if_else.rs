pub fn if_else() {
    use super::*;
    let number = 6;

    //Basic if else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0 {
        println!("number is divisible by 3");
    }else if number % 2 == 0 {
        println!("number is divisible by 2");
    }else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//if as an expression
pub fn if_as_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);
}

