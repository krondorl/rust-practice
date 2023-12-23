fn factorial(n: u64) -> u64 {
    let mut factorial_result = 1;
    if n < 2 {
        return 1;
    }
    for index in 2..n + 1 {
        factorial_result *= index;
    }
    factorial_result
}

fn main() {
    println!("Factorial program");
    let n = 5;
    let factorial_value = factorial(n);
    println!("Factorial of {n} is {factorial_value}");
}
