fn main() {
    loop {
        println!("Hello, world!");
        break;
    }
    println!("------------------------------------------------------");

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result = {result}");
    println!("------------------------------------------------------");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_label: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("outer loop----------------------");
            println!("Remaining = {remaining}");
            if remaining == 9 {
                println!("inner loop - remaining == 9");
                break;
            }
            if count == 2 {
                println!("inner loop - count == 2");
                break 'counting_label;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("------------------------------------------------------");

    // while
    let mut n = 3;

    while n != 0 {
        println!("{n} !!");

        n -= 1;
    }
    println!("LIFTOFF~~!!!");
    println!("------------------------------------------------------");

    // Looping Through a Collection with for
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the arr[{}] val = {}", index, arr[index]);
        index += 1;
    }

    println!("------------------------------------------------------");

    // more concise alternative, for each
    let a = [100, 200, 300, 400, 500];
    for elment in a {
        println!("the value = {}", elment);
    }

    println!("------------------------------------------------------");
    // for loop
    for number in (1..4).rev() {
        print!("{number}, ");
    }
    println!("LIFTOFF~~~~~!!!!!");
}
