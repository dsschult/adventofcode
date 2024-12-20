use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = u32;
type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct Memory {
    grid: Vec<Vec<bool>>,
    byte_stream: Vec<Pos>,
    start: Pos,
    exit: Pos,
}

impl Memory {
    fn new(lines: &Vec<String>, dims: Pos) -> Memory {
        let g = vec![vec![false; dims.1+1]; dims.0+1];
        let mut b = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            let parts = trim_line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            b.push((parts[1], parts[0]));
        }
        Memory{
            grid: g,
            byte_stream: b,
            start: (0,0),
            exit: dims,
        }
    }

    fn memory_fall(&mut self, num: usize) {
        assert!(self.byte_stream.len() >= num);
        for pos in self.byte_stream.iter().take(num) {
            self.grid[pos.0][pos.1] = true;
        }
    }

    fn min_steps(&self) -> Option<usize> {
        let mut min = None;
        let mut queue = vec![(0, self.start)];
        let mut history = HashMap::new();
        let max_row = self.grid.len()-1;
        let max_col = self.grid[0].len()-1;
        while !queue.is_empty() {
            let (steps, pos) = queue.pop().unwrap();

            match min {
                Some(x) if x < steps => {
                    continue;
                },
                _ => { }
            };

            if pos == self.exit {
                min = Some(steps);
                continue
            }

            match history.get(&pos) {
                Some(x) if *x <= steps => {
                    continue;
                },
                _ => { }
            };
            history.insert(pos, steps);

            if pos.0 != 0 && !self.grid[pos.0-1][pos.1] {
                // try up
                queue.push((steps+1, (pos.0-1, pos.1)));
            }
            if pos.0 != max_row && !self.grid[pos.0+1][pos.1] {
                // try down
                queue.push((steps+1, (pos.0+1, pos.1)));
            }
            if pos.1 != 0 && !self.grid[pos.0][pos.1-1] {
                // try left
                queue.push((steps+1, (pos.0, pos.1-1)));
            }
            if pos.1 != max_col && !self.grid[pos.0][pos.1+1] {
                // try right
                queue.push((steps+1, (pos.0, pos.1+1)));
            }
        }

        min
    }

    fn first_cutoff(&mut self) -> Pos {
        for i in 0..self.byte_stream.len() {
            let pos = self.byte_stream[i];
            self.grid[pos.0][pos.1] = true;
            if self.min_steps().is_none() {
                return (pos.1, pos.0);
            }
        }
        panic!("no cutoff!");
    }
}

fn main() {
    let lines = read_lines("input");
    let mut c = Memory::new(&lines, (70,70));
    println!("cutoff: {:?}", c.first_cutoff());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
".lines().map(String::from).collect();

        let mut c = Memory::new(&sample, (6,6));
        assert_eq!(c.first_cutoff(), (6,1));
    }
}