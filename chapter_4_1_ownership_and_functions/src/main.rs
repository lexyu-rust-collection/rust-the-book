fn main() {
    let s = String::from("hello");  // s comes into scope
    println!("s = {:p}", &s);
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("s = {}", s); // value borrowed here after move rustc

    let x = 5;                      // x comes into scope
    println!("x = {:p}", &x);
    makes_copy(x);                  // x would move into the function,   
    println!("x = {:p}", &x);
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
    println!("some_string = {:p}", &some_string);
} // Here, some_string goes out of scope and `drop` is called. 
  // The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
    println!("some_integer = {:p}", &some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
