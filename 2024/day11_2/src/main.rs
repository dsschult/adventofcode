use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = u64;

#[derive(Debug, Clone)]
struct Stones {
    map: Vec<Num>,
}

impl Stones {
    fn new(lines: &Vec<String>) -> Stones {
        let mut ret = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            ret.extend(trim_line.split_whitespace().map(|x| x.parse::<Num>().unwrap()))
        }
        Stones{ map: ret }
    }

    fn blink(&mut self) {
        let mut new_map = Vec::new();
        for stone in self.map.iter() {
            let mut str_stone = stone.to_string();
            match stone {
                0 => {
                    new_map.push(1);
                },
                _ if str_stone.len() % 2 == 0 => {
                    let s2 = str_stone.split_off(str_stone.len()/2);
                    new_map.push(str_stone.parse::<Num>().unwrap());
                    new_map.push(s2.parse::<Num>().unwrap());
                },
                s => {
                    new_map.push(s * 2024);
                }
            }
        }
        self.map = new_map;
    }

    fn blinks(&mut self, n: Num) -> Num {
        let mut ret = 0;
        let mut queue = self.map.iter().map(|x| (0, *x)).collect::<Vec<_>>();
        while !queue.is_empty() {
            let (blinks,stone) = queue.pop().unwrap();
            if blinks == n {
                ret += 1;
            } else {
                let new_blinks = blinks+1;
                let mut str_stone = stone.to_string();
                match stone {
                    0 => {
                        queue.push((new_blinks, 1));
                    },
                    _ if str_stone.len() % 2 == 0 => {
                        let s2 = str_stone.split_off(str_stone.len()/2);
                        queue.push((new_blinks, str_stone.parse::<Num>().unwrap()));
                        queue.push((new_blinks, s2.parse::<Num>().unwrap()));
                    },
                    s => {
                        queue.push((new_blinks, s * 2024));
                    }
                };
            }
        }
        ret
    }

    fn blinks2(&mut self, n: Num) -> Num {
        if n % 2 == 1 {
            self.blink();
        }
        let mut ret = 0;
        let mut answers: HashMap<Num, Vec<Num>> = HashMap::new();
        let mut answers2: HashMap<Num, Num> = HashMap::new();
        let mut mid = Vec::new();
        for num in self.map.iter() {
            match answers.get(num) {
                Some(a) => {
                    mid.extend(a.clone());
                },
                None => {
                    let mut st = Stones{ map: vec![*num] };
                    for _ in 0..(n/2) {
                        st.blink();
                    }
                    mid.extend(st.map.clone());
                    answers.insert(*num, st.map);
                }
            }
        }
        println!("at mid {}", mid.len());
        for num in mid {
            match answers.get(&num) {
                Some(a) => { ret += a.len() as Num; },
                None => {
                    match answers2.get(&num) {
                        Some(a) => { ret += a; },
                        None => {
                            let r = Stones{ map: vec![num] }.blinks(n/2);
                            answers2.insert(num, r);
                            ret += r;
                        }
                    };
                }
            }
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let mut stones = Stones::new(&lines);
    /*for i in 0..75 {
        println!("blink {}", i);
        stones.blink();
    }
    let ret = stones.map.len();*/
    let ret = stones.blinks2(75);
    //println!("stones: {:?}", stones.map);
    println!("stones len: {}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
0 1 10 99 999
".lines().map(String::from).collect();

        let mut stones = Stones::new(&sample);
        assert_eq!(stones.map, vec![0, 1, 10, 99, 999]);

        assert_eq!(stones.blinks(1), 7);
        //assert_eq!(stones.map, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_2() {
        let sample: Vec<String> = "
125 17
".lines().map(String::from).collect();

        let mut stones = Stones::new(&sample);
        assert_eq!(stones.blinks(25), 55312);
    }

    #[test]
    fn test_3() {
        let sample: Vec<String> = "
125 17
".lines().map(String::from).collect();

        let mut stones = Stones::new(&sample);
        assert_eq!(stones.blinks2(25), 55312);
    }
}