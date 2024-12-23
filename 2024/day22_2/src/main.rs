use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = i64;
type Price = i8;

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
struct Histories {
    data: Vec<Vec<Price>>
}

impl Histories {
    fn find_best_delta(&self) -> Num {
        let four_seqs = self.data.iter().map(|x| {
            let mut ret = HashMap::new();
            for i in 4..x.len() {
                let tup = (
                    x[i-3] - x[i-4],
                    x[i-2] - x[i-3],
                    x[i-1] - x[i-2],
                    x[i] - x[i-1],
                );
                match ret.get(&tup) {
                    None => { ret.insert(tup, x[i]); },
                    _ => { }
                };
            }
            ret
        }).collect::<Vec<_>>();
        let mut best_seq = HashMap::new();
        for seqs in four_seqs.iter() {
            for (seq, price) in seqs.iter() {
                *best_seq.entry(seq).or_insert(0 as Num) += *price as Num;
            }
        }
        let best = best_seq.iter().fold((0, None), |(max, max_seq), (seq, price)| {
            if *price > max {
                (*price, Some(seq))
            } else {
                (max, max_seq)
            }
        });
        let mut b = 0 as Num;
        for seqs in four_seqs.iter() {
            match seqs.get(best.1.unwrap()) {
                None => { },
                Some(v) => {
                    println!("  sells for {}", v);
                    b += *v as Num;
                }
            };
        }
        println!("best: {:?}", best);
        println!("b = {}, best.0 = {}", b, best.0);
        best.0
        //*best_seq.values().max().unwrap()
    }
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

    fn evolve(&mut self, n: Num) -> Histories {
        let mut ret = Vec::new();
        for d in self.data.iter_mut() {
            let mut ret2 = Vec::new();
            for _ in 0..n {
                ret2.push((*d % 10) as Price);
                *d = evolve(*d);
            }
            ret2.push((*d % 10) as Price);
            ret.push(ret2);
        }
        Histories{ data: ret }
    }

    fn sum(&self) -> Num {
        self.data.iter().sum()
    }
}

fn main() {
    let lines = read_lines("input");
    let mut c = Buyers::new(&lines);
    let hist = c.evolve(2000);
    println!("most: {}", hist.find_best_delta());
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
    fn test_2() {
        let hist = Histories{ data: vec![vec![3,0,6,5,4,4,6,4,4,2]] };
        assert_eq!(hist.find_best_delta(), 6);
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

    #[test]
    fn test_11() {
        let sample: Vec<String> = "
1
2
3
2024
".lines().map(String::from).collect();

        let mut c = Buyers::new(&sample);
        let hist = c.evolve(2000);
        assert_eq!(hist.find_best_delta(), 23);
    }

    #[test]
    fn test_12() {
        let sample: Vec<String> = "
2021
5017
19751
".lines().map(String::from).collect();

        let mut c = Buyers::new(&sample);
        let hist = c.evolve(2000);
        assert_eq!(c.sum(), 18183557);
        assert_eq!(hist.find_best_delta(), 27);
    }

    #[test]
    fn test_13() {
        let sample: Vec<String> = "
5053 
10083 
11263 
".lines().map(String::from).collect();

        let mut c = Buyers::new(&sample);
        let hist = c.evolve(2000);
        assert_eq!(c.sum(), 8876699);
        assert_eq!(hist.find_best_delta(), 27);
    }
}