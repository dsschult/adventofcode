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

    fn find_mirror(&self) -> Vec<(Mirror, usize)> {
        let mut ret = Vec::new();
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
            if mirror {
                println!("mirror? {} {} {}", i, top_index, bottom_index);
                ret.push((Mirror::Horizontal, i));
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
            if mirror {
                println!("mirror? {} {} {}", i, left_index, right_index);
                ret.push((Mirror::Vertical, i))
            }
        }

        ret
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
            for r in m.find_mirror() {
                ret += match r {
                    (Mirror::Vertical, x) => x,
                    (Mirror::Horizontal, x) => 100*x,
                }
            }
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
        assert_eq!(patterns.list[0].find_mirror(), vec![(Mirror::Vertical, 5)]);
        assert_eq!(patterns.list[1].find_mirror(), vec![(Mirror::Horizontal, 4)]);
        assert_eq!(patterns.mirror_calc(), 405);
    }
}
