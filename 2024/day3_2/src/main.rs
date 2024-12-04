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
        let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))").unwrap();
        let mut enable = true;
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            for caps in re.captures_iter(line) {
                println!("capture {:?}", caps);
                let mut it = caps.iter();
                let full = it.next().unwrap().map(|m| m.as_str()).unwrap();
                if full.starts_with("mul") {
                    if !enable {
                        println!("disabled! skipping mul");
                        continue;
                    }
                    it.next().unwrap();
                    let a = it.next().unwrap().map(|m| m.as_str()).unwrap();
                    let b = it.next().unwrap().map(|m| m.as_str()).unwrap();
                    ret.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
                } else if full.starts_with("don") {
                    enable = false;
                } else if full.starts_with("do") {
                    enable = true;
                } else {
                    panic!("bad match {}", full);
                }
            }
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
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
".lines().map(String::from).collect();

        let c = Computer::new(&sample);
        assert_eq!(c.instructions.len(), 2);
        assert_eq!(c.mul(), 48);
    }
}
