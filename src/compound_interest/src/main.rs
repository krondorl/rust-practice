#[allow(dead_code)]
fn calculate_interest(capital: u32, annual_rate: f32, years: u32) -> f32 {
    let mut interest = capital as f32;
    for _y in 0..years {
        interest *= 1.0 + (annual_rate / 100.0);
    }
    interest
}

#[allow(dead_code)]
fn calculate_yearly_interest(capital: u32, annual_rate: f32, years: u32) -> Vec<f32> {
    let mut yearly_interests = Vec::new();
    let mut interest = capital as f32;
    for _y in 0..years {
        interest *= 1.0 + (annual_rate / 100.0);
        yearly_interests.push(interest);
    }
    yearly_interests
}

fn print_interests(capital: u32, annual_rate: f32, years: u32) {
    const CURRENCY: &str = "$";
    let yearly_interests = calculate_yearly_interest(capital, annual_rate, years);
    println!("Let's calculate the following");
    println!("{}{} capital with {}% over {} years", CURRENCY, capital, annual_rate, years);
    for yearly_interest in &yearly_interests {
        println!("{}{:.2}", CURRENCY, yearly_interest);
    }
}

fn main() {
    println!("Compound interest calculator");
    println!("***");
    print_interests(1000, 5.0, 5);
}
