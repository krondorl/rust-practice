fn fibonacci(n: u32) -> Vec<u32> {
    let mut fib_numbers = vec![1, 1];
    let mut prev_prev_index = 0;
    if n < 3 {
        return fib_numbers;
    }
    for _i in 3..n {
        fib_numbers.push(fib_numbers[prev_prev_index] + fib_numbers[prev_prev_index + 1]);
        prev_prev_index += 1;
    }
    fib_numbers
}

fn main() {
    println!("Fibonacci sequence in Rust");
    let fib_numbers = fibonacci(10);
    let s = format!("{:?}", &fib_numbers);
    println!("{}", s);
}
