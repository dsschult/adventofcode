use std::fs::read_to_string;
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
    pairs: Vec<(i32, i32)>
}

impl Lists {
    fn new(lines: &Vec<String>) -> Lists {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            println!("line: {}", line);
            let mut ids = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).take(2);
            a.push(ids.next().unwrap());
            b.push(ids.next().unwrap());
        }
        a.sort();
        b.sort();
        Lists{ pairs: zip(a,b).collect::<Vec<(i32,i32)>>() }
    }

    fn diff(&self) -> i32 {
        let mut ret = 0;
        for (a,b) in self.pairs.iter() {
            ret += (a-b).abs()
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let ids = Lists::new(&lines);
    println!("Total Distance: {}", ids.diff());
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
}