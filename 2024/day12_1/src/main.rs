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

#[derive(Debug, Clone)]
struct Region {
    letter: char,
    area: Num,
    perimeter: Num
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
                let mut perimeter = 0;
                let mut region_pos: Vec<Pos> = vec![*locations.iter().next().unwrap()];

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
                            perimeter += 1;
                        },
                        (i, j) => {
                            let new_pos = (i-1,j);
                            if self.map[i-1][j] != *letter {
                                perimeter += 1;
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }

                    // check bottom
                    match pos {
                        (i, _) if i == max_rows-1 => {
                            perimeter += 1;
                        },
                        (i, j) => {
                            let new_pos = (i+1,j);
                            if self.map[i+1][j] != *letter {
                                perimeter += 1;
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }

                    // check left
                    match pos {
                        (_, 0) => {
                            perimeter += 1;
                        },
                        (i, j) => {
                            let new_pos = (i,j-1);
                            if self.map[i][j-1] != *letter {
                                perimeter += 1;
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }

                    // check right
                    match pos {
                        (_, j) if j == max_cols-1 => {
                            perimeter += 1;
                        },
                        (i, j) => {
                            let new_pos = (i,j+1);
                            if self.map[i][j+1] != *letter {
                                perimeter += 1;
                            } else if locations.contains(&new_pos) {
                                region_pos.push(new_pos);
                            }
                            // else, visited already
                        }
                    }
                }
                println!(" complete\n  area: {}, perimeter: {}", area, perimeter);
                ret.push(Region{letter: *letter, area: area, perimeter: perimeter});
            }
        }
        ret
    }

    fn fence_price(&self) -> Num {
        let mut ret = 0;
        for region in self.calc_regions().iter() {
            ret += region.area * region.perimeter;
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

        assert_eq!(plots.fence_price(), 140);
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

        assert_eq!(plots.fence_price(), 772);
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

        assert_eq!(plots.fence_price(), 1930);
    }
}