use std::fs::read_to_string;
use std::collections::HashMap;
use std::iter::zip;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone)]
struct Lists {
    a: Vec<i32>,
    b: Vec<i32>
}

impl Lists {
    fn new(lines: &Vec<String>) -> Lists {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            let mut ids = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).take(2);
            a.push(ids.next().unwrap());
            b.push(ids.next().unwrap());
        }
        a.sort();
        b.sort();
        Lists{ a: a, b:b }
    }

    fn diff(&self) -> i32 {
        let mut ret = 0;
        for (a,b) in zip(self.a.iter(), self.b.iter()) {
            ret += (a-b).abs()
        }
        ret
    }

    fn similarity(&self) -> i32 {
        let mut ret = 0;
        let mut b_counts = HashMap::new();
        for b in self.b.iter() {
            let x = b_counts.entry(b).or_insert(0);
            *x += 1;
        }
        for a in self.a.iter() {
            match b_counts.get(a) {
                None => {
                    println!("sim for {} is {}", a, 0);
                },
                Some(x) => {
                    println!("sim for {} is {}", a, a*x);
                    ret += a*x
                },
            }
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let ids = Lists::new(&lines);
    println!("Total Distance: {}", ids.diff());
    println!("Similarity: {}", ids.similarity());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        let sample: Vec<String> = "
3   4
4   3
2   5
1   3
3   9
3   3
".lines().map(String::from).collect();

        let ids = Lists::new(&sample);
        assert_eq!(ids.diff(), 11);
    }

    #[test]
    fn test_8() {
        let sample: Vec<String> = "
3   4
4   3
2   5
1   3
3   9
3   3
".lines().map(String::from).collect();

        let ids = Lists::new(&sample);
        assert_eq!(ids.similarity(), 31);
    }
}