use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = u64;

fn evolve(mut secret: Num) -> Num {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

#[derive(Debug, Clone)]
struct Buyers {
    data: Vec<Num>,
}

impl Buyers {
    fn new(lines: &Vec<String>) -> Buyers {
        Buyers{
            data: lines.iter().map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| x.parse::<Num>().unwrap()).collect::<Vec<_>>(),
        }
    }

    fn evolve(&mut self, n: Num) {
        for d in self.data.iter_mut() {
            for _ in 0..n {
                *d = evolve(*d);
            }
        }
    }

    fn sum(&self) -> Num {
        self.data.iter().sum()
    }
}

fn main() {
    let lines = read_lines("input");
    let mut c = Buyers::new(&lines);
    c.evolve(2000);
    println!("sum: {}", c.sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut secret = 123;
        secret = evolve(secret);
        assert_eq!(secret, 15887950);
        
        secret = evolve(secret);
        assert_eq!(secret, 16495136);
        
        secret = evolve(secret);
        assert_eq!(secret, 527345);
        
        secret = evolve(secret);
        assert_eq!(secret, 704524);
        
        secret = evolve(secret);
        assert_eq!(secret, 1553684);
        
        secret = evolve(secret);
        assert_eq!(secret, 12683156);
        
        secret = evolve(secret);
        assert_eq!(secret, 11100544);
        
        secret = evolve(secret);
        assert_eq!(secret, 12249484);
        
        secret = evolve(secret);
        assert_eq!(secret, 7753432);
        
        secret = evolve(secret);
        assert_eq!(secret, 5908254);
    }




    #[test]
    fn test_10() {
        let sample: Vec<String> = "
1
10
100
2024
".lines().map(String::from).collect();

        let mut c = Buyers::new(&sample);
        c.evolve(2000);
        assert_eq!(c.sum(), 37327623);
    }
}