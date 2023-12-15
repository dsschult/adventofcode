use std::fs::read_to_string;
use std::cmp::Ordering;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug, Clone, Eq, PartialEq)]
struct Platform {
    cols: Vec<Vec<char>>
}

impl Platform {
    fn new(lines: &Vec<String>) -> Platform {
        let mut cols = Vec::new();
        let col_len = lines.len();
        for _ in lines[0].chars() {
            cols.push(vec!['.'; col_len]);
        }
        for (i,line) in lines.iter().enumerate() {
            for (j,c) in line.chars().enumerate() {
                cols[j][i] = c;
            }
        }
        Platform{cols: cols}
    }

    fn tilt_north(&mut self) -> () {
        // slide every O as far "down" as it will go
        for col in self.cols.iter_mut() {
            for part in col.split_mut(|x| *x == '#') {
                if part.is_empty() {
                    continue;
                }
                part.sort_by(|a,b| match (*a,*b) { ('O','.') => Ordering::Less, ('.','O') => Ordering::Greater, _ => Ordering::Equal });
            }
        }
    }

    fn tilt_south(&mut self) -> () {
        // slide every O as far "up" as it will go
        for col in self.cols.iter_mut() {
            for part in col.split_mut(|x| *x == '#') {
                if part.is_empty() {
                    continue;
                }
                part.sort_by(|a,b| match (*a,*b) { ('O','.') => Ordering::Greater, ('.','O') => Ordering::Less, _ => Ordering::Equal });
            }
        }
    }

    fn tilt_west(&mut self) -> () {
        // slide every O as far "right" as it will go
        let col_len = self.cols.len()-1;
        for i in 0..self.cols[0].len() {
            // bubble sort col
            loop {
                let mut change = false;
                for j in 0..col_len {
                    if self.cols[j][i] == '.' && self.cols[j+1][i] == 'O' {
                        self.cols[j][i] = 'O';
                        self.cols[j+1][i] = '.';
                        change = true;
                    }
                }
                if !change {
                    break;
                }
            }
        }
    }
    
    fn tilt_east(&mut self) -> () {
        // slide every O as far "right" as it will go
        let col_len = self.cols.len()-1;
        for i in 0..self.cols[0].len() {
            // bubble sort col
            loop {
                let mut change = false;
                for j in 0..col_len {
                    if self.cols[j][i] == 'O' && self.cols[j+1][i] == '.' {
                        self.cols[j][i] = '.';
                        self.cols[j+1][i] = 'O';
                        change = true;
                    }
                }
                if !change {
                    break;
                }
            }
        }
    }

    fn spin(&mut self) -> () {
        // do 4 tilts on each axis
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn print(&self) -> () {
        for i in 0..self.cols[0].len() {
            for j in 0..self.cols.len() {
                print!("{}", self.cols[j][i]);
            }
            println!("");
        }
    }

    fn load(&self) -> usize {
        let mut ret = 0;
        let col_len = self.cols.len();
        for col in self.cols.iter() {
            for (i,c) in col.iter().enumerate() {
                if *c == 'O' {
                    ret += col_len-i;
                }
            }
        }
        ret
    }
}

fn find_repeats(slice: &[usize]) -> Option<usize> {
    for i in 1..slice.len()/10 {
        let mut iter = slice.chunks(i);
        let c1 = iter.next().unwrap();
        let c2 = iter.next().unwrap();
        let c3 = iter.next().unwrap();
        let c4 = iter.next().unwrap();
        if c1 == c2 && c2 == c3 && c3 == c4 {
            return Some(i);
        }
    }
    None
}


fn main() {
    let lines = read_lines("input");
    
    let mut platform = Platform::new(&lines);
    let mut loads = Vec::new();
    let spin_num = 1000000000;
    for i in 0..spin_num {
        platform.spin();
        loads.push(platform.load());
        if i >= 1000 {
            match find_repeats(&loads[loads.len()-1000..]) {
                Some(x) => {
                    println!("found repeat of len {}", x);
                    let spins_remaining = spin_num-i-1;
                    for _ in 0..spins_remaining%x {
                        platform.spin();
                    }
                    break;
                },
                None => { }
            }
        }
    }
    println!("load: {}", platform.load());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let sample: Vec<String> = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
".lines().map(String::from).collect();

        let mut platform = Platform::new(&sample);
        platform.tilt_north();
        platform.print();
        assert_eq!(platform.load(), 136);
    }

    #[test]
    fn test_spin() {
        let sample: Vec<String> = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
".lines().map(String::from).collect();

        let mut platform = Platform::new(&sample);
        let mut loads = Vec::new();
        let spin_num = 1000000000;
        for i in 0..spin_num {
            platform.spin();
            loads.push(platform.load());
            if i >= 1000 {
                match find_repeats(&loads[loads.len()-1000..]) {
                    Some(x) => {
                        println!("found repeat of len {}", x);
                        let spins_remaining = spin_num-i-1;
                        for _ in 0..spins_remaining%x {
                            platform.spin();
                        }
                        break;
                    },
                    None => { }
                }
            }
        }
        println!("loads: {:?}", loads);
        assert_eq!(platform.load(), 64);
    }
}
