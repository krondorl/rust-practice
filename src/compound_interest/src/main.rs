#[allow(dead_code)]
fn calculate_interest(capital: u32, annual_rate: f32, years: u32) -> Result<f32, String> {
    if capital < 1 || annual_rate < 1.0 || years < 1 {
        return Err(String::from(
            "Error: each parameters should be at least 1 or greater.",
        ));
    }
    let mut interest = capital as f32;
    for _y in 0..years {
        interest *= 1.0 + (annual_rate / 100.0);
    }
    Ok(interest)
}

#[allow(dead_code)]
fn calculate_yearly_interest(
    capital: u32,
    annual_rate: f32,
    years: u32,
) -> Result<Vec<f32>, String> {
    if capital < 1 || annual_rate < 1.0 || years < 1 {
        return Err(String::from(
            "Error: each parameters should be at least 1 or greater.",
        ));
    }
    let mut yearly_interests = Vec::new();
    let mut interest = capital as f32;
    for _y in 0..years {
        interest *= 1.0 + (annual_rate / 100.0);
        yearly_interests.push(interest);
    }
    Ok(yearly_interests)
}

fn print_interests(capital: u32, annual_rate: f32, years: u32) {
    const CURRENCY: &str = "$";
    let yearly_interests = calculate_yearly_interest(capital, annual_rate, years);
    match yearly_interests {
        Ok(val) => {
            println!("Let's calculate the following");
            println!("{CURRENCY}{capital} capital with {annual_rate}% over {years} years");
            for yearly_interest in val {
                println!("{CURRENCY}{yearly_interest:.2}");
            }
        }
        Err(e) => println!("{e}"),
    }
}

fn main() {
    println!("Compound interest calculator");
    println!("***");
    print_interests(1000, 5.0, 5);
}
