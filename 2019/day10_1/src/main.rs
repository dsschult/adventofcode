

type Numeric = i32;

fn common_factors(num: Numeric, num2: Numeric) -> Vec<Numeric> {
    let mut factors = Vec::new(); // creates a new vector for the factors of the number
 
    let smaller = match num < num2 {
        true => num,
        false => num2,
    };
    for i in 2..=smaller { 
        if num % i == 0 && num2 % i == 0 {
            factors.push(i);
        }
    }
    factors
}

struct Board {
    data: Vec::<Vec::<bool>>,
}

impl Board {
    fn from(input: &str) -> Board {
        let mut b = Board{data: Vec::new()};
        for line in input.lines() {
            let trim_line = line.trim();
            if trim_line.len() < 1 { continue; }
            let mut row = vec![false; trim_line.len()];
            for (i, c) in trim_line.chars().enumerate() {
                if c == '#' {
                    row[i] = true;
                }
            }
            b.data.push(row);
        }
        b
    }

    fn get(&self, col: Numeric, row: Numeric) -> bool {
        if row < 0 || row >= self.data.len() as Numeric { panic!("row out of bounds"); }
        if col < 0 || col >= self.data[0].len() as Numeric { panic!("col out of bounds"); }
        self.data[row as usize][col as usize]
    }

    fn detections(&self, col: Numeric, row: Numeric) -> Numeric {
        let row_max = self.data.len() as Numeric - 1;
        if row < 0 || row > row_max  { panic!("row out of bounds"); }
        let col_max = self.data[0].len() as Numeric - 1;
        if col < 0 || col > col_max { panic!("col out of bounds"); }
        let mut det = 0;
        for r in 0..=col_max {
            for c in 0..=row_max {
                if r == row && c == col { continue; }
                if !self.data[r as usize][c as usize] { continue; }
                // compute ray
                let mut hit = false;
                match (c - col, r - row) {
                    (0, dy) => {
                        if dy < 0 {
                            for y in r+1 .. row {
                                if self.data[y as usize][col as usize] {
                                    hit = true;
                                    break;
                                }
                            }
                        } else {
                            for y in row+1 .. r {
                                if self.data[y as usize][col as usize] {
                                    hit = true;
                                    break;
                                }
                            }
                        }
                    },
                    (dx, 0) => {
                        if dx < 0 {
                            for x in c+1 .. col {
                                if self.data[row as usize][x as usize] {
                                    hit = true;
                                    break;
                                }
                            }
                        } else {
                            for x in col+1 .. c {
                                if self.data[row as usize][x as usize] {
                                    hit = true;
                                    break;
                                }
                            }
                        }
                    },
                    (dx, dy) => {                    
                        let factors = common_factors(dx.abs(), dy.abs());
                        for f in factors.iter() {
                            let mut x = col;
                            let mut y = row;
                            loop {
                                x += dx/f;
                                y += dy/f;
                                if x < 0 || x > col_max || y < 0 || y > row_max || x == c || y == r {
                                    break;
                                }
                                if self.data[y as usize][x as usize] {
                                    hit = true;
                                    break;
                                }
                            }
                            if hit {
                                break;
                            }
                        }
                    },
                }
                if !hit {
                    det += 1;
                }
                //println!("({},{}) -> ({},{}) hit {}", col, row, c, r, hit);
            }
        }
        det
    }

    fn find_highest_detections(&self) -> (Numeric, Numeric) {
        let mut max_det = 0;
        let mut ret = (-1,-1);
        for row in 0..self.data.len() as Numeric {
            for col in 0..self.data[0].len() as Numeric {
                if self.data[row as usize][col as usize] {
                    let det = self.detections(col, row);
                    println!("{} detections at ({},{})", det, col, row);
                    if det > max_det {
                        max_det = det;
                        ret = (col,row);
                    }
                }
            }
        }
        ret
    }
}

