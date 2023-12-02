use std::f32;

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


fn calc_angle(p1: (Numeric, Numeric), p2: (Numeric, Numeric)) -> f32 {
    /* Get angle from pos1 to pos2.
     *
     * Assuming a (col, row) input:
     *
     *   --------p2
     *   |      /
     *   |     /
     *   |    /
     *   | a /
     *   |  /
     *   | /
     *  p1
     */
    let dx = p2.0-p1.0;
    let dy = p1.1-p2.1;
    let mut ret = if dy >= 0 && dy >= 0 { // quadrant 1
        (dx as f32 / dy as f32).atan()
    } else if dx >= 0 && dy < 0 { // quadrant 2
        (dy.abs() as f32 / dx as f32).atan()+f32::consts::FRAC_PI_2
    } else if dx < 0 && dy < 0 { // quadrant 3
        (dx.abs() as f32 / dy.abs() as f32).atan()+f32::consts::PI
    } else { // quadrant 4
        (dy as f32 / dx.abs() as f32).atan()+f32::consts::FRAC_PI_2*3.
    };
    if ret < 0. {
        ret += f32::consts::PI*2.;
    }
    ret
}

fn calc_dist(p1: (Numeric, Numeric), p2: (Numeric, Numeric)) -> f32 {
    /* Get distance from p1 to p2 */
    (((p1.0-p2.0).pow(2) + (p1.1-p2.1).pow(2)) as f32).sqrt()
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

    fn clockwise_ordering_raw(&self, col: Numeric, row: Numeric) -> Vec<(f32, f32, (Numeric, Numeric))> {
        let row_max = self.data.len() as Numeric - 1;
        if row < 0 || row > row_max  { panic!("row out of bounds"); }
        let col_max = self.data[0].len() as Numeric - 1;
        if col < 0 || col > col_max { panic!("col out of bounds"); }

        let mut angles = Vec::new();
        for r in 0..=row_max {
            for c in 0..=col_max {
                if r == row && c == col {
                    continue;
                }
                if self.data[r as usize][c as usize] {
                    let val = (calc_angle((col,row),(c,r)), calc_dist((col,row),(c,r)), (c,r));
                    angles.push(val);
                }
            }
        }
        angles.sort_unstable_by(|a,b|{
            if a.0 != b.0 {
                a.0.partial_cmp(&b.0).unwrap()
            } else {
                a.1.partial_cmp(&b.1).unwrap()
            }
        });
        angles
    }

    fn clockwise_ordering(&self, col: Numeric, row: Numeric) -> Vec<(Numeric, Numeric)> {
        let angles = self.clockwise_ordering_raw(col, row);
        let mut ret = Vec::new();
        for (_angle, _dist, pos) in angles {
            ret.push(pos);
        }
        ret
    }

    fn len(&self) -> usize {
        let mut ret = 0;
        for row in self.data.iter() {
            for pos in row.iter() {
                if *pos {
                    ret += 1;
                }
            }
        }
        ret
    }

    fn vaporize(&mut self, col: Numeric, row: Numeric) -> () {
        if row < 0 || row >= self.data.len() as Numeric { panic!("row out of bounds"); }
        if col < 0 || col >= self.data[0].len() as Numeric { panic!("col out of bounds"); }
        self.data[row as usize][col as usize] = false
    }

    fn laser_from(&mut self, col: Numeric, row: Numeric) -> Vec<(Numeric, Numeric)> {
        let row_max = self.data.len() as Numeric - 1;
        if row < 0 || row > row_max  { panic!("row out of bounds"); }
        let col_max = self.data[0].len() as Numeric - 1;
        if col < 0 || col > col_max { panic!("col out of bounds"); }

        let mut destructions = Vec::new();
        while self.len() > 1 {
            let mut last = -1.;
            for (angle,_dist,pos) in self.clockwise_ordering_raw(col, row) {
                if last == angle {
                    continue; // blocked
                }
                destructions.push(pos);
                self.vaporize(pos.0, pos.1);
                last = angle;
            }
        }
        destructions
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
    
    let mut b = Board::from(&input);
    let (col,row) = b.find_highest_detections();
    println!("best location ({}, {})", col, row);
    println!("detections: {}", b.detections(col, row));

    let destruction_order = b.laser_from(col, row);
    let (x,y) = destruction_order[199];
    println!("200th asteroid destroyed: ({}, {})", x, y);
    println!("x*100+y = {}", x*100+y);
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
    fn test_calc_angle() {
        let mut a = calc_angle((2,2),(2,0));
        assert_eq!(a, 0.);
        a = calc_angle((2,2),(4,0));
        assert_eq!(a, f32::consts::FRAC_PI_4);
        a = calc_angle((2,2),(0,0));
        assert_eq!(a, f32::consts::FRAC_PI_4*7.);
        a = calc_angle((2,2),(0,2));
        assert_eq!(a, f32::consts::FRAC_PI_2*3.);
        a = calc_angle((2,2),(4,2));
        assert_eq!(a, f32::consts::FRAC_PI_2);
        a = calc_angle((2,2),(0,4));
        assert_eq!(a, f32::consts::FRAC_PI_4*5.);
        a = calc_angle((2,2),(4,4));
        assert_eq!(a, f32::consts::FRAC_PI_4*3.);
    }

    #[test]
    fn test_calc_angle2() {
        let mut a = calc_angle((2,2),(3,0));
        let mut b = calc_angle((2,2),(4,0));
        assert!(b > a);
        a = calc_angle((2,2),(3,0));
        b = calc_angle((2,2),(3,1));
        assert!(b > a);
        a = calc_angle((2,2),(2,0));
        b = calc_angle((2,2),(1,0));
        assert!(b > a);
        a = calc_angle((2,2),(4,2));
        b = calc_angle((2,2),(4,3));
        assert!(b > a);
        a = calc_angle((2,2),(4,3));
        b = calc_angle((2,2),(3,4));
        assert!(b > a);
        a = calc_angle((2,2),(4,3));
        b = calc_angle((2,2),(2,4));
        assert!(b > a);
    }

    #[test]
    fn test_calc_dist() {
        assert_eq!(calc_dist((0,0),(0,2)), 2.);
        assert_eq!(calc_dist((0,0),(4,0)), 4.);
        assert_eq!(calc_dist((4,4),(4,0)), 4.);
        assert_eq!(calc_dist((4,0),(2,0)), 2.);
        assert_eq!(calc_dist((3,4),(0,8)), 5.);
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

    #[test]
    fn test_day10a_test() {
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
        assert_eq!(b.find_highest_detections(), (26, 29));
        assert_eq!(b.detections(26, 29), 299);
    }

    #[test]
    fn test_clockwise_array() {
        let input = "
#####
#####
#####
#####
#####";
        let b = Board::from(&input);
        let clockwise_order = b.clockwise_ordering(2, 2);
        assert_eq!(clockwise_order.len(), 24);
        assert_eq!(clockwise_order[0], (2,1));
        assert_eq!(clockwise_order[1], (2,0));
        assert_eq!(clockwise_order[2], (3,0));
        assert_eq!(clockwise_order[3], (3,1));
        assert_eq!(clockwise_order[4], (4,0));
        assert_eq!(clockwise_order[5], (4,1));
        assert_eq!(clockwise_order[6], (3,2));
        assert_eq!(clockwise_order[7], (4,2));
        assert_eq!(clockwise_order[8], (4,3));
        assert_eq!(clockwise_order[9], (3,3));
        assert_eq!(clockwise_order[10], (4,4));
        assert_eq!(clockwise_order[11], (3,4));
        assert_eq!(clockwise_order[12], (2,3));
        assert_eq!(clockwise_order[13], (2,4));
        assert_eq!(clockwise_order[14], (1,4));
        assert_eq!(clockwise_order[15], (1,3));
        assert_eq!(clockwise_order[16], (0,4));
        assert_eq!(clockwise_order[17], (0,3));
        assert_eq!(clockwise_order[18], (1,2));
        assert_eq!(clockwise_order[19], (0,2));
        assert_eq!(clockwise_order[20], (0,1));
        assert_eq!(clockwise_order[21], (1,1));
        assert_eq!(clockwise_order[22], (0,0));
        assert_eq!(clockwise_order[23], (1,0));
    }

    #[test]
    fn test_clockwise_array2() {
        let input = "
#####
#####
#####
#####
#####";
        let b = Board::from(&input);
        let clockwise_order = b.clockwise_ordering(2, 2);
        let should_be = vec![(2,1), (2,0), (3,0), (3,1), (4,0), (4,1), (3,2),
            (4,2), (4,3), (3,3), (4,4), (3,4), (2,3), (2,4), (1,4), (1,3),
            (0,4), (0,3), (1,2), (0,2), (0,1), (1,1), (0,0), (1,0)];
        assert_eq!(clockwise_order, should_be);
    }

    #[test]
    fn test_clockwise_array3() {
        let input = "
#.#.#
.#.#.
#.#.#
.#.#.
#.#.#";
        let b = Board::from(&input);
        let clockwise_order = b.clockwise_ordering(2, 2);
        let should_be = vec![(2,0), (3,1), (4,0), 
            (4,2), (3,3), (4,4), (2,4), (1,3),
            (0,4), (0,2), (1,1), (0,0)];
        assert_eq!(clockwise_order, should_be);
    }

    #[test]
    fn test_day10b() {
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
        let mut b = Board::from(&input);
        assert_eq!(b.find_highest_detections(), (11, 13));

        let destruction_order = b.laser_from(11, 13);
        assert_eq!(destruction_order[0], (11,12));
        assert_eq!(destruction_order[1], (12,1));
        assert_eq!(destruction_order[2], (12,2));
        assert_eq!(destruction_order[9], (12,8));
        assert_eq!(destruction_order[19], (16,0));
        assert_eq!(destruction_order[49], (16,9));
        assert_eq!(destruction_order[99], (10,16));
        assert_eq!(destruction_order[198], (9,6));
        assert_eq!(destruction_order[199], (8,2));
        assert_eq!(destruction_order[200], (10,9));
        assert_eq!(destruction_order[298], (11,1));
    }
}