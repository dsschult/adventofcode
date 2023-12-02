use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn calc(lines: &Vec<String>) -> i32 {
    let mut ret = 0;
    for line in lines {
        let mut first: char = '\0';
        let mut last: char = '\0';
        for c in line.chars() {
            if c.is_numeric() {
                if first == '\0' {
                    first = c;
                }
                last = c;
            }
        }
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
        let sample: Vec<String> = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".lines().map(String::from).collect();

        let ret = calc(&sample);
        assert_eq!(ret, 142);
    }
}