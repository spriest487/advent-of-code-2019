use std::str::FromStr;

fn fuel_required(mass: i64) -> i64 {
    (mass as f64 / 3.0).floor() as i64 - 2
}

fn fuel_required_full(mass: i64) -> i64 {
    let mut fuel = fuel_required(mass);
    let mut total = fuel;

    loop {
        match fuel_required(fuel) {
            some if some > 0 => {
                total += some;
                fuel = some;
            }

            _ => break total,
        }
    }
}

fn main() {
    let modules: Vec<_> = include_str!("day1.txt")
        .lines()
        .map(|line| i64::from_str(line).unwrap())
        .collect();

    let fuel_required: i64 = modules.iter().map(|mass| fuel_required(*mass)).sum();
    println!("fuel required: {}", fuel_required);

    let fuel_required_full: i64 = modules.iter().map(|mass| fuel_required_full(*mass)).sum();
    println!("fuel required (including fuel required for fuel): {}", fuel_required_full);
}
