use std::fs::read_to_string;

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
}

fn main() {
    let lines = read_lines("input");
    let mut stones = Stones::new(&lines);
    for _ in 0..25 {
        stones.blink();
    }
    println!("stones: {:?}", stones.map);
    println!("stones len: {}", stones.map.len());
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

        stones.blink();
        assert_eq!(stones.map, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }
}