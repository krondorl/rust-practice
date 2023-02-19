use rand::prelude::*;

// General function which can draw multiple game types
// These draws are popular in Hungary
// range # count
// 1..90 # 5
// 1..45 # 6
// 1..35 # 7
fn generate_draw(draw_type: &str) -> Vec<i32> {
    let mut drawn_numbers: Vec<i32> = Vec::new();
    let max_number: i32;
    let max_count: i32;

    match draw_type {
        "five-number draw" => {
            max_number = 90;
            max_count = 5;
        }
        "six-number draw" => {
            max_number = 45;
            max_count = 6;
        }
        "seven-number draw" => {
            max_number = 35;
            max_count = 7;
        }
        _ => {
            max_number = 90;
            max_count = 5;
        }
    }

    let mut rng: ThreadRng = thread_rng();
    loop {
        let random_number: i32 = rng.gen_range(1..max_number);

        if !drawn_numbers.contains(&random_number) {
            drawn_numbers.push(random_number);
        }
        if drawn_numbers.len() == max_count as usize {
            break;
        }
    }
    drawn_numbers.sort();
    drawn_numbers
}

fn print_draw(draw_type: &str) {
    let draw = generate_draw(draw_type);
    let d = format!("{:?}", &draw);
    println!("{}", d);
}

fn main() {
    println!("Lottery draw");
    print_draw("five-number draw");
    print_draw("six-number draw");
    print_draw("seven-number draw");
}
