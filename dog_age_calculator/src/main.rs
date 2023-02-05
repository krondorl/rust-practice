fn calculate_dog_age(age: u8) -> u8 {
    let result_age: f32 = (16. * (age as f32).ln() + 31.).round();
    result_age as u8
}

fn main() {
    println!("Dog Age Calculator");
    let age = 1;
    let dog_age = calculate_dog_age(age);
    println!("Dog age {} in human years is {}", age, dog_age);
}
