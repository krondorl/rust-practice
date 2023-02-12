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

fn sieve_of_atkin(limit: u32) -> Vec<u32> {
    let mut sieve = vec![false; limit as usize];
    if limit > 2 {
        sieve[2] = true;
    }
    if limit > 3 {
        sieve[3] = true;
    }
    let mut x = 1;
    while x * x <= limit {
        let mut y = 1;
        while y * y <= limit {
            let mut n = (4 * x * x) + (y * y);
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize] = !sieve[n as usize];
            }
            n = (3 * x * x) + (y * y);
            if n <= limit && (n % 12 == 7) {
                sieve[n as usize] = !sieve[n as usize];
            }
            n = (3 * x * x).wrapping_sub(y * y);
            if x > y && n <= limit && (n % 12 == 11) {
                sieve[n as usize] = !sieve[n as usize];
            }
            y += 1;
        }
        x += 1;
    }
    let mut r = 5;
    while r * r < limit {
        if sieve[r as usize] {
            let mut i = r * r;
            while i < limit {
                sieve[i as usize] = false;
                i += r * r;
            }
        }
        r += 1;
    }
    let mut primes: Vec<u32> = Vec::new();
    for prime_index in 0..limit as usize {
        if sieve[prime_index] {
            primes.push(prime_index as u32);
        }
    }
    return primes;
}

fn print_sieve(n: u32, sieve: &str) {
    println!("***");
    println!("Sieve of {}", sieve);
    let mut primes= vec![0];
    if sieve == "Eratosthenes" {
        primes = sieve_of_atkin(n);
    } else if sieve == "Atkin" {
        primes = sieve_of_eratosthenes(n);
    }
    let p = format!("{:?}", &primes);
    println!("{}", p);
}

fn main() {
    println!("Prime Sieves");
    print_sieve(100, "Eratosthenes");
    print_sieve(100, "Atkin");
}
