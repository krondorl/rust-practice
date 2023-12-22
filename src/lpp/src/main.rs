fn is_palindrome(x: i32) -> bool {
    let mut rev = 0;
    let mut num = x;
    if x < 0 {
        return false;
    }
    while num > 0 {
        rev = rev * 10 + num % 10;
        num /= 10;
    }
    rev == x
}

fn largest_palindrome_product() -> i32 {
    let mut palindromes = Vec::new();
    for x in 100..999 {
        for y in 100..999 {
            let prod = x * y;
            if is_palindrome(prod) {
                palindromes.push(prod);
            }
        }
    }
    palindromes.sort();
    return palindromes[palindromes.len() - 1];
}


fn main() {
    println!("Largest Palindrome Product");
    let lpp = largest_palindrome_product();
    println!("{}", lpp);
}
