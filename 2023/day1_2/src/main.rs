use std::fs::read_to_string;
use lazy_static::lazy_static;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

lazy_static! {
    static ref NUMS: HashMap<&'static str, char> = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ].into_iter().collect();
}

fn calc(lines: &Vec<String>) -> i32 {
    let mut ret = 0;
    for line in lines {
        let num_length = line.len();

        let mut first: char = '\0';
        for i in 0 .. num_length {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                first = c;
                break;
            }
            for (key, val) in NUMS.iter() {
                if line[i..].starts_with(key) {
                    first = *val;
                    break;
                }
            }
            if first != '\0' {
                break
            }
        }

        let mut last: char = '\0';
        for i in (0 .. num_length).rev() {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                last = c;
                break;
            }
            for (key, val) in NUMS.iter() {
                if line[i..].starts_with(key) {
                    last = *val;
                    break;
                }
            }
            if last != '\0' {
                break
            }
        }

        println!("{} -> {}{}", line, first, last);
        assert!(first != '\0');
        assert!(last != '\0');
        let combined = vec![first, last];
        ret += String::from_iter(combined).parse::<i32>().unwrap();
    }
    ret
}


fn main() {
    let ret = calc(&read_lines("input"));
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
".lines().map(String::from).collect();

        let ret = calc(&sample);
        assert_eq!(ret, 281);
    }
}