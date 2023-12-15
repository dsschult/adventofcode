use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, PartialEq)]
enum Mirror {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<Vec<char>>
}

impl Pattern {
    fn new(lines: &[String]) -> Pattern {
        Pattern{rows: lines.iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>()}
    }

    fn get_col(&self, i: usize) -> String {
        let mut ret = String::new();
        for row in self.rows.iter() {
            for (a,c) in row.iter().enumerate() {
                if a == i {
                    ret.push(*c);
                }
            }
        }
        ret
    }

    fn find_mirror(&self, ignore: &Option<(Mirror, usize)>) -> Option<(Mirror, usize)> {
        // search horizontal first
        let row_len = self.rows.len();
        let col_len = self.rows[0].len();
        for i in 1..row_len {
            let mut top_index = i-1;
            let mut bottom_index = i;
            let mut mirror = true;
            while bottom_index < row_len {
                if self.rows[top_index] != self.rows[bottom_index] {
                    mirror = false;
                    break;
                }
                if top_index == 0 {
                    break;
                }
                top_index -= 1;
                bottom_index += 1;
            }
            if mirror && *ignore != Some((Mirror::Horizontal, i)) {
                println!("mirror? {} {} {}", i, top_index, bottom_index);
                return Some((Mirror::Horizontal, i));
            }
        }

        // now search vertical
        for i in 1..col_len {
            let mut left_index = i-1;
            let mut right_index = i;
            let mut mirror = true;
            while right_index < col_len {
                if self.get_col(left_index) != self.get_col(right_index) {
                    mirror = false;
                    break;
                }
                if left_index == 0 {
                    break;
                }
                left_index -= 1;
                right_index += 1;
            }
            if mirror && *ignore != Some((Mirror::Vertical, i)) {
                println!("mirror? {} {} {}", i, left_index, right_index);
                return Some((Mirror::Vertical, i));
            }
        }

        None
    }

    fn find_smudge(&self) -> Option<(Mirror, usize)> {
        let old_mirror = self.find_mirror(&None);
        let mut cp = self.clone();
        for (i,row) in self.rows.iter().enumerate() {
            for (j,c) in row.iter().enumerate() {
                cp.rows[i][j] = match c {
                    '#' => '.',
                    '.' => '#',
                    _ => panic!("bad char"),
                };
                let new_mirror = cp.find_mirror(&old_mirror);
                if new_mirror != None {
                    println!("found smudge on {},{}", i,j);
                    return new_mirror;
                }
                cp.rows[i][j] = *c;
            }
        }
        println!("pattern:");
        for row in self.rows.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
        panic!("no smudge found!")
    }
}

#[derive(Debug, Clone)]
struct Patterns {
    list: Vec<Pattern>
}

impl Patterns {
    fn new(lines: &Vec<String>) -> Patterns {
        let mut start = 0;
        let mut ret = Vec::new();
        for (i,line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                ret.push(Pattern::new(&lines[start..i]));
                start = i+1;
            }
        }
        if start < lines.len() {
            ret.push(Pattern::new(&lines[start..]));
        }
        Patterns{list: ret}
    }

    fn mirror_calc(&self) -> usize {
        let mut ret = 0;
        for m in self.list.iter() {
            ret += match m.find_smudge() {
                None => panic!("no mirror!"),
                Some((Mirror::Vertical, x)) => x,
                Some((Mirror::Horizontal, x)) => 100*x,
            };
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    
    let patterns = Patterns::new(&lines);
    println!("pat: {}", patterns.mirror_calc());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let sample: Vec<String> = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
".lines().map(String::from).collect();

        let patterns = Patterns::new(&sample);
        assert_eq!(patterns.list.len(), 2);

        println!("patterns: {:?}", patterns);
        assert_eq!(patterns.list[0].find_mirror(&None), Some((Mirror::Vertical, 5)));
        assert_eq!(patterns.list[0].find_smudge(),  Some((Mirror::Horizontal, 3)));

        assert_eq!(patterns.list[1].find_mirror(&None),  Some((Mirror::Horizontal, 4)));
        assert_eq!(patterns.list[1].find_smudge(),  Some((Mirror::Horizontal, 1)));

        assert_eq!(patterns.mirror_calc(), 400);
    }
}
