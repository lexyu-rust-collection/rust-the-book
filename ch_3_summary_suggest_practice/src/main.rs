fn main() {
    for n in 2..16  {
        println!("fibonacci({}) => value = {}", n, fibonacci(n));
    }
}

fn fibonacci(x: u128) -> u128 {
    if x <= 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    return fibonacci(x-1) + fibonacci(x-2);
}