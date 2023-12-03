use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug)]
struct Elf {
    index: u32,  // 1-index
    calories: Vec<u32>
}

impl Elf {
    fn new(index: u32, lines: &[String]) -> Self {
        Elf {
            index: index+1,
            calories: lines.iter().map(|x| x.parse::<u32>().unwrap()).collect()
        }
    }

    fn sum(&self) -> u32 {
        let mut ret = 0;
        for c in self.calories.iter() {
            ret += c;
        }
        return ret
    }
}

#[derive(Debug)]
struct Party {
    members: Vec<Elf>
}

impl Party {
    fn new(lines: &Vec<String>) -> Self {
        let mut ret = Party{ members: Vec::new() };
        let mut start = 0;
        for (i,line) in lines.iter().enumerate() {
            if line == "" {
                ret.members.push(Elf::new(ret.members.len() as u32, &lines[start..i]));
                start = i+1;
            }
        }
        if start < lines.len() {
            ret.members.push(Elf::new(ret.members.len() as u32, &lines[start..]));
        }
        ret
    }

    fn most(&self) -> &Elf {
        let mut ret = None;
        let mut cur = 0;
        for elf in self.members.iter() {
            let s = elf.sum();
            if s > cur {
                cur = s;
                ret = Some(elf);
            }
        }
        match ret {
            Some(x) => x,
            None => {
                println!("cur: {}", cur);
                panic!("No most elf!")
            }
        }
    }
}

fn calc(lines: &Vec<String>) -> Party {
    Party::new(lines)
}

fn main() {
    let ret = calc(&read_lines("input"));
    let most = ret.most();
    println!("Most: {:?} = {}", most, most.sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
".lines().map(String::from).collect();

        let elves = calc(&sample);
        let most = elves.most();
        println!("Party: {:?}", elves);
        println!("Most: {:?}", most);
        assert!(most.index == 4);
        assert!(most.sum() == 24000);
    }
}