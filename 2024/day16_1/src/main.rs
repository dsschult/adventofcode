use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone)]
enum Direction {
    EAST,
    NORTH,
    WEST,
    SOUTH,
}

type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct Maze {
    map: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Maze {
    fn new(lines: &Vec<String>) -> Maze {
        let mut ret = Vec::new();
        let mut start = None;
        let mut end = None;
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            match trim_line.find('S') {
                Some(j) => {
                    start = Some((ret.len(), j));
                },
                _ => { }
            };
            match trim_line.find('E') {
                Some(j) => {
                    end = Some((ret.len(), j));
                },
                _ => { }
            };
            ret.push(trim_line.chars().collect::<Vec<_>>());
        }
        Maze{ map: ret, start: start.unwrap(), end: end.unwrap() }
    }

    fn min_path(&self) -> usize {
        let mut min = None;
        let mut queue = vec![(0,self.start,Direction::EAST)];
        let mut history = HashMap::new();
        while !queue.is_empty() {
            let (score, pos, dir) = queue.pop().unwrap();
            
            // check if over
            match min {
                Some(x) if x <= score => {
                    continue;
                },
                _ => { }
            };

            // check if at end
            if pos == self.end {
                println!("winner! {}", score);
                min = Some(score);
                continue;
            }

            // check if we've been here
            match history.get(&pos) {
                Some(val) => {
                    if *val <= score {
                        continue;
                    }
                },
                None => { }
            }
            history.insert(pos, score);

            //println!("pos: {:?}, dir: {:?}", pos, dir);

            // make moves
            match dir {
                Direction::EAST => {
                    if self.map[pos.0][pos.1+1] != '#' {
                        queue.push((score+1, (pos.0, pos.1 + 1), Direction::EAST));
                    }
                    if self.map[pos.0-1][pos.1] != '#' {
                        queue.push((score+1001, (pos.0 - 1, pos.1), Direction::NORTH));
                    }
                    if self.map[pos.0+1][pos.1] != '#' {
                        queue.push((score+1001, (pos.0 + 1, pos.1), Direction::SOUTH));
                    }
                },
                Direction::WEST => {
                    if self.map[pos.0][pos.1-1] != '#' {
                        queue.push((score+1, (pos.0, pos.1 - 1), Direction::WEST));
                    }
                    if self.map[pos.0-1][pos.1] != '#' {
                        queue.push((score+1001, (pos.0 - 1, pos.1), Direction::NORTH));
                    }
                    if self.map[pos.0+1][pos.1] != '#' {
                        queue.push((score+1001, (pos.0 + 1, pos.1), Direction::SOUTH));
                    }
                },
                Direction::NORTH => {
                    if self.map[pos.0-1][pos.1] != '#' {
                        queue.push((score+1, (pos.0 - 1, pos.1), Direction::NORTH));
                    }
                    if self.map[pos.0][pos.1+1] != '#' {
                        queue.push((score+1001, (pos.0, pos.1 + 1), Direction::EAST));
                    }
                    if self.map[pos.0][pos.1-1] != '#' {
                        queue.push((score+1001, (pos.0, pos.1 - 1), Direction::WEST));
                    }
                },
                Direction::SOUTH => {
                    if self.map[pos.0+1][pos.1] != '#' {
                        queue.push((score+1, (pos.0 + 1, pos.1), Direction::SOUTH));
                    }
                    if self.map[pos.0][pos.1+1] != '#' {
                        queue.push((score+1001, (pos.0, pos.1 + 1), Direction::EAST));
                    }
                    if self.map[pos.0][pos.1-1] != '#' {
                        queue.push((score+1001, (pos.0, pos.1 - 1), Direction::WEST));
                    }
                }
            };
        }
        min.unwrap()
    }
}

fn main() {
    let lines = read_lines("input");
    let mut maze = Maze::new(&lines);
    println!("score: {}", maze.min_path());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
".lines().map(String::from).collect();

        let mut maze = Maze::new(&sample);
        println!("start: {:?}, end: {:?}", maze.start, maze.end);

        assert_eq!(maze.min_path(), 7036);
    }

    #[test]
    fn test_2() {
        let sample: Vec<String> = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
".lines().map(String::from).collect();

        let mut maze = Maze::new(&sample);
        println!("start: {:?}, end: {:?}", maze.start, maze.end);

        assert_eq!(maze.min_path(), 11048);
    }
}