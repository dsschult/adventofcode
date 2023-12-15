use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug, Clone)]
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

    fn tilt(&mut self) -> () {
        // slide every O as far "down" as it will go
        for col in self.cols.iter_mut() {
            // bubble sort col
            loop {
                let mut change = false;
                for j in 0..col.len()-1 {
                    if col[j] == '.' && col[j+1] == 'O' {
                        col.swap(j, j+1);
                        change = true;
                    }
                }
                if !change {
                    break;
                }
            }
        }
    }

    fn print(&self) -> () {
        for i in 0..self.cols[0].len() {
            for j in 0..self.cols.len() {
                print!("{}", self.cols[i][j]);
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


fn main() {
    let lines = read_lines("input");
    
    let mut platform = Platform::new(&lines);
    platform.tilt();
    //platform.print();
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
        platform.tilt();
        platform.print();
        assert_eq!(platform.load(), 136);
    }
}
