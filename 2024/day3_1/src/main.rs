use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone)]
struct Computer {
    instructions: Vec<(i32,i32)>
}

impl Computer {
    fn new(lines: &Vec<String>) -> Computer {
        let mut ret = Vec::new();
        let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))").unwrap();
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            let muls = re.captures_iter(line).map(|caps| {
                println!("capture {:?}", caps);
                let (_, [_, a, b]) = caps.extract();
                (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
            });
            ret.extend(muls);
        }
        Computer{instructions: ret}
    }

    fn mul(&self) -> i32 {
        let mut ret = 0;
        for (a,b) in self.instructions.iter() {
            ret += a*b;
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Computer::new(&lines);
    println!("Num instrs: {}", c.instructions.len());
    println!("Mul: {}", c.mul());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        let sample: Vec<String> = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
".lines().map(String::from).collect();

        let c = Computer::new(&sample);
        assert_eq!(c.instructions.len(), 4);
        assert_eq!(c.mul(), 161);
    }
}
