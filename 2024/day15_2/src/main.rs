use std::fs::read_to_string;
use std::collections::VecDeque;
use std::collections::HashSet;

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
    move_set: HashSet<Pos>
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
                let mut row = Vec::new();
                for c in trim_line.chars() {
                    if c == '@' {
                        robot = Some((map.len() as i64, row.len() as i64));
                        row.push('@');
                        row.push('.');
                    } else if c == 'O' {
                        row.push('[');
                        row.push(']');
                    } else if c == '#' {
                        row.push('#');
                        row.push('#');
                    } else {
                        row.push('.');
                        row.push('.');
                    }
                }
                map.push(row);
            }
        }
        Warehouse{ map: map, robot_moves: moves, robot_pos: robot.unwrap(), move_set: HashSet::new() }
    }

    fn can_move_obj(&mut self, pos: Pos, dir: &Pos) -> bool {
        match self.map[pos.0 as usize][pos.1 as usize] {
            '#' => {
                // wall
                false
            },
            '.' => {
                // empty space
                true
            },
            c if c == '[' || c == ']' => {
                // box
                if dir.0 != 0 {
                    // moving vertically, so move whole box
                    let other = match c {
                        '[' => (pos.0, pos.1 + 1),
                        _ => (pos.0, pos.1 - 1)
                    };
                    let next_pos1 = (pos.0 + dir.0, pos.1 + dir.1);
                    let next_pos2 = (other.0 + dir.0, other.1 + dir.1);
                    if self.can_move_obj(next_pos1, dir) && self.can_move_obj(next_pos2, dir) {
                        // they can both move
                        true
                    } else {
                        false
                    }
                } else {
                    // moving horizontally
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if self.move_obj(next_pos, dir) {
                        true
                    } else {
                        false
                    }
                }
            },
            x => panic!("unknown object: {}", x)
        }
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
            c if c == '[' || c == ']' => {
                // box
                if dir.0 != 0 {
                    // moving vertically, so move whole box
                    let other = match c {
                        '[' => (pos.0, pos.1 + 1),
                        _ => (pos.0, pos.1 - 1)
                    };
                    let next_pos1 = (pos.0 + dir.0, pos.1 + dir.1);
                    let next_pos2 = (other.0 + dir.0, other.1 + dir.1);
                    let do_move_1 = self.move_set.insert(next_pos1);
                    let do_move_2 = self.move_set.insert(next_pos2);
                    if self.can_move_obj(next_pos1, dir) && self.can_move_obj(next_pos2, dir) {
                        // they can both move
                        self.move_obj(next_pos1, dir);
                        self.move_obj(next_pos2, dir);
                        
                        let (p,o) = match c {
                            '[' => ('[', ']'),
                            _ => (']', '[')
                        };
                        if do_move_1 {
                            self.map[pos.0 as usize][pos.1 as usize] = '.';
                            self.map[next_pos1.0 as usize][next_pos1.1 as usize] = p;
                        }
                        if do_move_2 {
                            self.map[other.0 as usize][other.1 as usize] = '.';
                            self.map[next_pos2.0 as usize][next_pos2.1 as usize] = o;
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    // moving horizontally
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if self.move_obj(next_pos, dir) {
                        self.map[pos.0 as usize][pos.1 as usize] = '.';
                        self.map[next_pos.0 as usize][next_pos.1 as usize] = c;
                        true
                    } else {
                        false
                    }
                }
            },
            x => panic!("unknown object: {}", x)
        }
    }

    fn all_moves(&mut self) {
        let mut pos = self.robot_pos;
        for mv in self.robot_moves.clone() {
            self.move_set = HashSet::new();
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
            //self.print();
        }
    }

    fn sum_coords(&self) -> usize {
        let mut ret = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == '[' {
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
    w.print();
    println!("sum: {}", w.sum_coords());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
#######
#.....#
#.OO@.#
#.....#
#######

<<
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.print();
        w.all_moves();
        w.print();
        assert_eq!(w.sum_coords(), 406);
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
        w.print();
        w.all_moves();
        w.print();
        assert_eq!(w.sum_coords(), 9021);
    }

    #[test]
    fn test_3() {
        let sample: Vec<String> = "
#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.print();
        w.all_moves();
        w.print();
        assert_eq!(w.sum_coords(), 509);
    }

    #[test]
    fn test_4() {
        let sample: Vec<String> = "
#######
#.....#
#.#O..#
#..O@.#
#.....#
#######

<v<^
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.print();
        w.all_moves();
        w.print();
        assert_eq!(w.sum_coords(), 511);
    }

    #[test]
    fn test_5() {
        let sample: Vec<String> = "
######
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.print();
        w.all_moves();
        w.print();
        assert_eq!(w.sum_coords(), 816);
    }

    #[test]
    fn test_6() {
        let sample: Vec<String> = "
#######
#.....#
#.O.O@#
#..O..#
#..O..#
#.....#
#######

<v<<>vv<^^
".lines().map(String::from).collect();

        let mut w = Warehouse::new(&sample);
        w.print();
        w.all_moves();
        w.print();
        assert_eq!(w.sum_coords(), 822);
    }

}