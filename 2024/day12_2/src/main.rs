use std::fs::read_to_string;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = u64;

type Pos = (usize, usize);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Side {
    Left,
    Right,
    Top,
    Bottom
}

type SidePos = (Pos, Side);

#[derive(Debug, Clone)]
struct Region {
    letter: char,
    area: Num,
    sides: Num
}

#[derive(Debug, Clone)]
struct Plots {
    map: Vec<Vec<char>>,
    letters: HashSet<char>,
}

impl Plots {
    fn new(lines: &Vec<String>) -> Plots {
        let mut ret = Vec::new();
        let mut letters = HashSet::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            letters.extend(trim_line.chars());
            ret.push(trim_line.chars().collect::<Vec<_>>());
        }
        Plots{ map: ret, letters: letters }
    }

    fn find_locations(&self, letter: char) -> HashSet<Pos> {
        let mut ret = HashSet::new();
        let max_rows = self.map.len();
        let max_cols = self.map[0].len();
        for i in 0..max_rows {
            for j in 0..max_cols {
                if self.map[i][j] == letter {
                    ret.insert((i,j));
                }
            }
        }
        ret
    }

    fn calc_regions(&self) -> Vec<Region> {
        let mut ret = Vec::new();
        let max_rows = self.map.len();
        let max_cols = self.map[0].len();
        for letter in self.letters.iter() {
            let mut locations = self.find_locations(*letter);
            while !locations.is_empty() {
                let mut area = 0;
                let mut region_pos: Vec<Pos> = vec![*locations.iter().next().unwrap()];
                let mut side_set = HashSet::new();

                print!("processing letter {} region ", letter);
                while !region_pos.is_empty() {
                    let pos = region_pos.pop().unwrap();
                    if locations.remove(&pos) == false {
                        print!("av{:?} ", pos);
                        continue;
                    }
                    area += 1;
                    print!("{:?} ", pos);

                    // check top
                    match pos {
                        (0, _) => {
                            side_set.insert((pos, Side::Top));
                        },
                        (i, j) => {
                            let new_pos = (i-1,j);
                            if self.map[i-1][j] != *letter {
                                side_set.insert((pos, Side::Top));
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }

                    // check bottom
                    match pos {
                        (i, _) if i == max_rows-1 => {
                            side_set.insert((pos, Side::Bottom));
                        },
                        (i, j) => {
                            let new_pos = (i+1,j);
                            if self.map[i+1][j] != *letter {
                                side_set.insert((pos, Side::Bottom));
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }

                    // check left
                    match pos {
                        (_, 0) => {
                            side_set.insert((pos, Side::Left));
                        },
                        (i, j) => {
                            let new_pos = (i,j-1);
                            if self.map[i][j-1] != *letter {
                                side_set.insert((pos, Side::Left));
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }

                    // check right
                    match pos {
                        (_, j) if j == max_cols-1 => {
                            side_set.insert((pos, Side::Right));
                        },
                        (i, j) => {
                            let new_pos = (i,j+1);
                            if self.map[i][j+1] != *letter {
                                side_set.insert((pos, Side::Right));
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }
                }

                let mut sides = 0;
                while !side_set.is_empty() {
                    let mut queue = vec![side_set.iter().next().unwrap().clone()];
                    sides += 1;

                    // traverse side
                    while !queue.is_empty() {
                        let s = queue.pop().unwrap();
                        side_set.remove(&s);
                        match s.1 {
                            e if e == Side::Top || e == Side::Bottom => {
                                match s.0.1 {
                                    0 => { // go right
                                        let s2 = ((s.0.0, 1), e.clone());
                                        if side_set.contains(&s2) {
                                            queue.push(s2);
                                        }
                                    },
                                    j if j == max_cols-1 => { // go left
                                        let s2 = ((s.0.0, max_cols-2), e.clone());
                                        if side_set.contains(&s2) {
                                            queue.push(s2);
                                        }
                                    },
                                    j => { // go both right and left
                                        let s2 = ((s.0.0, j-1), e.clone());
                                        if side_set.contains(&s2) {
                                            queue.push(s2);
                                        }
                                        let s3 = ((s.0.0, j+1), e.clone());
                                        if side_set.contains(&s3) {
                                            queue.push(s3);
                                        }
                                    }
                                };
                            },
                            e if e == Side::Left || e == Side::Right => {
                                match s.0.0 {
                                    0 => { // go down
                                        let s2 = ((1, s.0.1), e.clone());
                                        if side_set.contains(&s2) {
                                            queue.push(s2);
                                        }
                                    },
                                    i if i == max_rows-1 => { // go up
                                        let s2 = ((max_rows-2, s.0.1), e.clone());
                                        if side_set.contains(&s2) {
                                            queue.push(s2);
                                        }
                                    },
                                    i => { // go both up and down
                                        let s2 = ((i-1, s.0.1), e.clone());
                                        if side_set.contains(&s2) {
                                            queue.push(s2);
                                        }
                                        let s3 = ((i+1, s.0.1), e.clone());
                                        if side_set.contains(&s3) {
                                            queue.push(s3);
                                        }
                                    }
                                };
                            },
                            _ => { }
                        }
                    }
                }
                println!(" complete\n  area: {}, sides: {}", area, sides);
                ret.push(Region{letter: *letter, area: area, sides: sides});
            }
        }
        ret
    }

    fn fence_price(&self) -> Num {
        let mut ret = 0;
        for region in self.calc_regions().iter() {
            ret += region.area * region.sides;
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let plots = Plots::new(&lines);
    println!("fence price: {}", plots.fence_price());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
AAAA
BBCD
BBCC
EEEC
".lines().map(String::from).collect();

        let plots = Plots::new(&sample);

        assert_eq!(plots.letters.len(), 5);

        let regions = plots.calc_regions();
        assert_eq!(regions.len(), 5);
        println!("{:?}", regions);

        assert_eq!(plots.fence_price(), 80);
    }

    #[test]
    fn test_2() {
        let sample: Vec<String> = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
".lines().map(String::from).collect();

        let plots = Plots::new(&sample);

        assert_eq!(plots.letters.len(), 2);

        let regions = plots.calc_regions();
        assert_eq!(regions.len(), 5);
        println!("{:?}", regions);

        assert_eq!(plots.fence_price(), 436);
    }

    #[test]
    fn test_2a() {
        let sample: Vec<String> = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
".lines().map(String::from).collect();

        let plots = Plots::new(&sample);

        assert_eq!(plots.letters.len(), 2);

        let regions = plots.calc_regions();
        assert_eq!(regions.len(), 3);
        println!("{:?}", regions);

        assert_eq!(plots.fence_price(), 236);
    }

    #[test]
    fn test_2b() {
        let sample: Vec<String> = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
".lines().map(String::from).collect();

        let plots = Plots::new(&sample);

        assert_eq!(plots.letters.len(), 2);

        let regions = plots.calc_regions();
        assert_eq!(regions.len(), 3);
        println!("{:?}", regions);

        assert_eq!(plots.fence_price(), 368);
    }

    #[test]
    fn test_3() {
        let sample: Vec<String> = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
".lines().map(String::from).collect();

        let plots = Plots::new(&sample);

        assert_eq!(plots.letters.len(), 9);

        let regions = plots.calc_regions();
        assert_eq!(regions.len(), 11);
        println!("{:?}", regions);

        assert_eq!(plots.fence_price(), 1206);
    }
}