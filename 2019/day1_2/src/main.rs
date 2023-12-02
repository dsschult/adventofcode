use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn calc_fuel(mass: u32) -> u32 {
    let fuel_mass = mass / 3;
    if fuel_mass < 2 {
        0
    } else {
        fuel_mass - 2
    }
}

fn calc_extra_fuel(mass: u32) -> u32 {
    let mut extra = calc_fuel(mass);
    let mut extra_sum = extra;
    while extra > 0 {
        extra = calc_fuel(extra);
        extra_sum += extra;
    }
    extra_sum
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass = line?.parse::<u32>().unwrap();
        total_fuel += calc_extra_fuel(mass);
    }
    println!("total fuel: {}", total_fuel);
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn test_calc_extra_fuel() {
        assert_eq!(calc_extra_fuel(14), 2);
        assert_eq!(calc_extra_fuel(1969), 966);
        assert_eq!(calc_extra_fuel(100756), 50346);
    }
}