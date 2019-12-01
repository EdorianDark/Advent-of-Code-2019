mod day1;

fn main() {
    println!("Hello, world!");
    println!("fuel required {}", day1::calculate_fuel_requirements());
    println!("correct fuel {}", day1::calculate_correct_fuel_requirements());
}
