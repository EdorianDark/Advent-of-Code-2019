mod day1;
mod day2;

fn main() {
    println!("Hello, world!");
    println!("day 1: fuel required {}", day1::fuel_requirements());
    println!("day 1: correct fuel {}", day1::correct_fuel_requirements());
    day2::puzzle();
    day2::try_all();
}
