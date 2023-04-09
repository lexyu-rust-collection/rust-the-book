fn main() {
    // let x = 5;
    let mut x = 5; // add mut resolve error
    println!("The value of x is: {x}");
    x = 6; // error, cannot mutate immutable variable `x`
    println!("The value of x is: {x}");

    // Constants
    const ONE_DAY_IN_SECONDS: u32 = 24 * 60 * 60;
    println!("ONE_DAY_IN_SECONDS = {}", ONE_DAY_IN_SECONDS);

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    /*
    let mut spaces2 = "   ";
    spaces2 = spaces2.len(); // error
    println!("spaces2 = {}", spaces2)
    */
}
