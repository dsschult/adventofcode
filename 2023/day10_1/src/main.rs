use std::fs::read_to_string;
use std::ops::Range;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn to_values(line: &str) -> Vec<i64> {
    line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

const PIPES: &str = "|-LJ7F";
const START: char = 'S';
const GROUND: char = '.';

type Pos = (i64, i64);

#[derive(Debug)]
struct Maze {
    rows: Vec<Vec<char>>
}

impl Maze {
    fn new(lines: &Vec<String>) -> Maze {
        Maze{ rows: lines.iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>() }
    }

    fn find_start(&self) -> Pos {
        for (i,row) in self.rows.iter().enumerate() {
            for (j,col) in row.iter().enumerate() {
                if *col == START {
                    return (i as i64, j as i64);
                }
            }
        }
        panic!("No start!");
    }
    
    fn valid_moves(&self, p: Pos) -> Vec<Pos> {
        let ret = match self.rows[p.0 as usize][p.1 as usize] {
            '|' => vec![(p.0-1, p.1), (p.0+1, p.1)],
            '-' => vec![(p.0, p.1-1), (p.0, p.1+1)],
            'L' => vec![(p.0-1, p.1), (p.0, p.1+1)],
            'J' => vec![(p.0-1, p.1), (p.0, p.1-1)],
            '7' => vec![(p.0, p.1-1), (p.0+1, p.1)],
            'F' => vec![(p.0, p.1+1), (p.0+1, p.1)],
            START => { // look at surrounding pipes
                let row_len = self.rows.len() as i64;
                let row_range = match p.0 {
                    0 => Range{ start: 0, end: 2 },
                    x => if x >= row_len-1 {
                        Range{ start: row_len-2, end: row_len }
                    } else {
                        Range{ start: x-1, end: x+2 }
                    },
                };
                let col_len = self.rows[0].len() as i64;
                let col_range = match p.1 {
                    0 => Range{ start: 0, end: 2 },
                    x => if x >= col_len-1 {
                        Range{ start: col_len-2, end: col_len }
                    } else {
                        Range{ start: x-1, end: x+2 }
                    },
                };
                //println!("row_range: {:?}", row_range);
                //println!("col_range: {:?}", col_range);

                let mut ret2 = Vec::new();
                for i in row_range {
                    for j in col_range.clone() {
                        if (i,j) != p  {
                            //println!("testing: {:?}", (i,j));
                            match self.rows[i as usize][j as usize] {
                                '|' => if i > 0 && (i-1,j) == p || (i+1,j) == p {
                                    ret2.push((i,j));
                                },
                                '-' => if j > 0 && (i,j-1) == p || (i,j+1) == p {
                                    ret2.push((i,j));
                                },
                                'L' => if i > 0 && (i-1,j) == p || (i,j+1) == p {
                                    ret2.push((i,j));
                                },
                                'J' => if i > 0 && (i-1,j) == p || j > 0 && (i,j-1) == p {
                                    ret2.push((i,j));
                                },
                                '7' => if j > 0 && (i,j-1) == p || (i+1,j) == p {
                                    ret2.push((i,j));
                                },
                                'F' => if (i,j+1) == p || (i+1,j) == p {
                                    ret2.push((i,j));
                                },
                                _ => { }
                            };
                        }
                    }
                }
                ret2
            },
            _ => panic!("invalid pipe")
        };
        ret.into_iter().filter(|(a,b)| !(*a < 0 || *a >= self.rows.len() as i64 || *b < 0 || *b >= self.rows[0].len() as i64)).collect::<Vec<Pos>>()
    }
}

#[derive(Debug)]
struct MoveHistory<'a> {
    maze: &'a Maze,
    moves: Vec<Vec<Pos>>,
    already_explored: HashSet<Pos>,
}

impl MoveHistory<'_> {
    fn new(maze: &Maze) -> MoveHistory {
        let start = maze.find_start();
        let mut explored = HashSet::new();
        explored.insert(start);
        MoveHistory{ maze: maze, moves: vec![vec![start]], already_explored: explored }
    }

    fn explore_maze(&mut self) -> () {
        let mut explore_len = 0;
        while self.already_explored.len() != explore_len {
            explore_len = self.already_explored.len();
            let mut new_moves = Vec::new();
            for moves in self.moves.iter_mut() {
                let last_move = moves[moves.len()-1];
                let all_poss = self.maze.valid_moves(last_move);
                let poss = all_poss.iter().filter(|mv| !self.already_explored.contains(mv)).collect::<Vec<_>>();
                match poss.len() {
                    0 => { new_moves.push(moves.to_vec()); },
                    _ => {
                        for mv in poss.into_iter() {
                            let mut n = moves.to_vec();
                            n.push(mv.clone());
                            new_moves.push(n);
                            self.already_explored.insert(*mv);
                        }
                    }
                }
            }
            self.moves = new_moves;
        }
    }

    fn longest_dist(&self) -> (usize, Pos) {
        let mut max = 0;
        let mut max_pos = (0,0);
        for moves in self.moves.iter() {
            let l = moves.len();
            if l > max {
                max = l;
                max_pos = moves[l-1];
            }
        }
        (max-1, max_pos)
    }
}



fn main() {
    let lines = read_lines("input");
    
    let maze = Maze::new(&lines);
    let mut mouse = MoveHistory::new(&maze);
    mouse.explore_maze();
    let longest_dist = mouse.longest_dist();
    println!("longest_dist: {:?}", longest_dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = ".....
.S-7.
.|.|.
.L-J.
.....
".lines().map(String::from).collect();

        let maze = Maze::new(&sample);
        assert_eq!(maze.find_start(), (1,1));

        assert_eq!(maze.valid_moves((2,1)), vec![(1,1), (3,1)]);
        assert_eq!(maze.valid_moves((3,1)), vec![(2,1), (3,2)]);
        assert_eq!(maze.valid_moves((3,2)), vec![(3,1), (3,3)]);
        assert_eq!(maze.valid_moves((3,3)), vec![(2,3), (3,2)]);
        assert_eq!(maze.valid_moves((2,3)), vec![(1,3), (3,3)]);
        assert_eq!(maze.valid_moves((1,3)), vec![(1,2), (2,3)]);
        assert_eq!(maze.valid_moves((1,2)), vec![(1,1), (1,3)]);

        // now for start
        assert_eq!(maze.valid_moves((1,1)), vec![(1,2), (2,1)]);
    }

    #[test]
    fn test_sample2() {
        let sample: Vec<String> = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
".lines().map(String::from).collect();

        let maze = Maze::new(&sample);
        assert_eq!(maze.find_start(), (1,1));

        assert_eq!(maze.valid_moves((2,1)), vec![(1,1), (3,1)]);
        assert_eq!(maze.valid_moves((3,1)), vec![(2,1), (3,2)]);
        assert_eq!(maze.valid_moves((3,2)), vec![(3,1), (3,3)]);
        assert_eq!(maze.valid_moves((3,3)), vec![(2,3), (3,2)]);
        assert_eq!(maze.valid_moves((2,3)), vec![(1,3), (3,3)]);
        assert_eq!(maze.valid_moves((1,3)), vec![(1,2), (2,3)]);
        assert_eq!(maze.valid_moves((1,2)), vec![(1,1), (1,3)]);

        // now for start
        assert_eq!(maze.valid_moves((1,1)), vec![(1,2), (2,1)]);

        let mut mouse = MoveHistory::new(&maze);
        mouse.explore_maze();
        println!("mouse moves: {:?}", mouse.moves);
        let longest_dist = mouse.longest_dist();
        assert_eq!(longest_dist, (4, (3,3)));
    }
}
