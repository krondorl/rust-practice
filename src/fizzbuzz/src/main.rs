fn fizz_buzz_for(n: u32) {
    for i in 1..=n {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn fizz_buzz_while(n: u32) {
    let mut i = 1;
    while i <= n {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}

fn fizz_buzz_loop(n: u32) {
    let mut i = 1;
    loop {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }

        if i == n {
            break;
        }
        i += 1;
    }
}

fn fizz_buzz_match(n: u32) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i)
        }
    }
}


fn main() {
    println!("FizzBuzz program in multiple ways");
    println!("");

    fizz_buzz_for(30);
    println!("");
    println!("***");
    println!("");

    fizz_buzz_while(30);
    println!("");
    println!("***");
    println!("");

    fizz_buzz_loop(30);
    println!("");
    println!("***");
    println!("");

    fizz_buzz_match(30);
    println!("");
}
