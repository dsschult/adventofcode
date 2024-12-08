use std::fs::read_to_string;
use std::collections::HashSet;
use itertools::Itertools;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Pair = (i32, i32);

fn antinodes(pos1: Pair, pos2: Pair) -> (Pair, Pair) {
    let xdiff = (pos1.0 - pos2.0).abs();
    let ydiff = (pos1.1 - pos2.1).abs();
    match (pos1.0 < pos2.0, pos1.1 < pos2.1) {
        (true, true) => (
                         (pos1.0 - xdiff, pos1.1 - ydiff),
                         (pos2.0 + xdiff, pos2.1 + ydiff)
                        ),
        (true, false) => (
                         (pos1.0 - xdiff, pos1.1 + ydiff),
                         (pos2.0 + xdiff, pos2.1 - ydiff)
                        ),
        (false, true) => (
                         (pos1.0 + xdiff, pos1.1 - ydiff),
                         (pos2.0 - xdiff, pos2.1 + ydiff)
                        ),
        (false, false) => (
                         (pos1.0 + xdiff, pos1.1 + ydiff),
                         (pos2.0 - xdiff, pos2.1 - ydiff)
                        )
    }       
}

#[derive(Debug, Clone)]
struct Antennas {
    grid: Vec<Vec<char>>,
    points: Vec<(char, Pair)>,
}

impl Antennas {
    fn new(lines: &Vec<String>) -> Antennas {
        let mut grid = Vec::new();
        let mut points = Vec::new();
        let mut i: i32 = 0;
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            let c = line.chars().collect::<Vec<_>>();
            for j in 0..c.len() {
                if c[j].is_alphanumeric() {
                    points.push((c[j], (i,j as i32)))
                }
            }
            grid.push(c);
            i += 1;
        }
        Antennas{grid: grid, points: points}
    }

    fn count_antinodes(&self) -> usize {
        let max_rows = self.grid.len() as i32;
        let max_cols = self.grid[0].len() as i32;
        let mut nodes = HashSet::new();
        for vals in self.points.iter().combinations(2) {
            if vals[0].0 == vals[1].0 {
                println!("analzying pairs {:?}", vals);
                let (a1, a2) = antinodes(vals[0].1, vals[1].1);
                if a1.0 >= 0 && a1.0 < max_rows && a1.1 >= 0 && a1.1 < max_cols {
                    println!("adding {:?}", a1);
                    nodes.insert(a1);
                }
                if a2.0 >= 0 && a2.0 < max_rows && a2.1 >= 0 && a2.1 < max_cols {
                    println!("adding {:?}", a2);
                    nodes.insert(a2);
                }
            }
        }
        nodes.len()
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Antennas::new(&lines);
    println!("valid: {}", c.count_antinodes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(antinodes((1,8),(2,5)), ((0,11), (3,2)));
        
        assert_eq!(antinodes((2,5),(3,7)), ((1,3), (4,9)));
        
        assert_eq!(antinodes((1,1),(1,2)), ((1,0), (1,3)));
    }


    #[test]
    fn test_10() {
        let sample: Vec<String> = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
".lines().map(String::from).collect();

        let c = Antennas::new(&sample);
        assert_eq!(c.count_antinodes(), 14);
    }
}
