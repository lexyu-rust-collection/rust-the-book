fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);

    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}, y = {}", x, y);

    // Numeric Operations
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5;

    println!(
        "sum = {}, difference = {},
        product = {}, quotient = {},
        truncated = {}, remainder = {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // The Boolean Type
    let t = truncated;
    let f: bool = false;
    println!("t={},f={}", t, f);

    // The Character Type, 4 bytes
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😀';
    println!("c={},z={},heart_eyed_cat={}", c, z, heart_eyed_cat);

    // Compound Types
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x = {x}, y = {y}, z  = {z}");
    println!("x = {}, y = {}", tup.0, tup.1);
    // array
    // let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit
    println!("a = {:?}", a);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months = {:?}", months);

    let b = [10; 5];
    println!("b = {:?}", b);

    // Accessing Array Elements
    let index_0 = a[0];
    let index_1 = a[1];
    println!("a[0] = {}, a[1]= {}", index_0, index_1)
}
