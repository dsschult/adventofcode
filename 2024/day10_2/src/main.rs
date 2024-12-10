use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Pos = (usize,usize);

#[derive(Debug, Clone)]
struct Grid {
    map: Vec<Vec<char>>,
}

impl Grid {
    fn new(lines: &Vec<String>) -> Grid {
        let mut ret = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            ret.push(trim_line.chars().collect::<Vec<_>>())
        }
        Grid{ map: ret }
    }

    fn find_trailheads(&self) -> Vec<Vec<Pos>> {
        let mut ret = Vec::new();
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == '0' {
                    ret.push(vec![(i,j)]);
                }
            }
        }
        ret
    }

    fn higher(&self, pos: Pos) -> Option<char> {
        match self.map[pos.0][pos.1] {
            '0' => Some('1'),
            '1' => Some('2'),
            '2' => Some('3'),
            '3' => Some('4'),
            '4' => Some('5'),
            '5' => Some('6'),
            '6' => Some('7'),
            '7' => Some('8'),
            '8' => Some('9'),
            _ => None
        }
    }

    fn find_trails(&self) -> Vec<Vec<Pos>> {
        let mut ret = Vec::new();
        let mut queue = self.find_trailheads();
        let max_row = self.map.len()-1;
        let max_col = self.map[0].len()-1;
        while !queue.is_empty() {
            let route: Vec<Pos> = queue.pop().unwrap();
            let pos = route.iter().last().unwrap().clone();
            let nextval = self.higher(pos);
            if nextval == None {
                // found number 9
                ret.push(route);
                continue;
            }
            // search surroundings for next highest number
            let possible_pos = match pos {
                (0,0) => vec![(0,1), (1,0)],
                (0,y) if y == max_col => vec![(0,max_col-1), (1, max_col)],
                (x,0) if x == max_row => vec![(max_row-1,0), (max_row,1)],
                (x,y) if x == max_row && y == max_col => vec![(max_row-1,max_col), (max_row, max_col-1)],
                (0,y) => vec![(0,y-1), (1, y), (0, y+1)],
                (x,y) if x == max_row => vec![(max_row,y-1), (max_row-1,y), (max_row,y+1)],
                (x,0) => vec![(x-1,0), (x,1), (x+1,0)],
                (x,y) if y == max_col => vec![(x-1,max_col), (x,max_col-1), (x+1,max_col)],
                (x,y) => vec![(x-1,y), (x,y-1), (x+1,y), (x,y+1)]
            };
            for (x,y) in possible_pos.into_iter() {
                if self.map[x][y] == nextval.unwrap() {
                    let mut route2 = route.clone();
                    route2.push((x,y));
                    queue.push(route2);
                }
            }
        }
        ret
    }

    fn find_ratings(&self) -> usize {
        let mut ret = HashMap::new();
        for route in self.find_trails().into_iter() {
            let start = route[0];
            *ret.entry(start).or_insert(0) += 1;
        }
        println!("{:?}", ret);
        ret.into_values().sum()
    }
}

fn main() {
    let lines = read_lines("input");
    let fs = Grid::new(&lines);
    println!("trail sum: {}", fs.find_ratings());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
012345
123456
234567
345678
4.6789
56789.
".lines().map(String::from).collect();

        let fs = Grid::new(&sample);
        assert_eq!(fs.find_ratings(), 227);
    }

    #[test]
    fn test_10() {
        let sample: Vec<String> = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
".lines().map(String::from).collect();

        let fs = Grid::new(&sample);
        assert_eq!(fs.find_ratings(), 81);
    }
}