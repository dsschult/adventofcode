use std::fs::read_to_string;
use std::collections::VecDeque;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = usize;

type Pos = (i64, i64);

#[derive(Debug, Clone)]
struct Warehouse {
    map: Vec<Vec<char>>,
    robot_moves: Vec<char>,
    robot_pos: Pos,
}

impl Warehouse {
    fn new(lines: &Vec<String>) -> Warehouse {
        let mut map = Vec::new();
        let mut moves = Vec::new();
        let mut robot = None;
        let mut find_moves = false;

        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                if robot.is_some() {
                    find_moves = true;
                }
                continue;
            }
            if find_moves {
                moves.extend(trim_line.chars());
            } else {
                let mut j = 0;
                for c in trim_line.chars() {
                    if c == '@' {
                        robot = Some((map.len() as i64, j));
                        break;
                    }
                    j += 1;
                }
                map.push(trim_line.chars().collect::<Vec<_>>());
            }
        }
        Warehouse{ map: map, robot_moves: moves, robot_pos: robot.unwrap() }
    }

    fn move_obj(&mut self, pos: Pos, dir: &Pos) -> bool {
        match self.map[pos.0 as usize][pos.1 as usize] {
            '#' => {
                // wall
                false
            },
            '.' => {
                // empty space
                true
            },
            'O' => {
                // box
                let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if self.move_obj(next_pos, dir) {
                    self.map[pos.0 as usize][pos.1 as usize] = '.';
                    self.map[next_pos.0 as usize][next_pos.1 as usize] = 'O';
                    true
                } else {
                    false
                }
            },
            _ => panic!("unknown object")
        }
    }

    fn all_moves(&mut self) {
        let mut pos = self.robot_pos;
        for mv in self.robot_moves.clone() {
            let dir = match mv {
                '<' => {
                    // left
                    (0, -1)
                },
                '>' => {
                    // right
                    (0, 1)
                },
                '^' => {
                    // up
                    (-1, 0)
                },
                'v' => {
                    // down
                    (1, 0)
                },
                _ => panic!("unknown move"),
            };
            println!("robot at {:?}, moving {:?}", pos, dir);
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if self.move_obj(next_pos, &dir) {
                // it moved, so we can move into here
                self.map[pos.0 as usize][pos.1 as usize] = '.';
                self.map[next_pos.0 as usize][next_pos.1 as usize] = '@';
                pos = next_pos;
            }
        }
    }

    fn sum_coords(&self) -> usize {
        let mut ret = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == 'O' {
                    ret += 100 * i + j;
                }
            }
        }
        ret
    }

    fn print(&self) {
        for row in self.map.iter() {
            for col in row {
                print!("{}", col);
            }
            println!("");
        }
    }
}

fn main() {
    let lines = read_lines("input");
    let mut w = Warehouse::new(&lines);
    w.all_moves();
    println!("sum: {}", w.sum_coords());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.all_moves();
        assert_eq!(w.sum_coords(), 2028);
    }

    #[test]
    fn test_2() {
        let sample: Vec<String> = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.all_moves();
        assert_eq!(w.sum_coords(), 10092);
    }

}