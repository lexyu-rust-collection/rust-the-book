fn main() {
    {                           // s is not valid here, it’s not yet declared
        let s1 = "hello"; // s is valid from this point forward

        println!(" s1 = {}", s1);
        println!(" s1 = {:p}", &s1);

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

        println!("-------------------------------------------------------------");

        the_second_string_type(s5);
    }                                   // this scope is now over, and s is no longer valid
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
fn check() {
    
    // error: borrow of moved value: `s1`
    // label: move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    /* 
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
     */
}
