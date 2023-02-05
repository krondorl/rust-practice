// Problem 1
// Multiples of 3 and 5
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn multiples(n: u32) -> u32 {
    if n < 1 {
        return 0;
    }
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("Multiples of 3 and 5");
    let sum_multiples = multiples(1000);
    println!("The sum is {}", sum_multiples);
}
