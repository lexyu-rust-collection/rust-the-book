fn main() {
    {                           // s is not valid here, it’s not yet declared
        let s1 = "hello"; // s is valid from this point forward

        println!(" s1 = {}", s1);
        println!(" s1 = {:p}", &s1);
        println!("-------------------------------------------------------------");

        let s5 = test(s1);
        println!("-------------------------------------------------------------");

        the_second_string_type(s5);
        println!("-------------------------------------------------------------");

        heap_clone();
        println!("-------------------------------------------------------------");

        listing_4_2();
    }                            // this scope is now over, and s is no longer valid
}

fn test(s1: &str) -> &str {
    let s2 = s1;

    println!(" s2 = {}", s2);
    println!(" s2 = {:p}", &s2);

    let s3 = s2;

    println!(" s3 = {}", s3);
    println!(" s3 = {:p}", &s3);

    let s4 = "test";

    println!(" s4 = {}", s4);
    println!(" s4 = {:p}", &s4);

    let s5 = s4;

    println!(" s5 = {}", s5);
    println!(" s5 = {:p}", &s5);

    return s5
}

fn the_second_string_type(s: &str) {
    println!("{:p}" , s);

    let mut s = String::from("Hello");
    
    println!("s address = {:p}", &s);
    
    s.push_str(", world!");

    println!("s address = {:p}", &s);

    println!("s = {}", s);
}

// Variables and Data Interacting with Move
fn figure_4_4() {
    
    // error: borrow of moved value: `s1`
    // label: move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    /* 
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);  // Rust invalidates the first variable, instead of being called a shallow copy
     */

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
}

// Variables and Data Interacting with Clone
fn heap_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s1 addr = {:p}, s2 addr = {:p}", &s1, &s2);
}

// Stack-Only Data: Copy
fn listing_4_2() {
    let x = 5;
    let y = x; // integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.
    println!("x = {}, y = {}", x, y);
}