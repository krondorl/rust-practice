// Original algorithm from
//
// chandramauliguptach
// Calculate Pi using Nilkantha’s series
// https://www.geeksforgeeks.org/calculate-pi-using-nilkanthas-series/
// Last access: December 14th, 2023.
//
// Ported to Rust by Adam Burucs in 2023.
//
fn calculate_pi(mut pi: f32, mut n: f32, mut sign: f32) -> f32 {
    for _i in 0..1000000 {
        pi += sign * (4.0 / ((n) * (n + 1.0) * (n + 2.0)));
        sign *= -1.0;
        n += 2.0;
    }
    pi
}

fn main() {
    println!("{}", calculate_pi(3.0, 2.0, 1.0));
}