fn main() {
    let input = "
.............#..#.#......##........#..#
.#...##....#........##.#......#......#.
..#.#.#...#...#...##.#...#.............
.....##.................#.....##..#.#.#
......##...#.##......#..#.......#......
......#.....#....#.#..#..##....#.......
...................##.#..#.....#.....#.
#.....#.##.....#...##....#####....#.#..
..#.#..........#..##.......#.#...#....#
...#.#..#...#......#..........###.#....
##..##...#.#.......##....#.#..#...##...
..........#.#....#.#.#......#.....#....
....#.........#..#..##..#.##........#..
........#......###..............#.#....
...##.#...#.#.#......#........#........
......##.#.....#.#.....#..#.....#.#....
..#....#.###..#...##.#..##............#
...##..#...#.##.#.#....#.#.....#...#..#
......#............#.##..#..#....##....
.#.#.......#..#...###...........#.#.##.
........##........#.#...#.#......##....
.#.#........#......#..........#....#...
...............#...#........##..#.#....
.#......#....#.......#..#......#.......
.....#...#.#...#...#..###......#.##....
.#...#..##................##.#.........
..###...#.......#.##.#....#....#....#.#
...#..#.......###.............##.#.....
#..##....###.......##........#..#...#.#
.#......#...#...#.##......#..#.........
#...#.....#......#..##.............#...
...###.........###.###.#.....###.#.#...
#......#......#.#..#....#..#.....##.#..
.##....#.....#...#.##..#.#..##.......#.
..#........#.......##.##....#......#...
##............#....#.#.....#...........
........###.............##...#........#
#.........#.....#..##.#.#.#..#....#....
..............##.#.#.#...........#.....";
    
    let b = Board::from(&input);
    let (col,row) = b.find_highest_detections();
    println!("best location ({}, {})", col, row);
    println!("detections: {}", b.detections(col, row));
}

mod tests {
    use super::*;

    #[test]
    fn test_common_factors() {
        assert_eq!(common_factors(20,30), vec![2,5,10]);
        assert_eq!(common_factors(10,5), vec![5]);
        assert_eq!(common_factors(3,3), vec![3]);
    }

    #[test]
    fn test_board_from() {
        let input = "
.#..
....
####
....
...#";
        let b = Board::from(&input);
        assert_eq!(b.data.len(), 5);
        assert_eq!(b.data[0].len(), 4);
        assert!(b.get(1,0));
        assert!(!b.get(0,1));
        assert!(b.get(0,2));
        assert!(!b.get(3,3));
        assert!(b.get(3,4));
    }

    #[test]
    fn test_detections() {
        let input = "
.#..#
.....
#####
....#
...##";
        let b = Board::from(&input);
        assert_eq!(b.detections(1,0), 7);
        assert_eq!(b.detections(4,0), 7);
        assert_eq!(b.detections(0,2), 6);
        assert_eq!(b.detections(1,2), 7);
        assert_eq!(b.detections(2,2), 7);
        assert_eq!(b.detections(3,2), 7);
        assert_eq!(b.detections(4,2), 5);
        assert_eq!(b.detections(4,3), 7);
        assert_eq!(b.detections(3,4), 8);
        assert_eq!(b.detections(4,4), 7);
    }

    #[test]
    fn test_day10a_1() {
        let input = "
.#..#
.....
#####
....#
...##";
        let b = Board::from(&input);
        assert_eq!(b.find_highest_detections(), (3,4));
    }

    #[test]
    fn test_day10a_2() {
        let input = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let b = Board::from(&input);
        assert_eq!(b.find_highest_detections(), (5,8));
    }

    #[test]
    fn test_day10a_3() {
        let input = "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let b = Board::from(&input);
        assert_eq!(b.find_highest_detections(), (1,2));
    }

    #[test]
    fn test_day10a_4() {
        let input = "
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let b = Board::from(&input);
        assert_eq!(b.find_highest_detections(), (6,3));
    }

    #[test]
    fn test_day10a_5() {
        let input = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let b = Board::from(&input);
        assert_eq!(b.find_highest_detections(), (11,13));
    }
}