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

#[derive(Debug, Clone)]
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

    fn replace_start(&mut self) -> () {
        let start = self.find_start();
        let valid_moves = self.valid_moves(start);
        let symbol = {
            if valid_moves[0].0 < start.0 && valid_moves[1].0 > start.0 {
                '|'
            } else if valid_moves[0].0 < start.0 && valid_moves[1].1 > start.1 {
                'L'
            } else if valid_moves[0].0 < start.0 && valid_moves[1].1 < start.1 {
                'J'
            } else if valid_moves[0].1 < start.1 && valid_moves[1].1 > start.1 {
                '-'
            } else if valid_moves[0].1 < start.1 && valid_moves[1].0 > start.0 {
                '7'
            } else if valid_moves[0].1 > start.1 && valid_moves[1].0 > start.0 {
                'F'
            } else { panic!("unknown shape for start") }
        };
        self.rows[start.0 as usize][start.1 as usize] = symbol;
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
struct MoveHistory {
    maze: Maze,
    moves: Vec<Pos>,
    already_explored: HashSet<Pos>,
}

impl MoveHistory {
    fn new(maze: &Maze) -> MoveHistory {
        let start = maze.find_start();
        let mut explored = HashSet::new();
        explored.insert(start);
        let mut m2 = maze.clone();
        m2.replace_start();
        MoveHistory{ maze: m2, moves: vec![start], already_explored: explored }
    }

    fn explore_maze(&mut self) -> () {
        let mut pos = self.maze.valid_moves(self.moves[0])[0];
        loop {
            self.moves.push(pos.clone());
            self.already_explored.insert(pos);
            let all_poss = self.maze.valid_moves(pos);
            let poss = all_poss.iter().filter(|mv| !self.already_explored.contains(mv)).collect::<Vec<_>>();
            match poss.len() {
                0 => return,
                1 => { pos = *poss[0]; },
                _ => panic!("more than one possible move"),
            }
        }
    }

    fn longest_dist(&self) -> (usize, Pos) {
        let len = self.moves.len();
        match len % 2 {
            0 => (len/2, self.moves[len/2]),
            1 => (len/2+1, self.moves[len/2]),
            _ => panic!("never reach here"),
        }
    }

    fn find_inside(&self) -> usize {
        // trace right side for inner, left side for outer
        // decide which is which at the end
        let mut inside = HashSet::new();
        let mut outside = HashSet::new();

        for i in 1..self.moves.len() {
            let prev_pos = self.moves[i-1];
            let cur_pos = self.moves[i];
            let (inner_pos, outer_pos) = match (cur_pos.0 - prev_pos.0, cur_pos.1 - prev_pos.1) {
                (1,0) => { // move down
                    match self.maze.rows[cur_pos.0 as usize][cur_pos.1 as usize] {
                        '|' => (vec![(cur_pos.0, cur_pos.1-1)], vec![(cur_pos.0, cur_pos.1+1)]),
                        'L' => (vec![(cur_pos.0, cur_pos.1-1), (cur_pos.0+1, cur_pos.1-1), (cur_pos.0+1, cur_pos.1)], Vec::new()),
                        'J' => (Vec::new(), vec![(cur_pos.0, cur_pos.1+1), (cur_pos.0+1, cur_pos.1+1), (cur_pos.0+1, cur_pos.1)]),
                        _ => panic!("bad shape")
                    }
                },
                (0,1) => { // move right
                    match self.maze.rows[cur_pos.0 as usize][cur_pos.1 as usize] {
                        '-' => (vec![(cur_pos.0+1, cur_pos.1)], vec![(cur_pos.0-1, cur_pos.1)]),
                        'J' => (vec![(cur_pos.0, cur_pos.1+1), (cur_pos.0+1, cur_pos.1+1), (cur_pos.0+1, cur_pos.1)], Vec::new()),
                        '7' => (Vec::new(), vec![(cur_pos.0-1, cur_pos.1), (cur_pos.0-1, cur_pos.1+1), (cur_pos.0, cur_pos.1+1)]),
                        _ => panic!("bad shape")
                    }
                },
                (-1,0) => { // move up
                    match self.maze.rows[cur_pos.0 as usize][cur_pos.1 as usize] {
                        '|' => (vec![(cur_pos.0, cur_pos.1+1)], vec![(cur_pos.0, cur_pos.1-1)]),
                        '7' => (vec![(cur_pos.0-1, cur_pos.1), (cur_pos.0-1, cur_pos.1+1), (cur_pos.0, cur_pos.1+1)], Vec::new()),
                        'F' => (Vec::new(), vec![(cur_pos.0, cur_pos.1-1), (cur_pos.0-1, cur_pos.1-1), (cur_pos.0-1, cur_pos.1)]),
                        _ => panic!("bad shape")
                    }
                },
                (0,-1) => { // move left
                    match self.maze.rows[cur_pos.0 as usize][cur_pos.1 as usize] {
                        '-' => (vec![(cur_pos.0-1, cur_pos.1)], vec![(cur_pos.0+1, cur_pos.1)]),
                        'F' => (vec![(cur_pos.0, cur_pos.1-1), (cur_pos.0-1, cur_pos.1-1), (cur_pos.0-1, cur_pos.1)], Vec::new()),
                        'L' => (Vec::new(), vec![(cur_pos.0, cur_pos.1-1), (cur_pos.0+1, cur_pos.1-1), (cur_pos.0+1, cur_pos.1)]),
                        _ => panic!("bad shape")
                    }
                },
                _ => panic!("cannot handle move!")
            };
            for i in inner_pos.into_iter() {
                if !(self.already_explored.contains(&i) || inside.contains(&i)) {
                    println!("adding {:?} to inner while looking at {:?}", i, cur_pos);
                    inside.insert(i);
                }
            }
            for o in outer_pos.into_iter() {
                if !(self.already_explored.contains(&o) || inside.contains(&o)) {
                    println!("adding {:?} to outer while looking at {:?}", o, cur_pos);
                    outside.insert(o);
                }
            }
        }

        // do fill in
        let row_max = self.maze.rows.len() as i64-1;
        let col_max = self.maze.rows[0].len() as i64-1;
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..=row_max {
                for j in 0..=col_max {
                    let pos = (i, j);
                    if self.already_explored.contains(&pos) || inside.contains(&pos) || outside.contains(&pos) {
                        continue;
                    }
                    let possible_positions = vec![(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)];
                    for pp in possible_positions.into_iter() {
                        if inside.contains(&pp) {
                            inside.insert(pos);
                            changed = true;
                            break
                        } else if outside.contains(&pp) {
                            outside.insert(pos);
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }

        // remove positions outside the maze
        inside = inside.into_iter().filter(|(r,c)| *r >= 0 && *r <= row_max && *c >= 0 && *c <= col_max).collect::<HashSet<_>>();
        outside = outside.into_iter().filter(|(r,c)| *r >= 0 && *r <= row_max && *c >= 0 && *c <= col_max).collect::<HashSet<_>>();

        // now decide which is inner and outer
        if inside.iter().all(|(r,c)| !(*r == 0 || *c == 0 || *r == row_max || *c == col_max)) {
            inside.len()
        } else {
            outside.len()
        }
    }
}



fn main() {
    let lines = read_lines("input");
    
    let maze = Maze::new(&lines);
    let mut mouse = MoveHistory::new(&maze);
    mouse.explore_maze();
    let inner = mouse.find_inside();
    println!("inner: {:?}", inner);
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

    #[test]
    fn test_sample3() {
        let sample: Vec<String> = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
".lines().map(String::from).collect();

        let maze = Maze::new(&sample);
        assert_eq!(maze.find_start(), (1,1));

        let mut mouse = MoveHistory::new(&maze);
        mouse.explore_maze();
        println!("mouse moves: {:?}", mouse.moves);

        let inner = mouse.find_inside();
        assert_eq!(inner, 4);
    }

    #[test]
    fn test_sample4() {
        let sample: Vec<String> = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
".lines().map(String::from).collect();

        let maze = Maze::new(&sample);
        assert_eq!(maze.find_start(), (1,1));

        let mut mouse = MoveHistory::new(&maze);
        mouse.explore_maze();
        println!("mouse moves: {:?}", mouse.moves);

        let inner = mouse.find_inside();
        assert_eq!(inner, 4);
    }

    #[test]
    fn test_sample5() {
        let sample: Vec<String> =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
".lines().map(String::from).collect();

        let maze = Maze::new(&sample);

        let mut mouse = MoveHistory::new(&maze);
        mouse.explore_maze();
        println!("mouse moves: {:?}", mouse.moves);

        let inner = mouse.find_inside();
        assert_eq!(inner, 8);
    }

    #[test]
    fn test_sample6() {
        let sample: Vec<String> =
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
".lines().map(String::from).collect();

        let maze = Maze::new(&sample);

        let mut mouse = MoveHistory::new(&maze);
        mouse.explore_maze();
        println!("mouse moves: {:?}", mouse.moves);

        let inner = mouse.find_inside();
        assert_eq!(inner, 10);
    }
}
