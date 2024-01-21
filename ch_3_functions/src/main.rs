fn main() {
    println!("Hello, world!");

    another_func(-10);

    print_labeled_measurement(5, 'h');

    //let x = (let y = 6); // error

    // expression, do not include ending semicolons.
    let z = {
        let x = 3;
        x + 1
    };
    println!("z = {z}");

    // Functions with Return Values
    let o = ten();
    println!("o = {o}");

    let k = plus_one(5);
    println!("k = {k}");
}

fn another_func(x: i32) {
    println!("Another function");
    println!("The value of x = {x}");
}

fn print_labeled_measurement(val: i32, unit_lable: char) {
    println!("Measurement = {val}{unit_lable}");
}

fn ten() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // error
}
