// Problem 1
// Multiples of 3 and 5
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn multiples(n: u32) -> Result<u32, String> {
    if n < 3 {
        return Err(String::from("Error: parameter should be at least 3."));
    }
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    Ok(sum)
}

fn main() {
    println!("Multiples of 3 and 5");
    let sum_multiples = multiples(1000);
    match sum_multiples {
        Ok(val) => println!("The sum is {val}"),
        Err(e) => println!("{e}"),
    }
}
