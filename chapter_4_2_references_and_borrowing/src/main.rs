fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // reference

    println!("The length of '{}' is {}.", s1, len);
    println!("The addr of '{:p}', {:p}.", &s1, &len);

    let mut s = String::from("hello");
    
    change(&s);

    change_plus_mut(&mut s);

    situation_1();

    situation_2();
}

fn situation_2() {
    let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // --------------------------------------------------------------------------
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn situation_1() {
    let mut t = String::from("hi");

    // error
    // let r1 = &mut t;
 
    // let r2 = &mut t;
    // println!("r1 = {}, r2 = {}", r1, r2); // first borrow later used hererustc
    // --------------------------------------------------------------------------

    {
        let r1 = &mut t;
        println!("r1 = {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut t;
    println!("r2 = {}", r2);
    println!("--------------------------------------------------------------")
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    println!("s addr = {:p}", &s);
    println!("s.len() addr = {:p}", &s.len());
    println!("--------------------------------------------------------------");

    s.len()
} // Here, s goes out of scope.
  // But because it does not have ownership of what it refers to, it is not dropped.

fn change(_some_string: &String) {
    // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // some_string.push_str(", world");
}

fn change_plus_mut(_some_string: &mut String) {
    _some_string.push_str(", world");
    println!("str = {}", _some_string);
    println!("--------------------------------------------------------------")
}