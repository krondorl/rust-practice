fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    let mut sieve_numbers: Vec<bool> = Vec::new();
    if n < 2 {
        return Vec::new();
    }
    for _array_index in 0..=n {
        sieve_numbers.push(true);
    }
    let square_root_of_n = (n as f32).sqrt() as u32;
    for array_index in 2..=square_root_of_n as usize {
        if sieve_numbers[array_index] {
            let mut mults_index = array_index * array_index;
            while mults_index <= n as usize {
                sieve_numbers[mults_index] = false;
                mults_index += array_index;
            }
        }
    }
    let mut primes: Vec<u32> = Vec::new();
    for array_index in 2..=n as usize {
        if sieve_numbers[array_index] {
            primes.push(array_index as u32);
        }
    }
    primes
}

// WIP
// fn sieve_of_atkin(limit: u32) -> Vec<u32> {
//     let mut sieve = vec![false; limit as usize];
//     if limit > 2 {
//         sieve[2] = true;
//     }
//     if limit > 3 {
//         sieve[3] = true;
//     }
//     let mut x = 1;
//     while x * x <= limit {
//         let mut y = 1;
//         while y * y <= limit {
//             let mut n = (4 * x * x) + (y * y);
//             ... todo
//             y += 1;
//         }
//         x += 1;
//     }
// }

fn main() {
    println!("Prime Sieves");
    let primes = sieve_of_eratosthenes(100);
    let p = format!("{:?}", &primes);
    println!("{}", p);
}
