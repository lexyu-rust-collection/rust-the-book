fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    check_num();

    // Using if in a let Statement
    let cond = false;
    let number = if cond { 5 } else { 6 };
    println!("Number = {number}");

    // let number2 = if cond { 5 } else { "six" };
    // println!("Number = {number2}");
}

fn check_num() {
    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
