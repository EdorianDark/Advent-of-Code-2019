fn required_fuel(mass: i64) -> i64 {
    mass.div_euclid(3) - 2
}

#[test]
fn test_required_fuel() {
    assert_eq!(required_fuel(12), 2);
    assert_eq!(required_fuel(14), 2);
    assert_eq!(required_fuel(1969), 654);
    assert_eq!(required_fuel(100756), 33583);
}

pub fn fuel_requirements() -> i64 {
    let input = include_str!("../static/day1_modules.txt");
    input
        .lines()
        .flat_map(str::parse::<i64>)
        .map(required_fuel)
        .sum()
}

fn fuel_for_fuel(mass: i64) -> i64 {
    let fuel = required_fuel(mass);
    if fuel > 0 {
        return fuel + fuel_for_fuel(fuel);
    }
    0
}

#[test]
fn test_fuel_for_fuel() {
    assert_eq!(fuel_for_fuel(14), 2);
    assert_eq!(fuel_for_fuel(1969), 966);
    assert_eq!(fuel_for_fuel(100756), 50346);
}

pub fn correct_fuel_requirements() -> i64 {
    let input = include_str!("../static/day1_modules.txt");
    input
        .lines()
        .flat_map(str::parse::<i64>)
        .map(fuel_for_fuel)
        .sum()
}
