fn main() {
    // let reference_to_nothing = dangle_error();
    let reference_to_nothing = no_dangle();
    println!("value = {}", reference_to_nothing);
}

/*
fn dangle_error() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello, there");

    s // Ownership is moved out, and nothing is deallocated.
}