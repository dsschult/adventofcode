use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone)]
struct Puzzle {
    chars: Vec<Vec<char>>
}

fn build_xmas_regex(len: usize) -> String {
    let mut ret = Vec::new();
    let mut base1 = String::from("XMAS");
    let mut base2 = String::from("SAMX");
    let extension1 = "AMX";
    let extension2 = "MAS";

    ret.push(base1.clone());
    ret.push(base2.clone());

    for i in 0..(len/4) {
        match i%2 {
            0 => {
                base1 += extension1;
                base2 += extension2;
            },
            1 => {
                base1 += extension2;
                base2 += extension1;
            },
            _ => {}
        };
        ret.push(base1.clone());
        ret.push(base2.clone());
    }
    ret.reverse();

    let mut ret_str = String::from("(");
    ret_str += &ret.join(")|(");
    ret_str += ")";
    ret_str
}

impl Puzzle {
    fn new(lines: &Vec<String>) -> Puzzle {
        let mut ret = Vec::new();
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            ret.push(line.chars().collect::<Vec<char>>())
        }
        Puzzle{ chars: ret }
    }

    fn xmas(&self) -> usize {
        let mut ret = 0;
        let max_rows = self.chars.len();
        let max_cols = self.chars[0].len();
        let re_str = build_xmas_regex(max_rows);
        println!("re_str: {}", re_str);
        let re = Regex::new(re_str.as_str()).unwrap();
        println!("max_rows: {}, max_cols: {}", max_rows, max_cols);

        // horizontal
        for cline in self.chars.iter() {
            let line: String = cline.iter().collect();
            let cnt = re.captures_iter(line.as_str()).fold(0, |n, caps| {
                let c = caps.iter().next().unwrap().map(|m| m.as_str()).unwrap();
                c.len()/3 + n
            });
            println!("H Examining {}: {}", line, cnt);
            ret += cnt;
        }

        // vertical
        for i in 0..max_cols {
            let mut cline = Vec::new();
            for j in 0..max_rows {
                cline.push(self.chars[j][i]);
            }
            let line: String = cline.iter().collect();
            let cnt = re.captures_iter(line.as_str()).fold(0, |n, caps| {
                let c = caps.iter().next().unwrap().map(|m| m.as_str()).unwrap();
                c.len()/3 + n
            });
            println!("V Examining {}: {}: {}", i, line, cnt);
            ret += cnt;
        }

        // diagonal right
        for j in 0..max_cols-3 {
            let mut cline = Vec::new();
            for k in 0..(max_cols+max_rows) {
                if k >= max_rows || j+k >= max_cols {
                    break;
                }
                cline.push(self.chars[k][j+k]);
            }
            let line: String = cline.iter().collect();
            let cnt = re.captures_iter(line.as_str()).fold(0, |n, caps| {
                let c = caps.iter().next().unwrap().map(|m| m.as_str()).unwrap();
                c.len()/3 + n
            });
            println!("D1 Examining {} {}: {}: {}", 0, j, line, cnt);
            ret += cnt;
        }
        for i in 1..max_rows-3 {
            let mut cline = Vec::new();
            for k in 0..(max_cols+max_rows) {
                if i+k >= max_rows || k >= max_cols {
                    break;
                }
                cline.push(self.chars[i+k][k]);
            }
            let line: String = cline.iter().collect();
            let cnt = re.captures_iter(line.as_str()).fold(0, |n, caps| {
                let c = caps.iter().next().unwrap().map(|m| m.as_str()).unwrap();
                c.len()/3 + n
            });
            println!("D1 Examining {} {}: {}: {}", i, 0, line, cnt);
            ret += cnt;
        }

        // diagonal left
        for i in 3..max_rows {
            let mut cline = Vec::new();
            for k in 0..(max_cols+max_rows) {
                if k > i || k >= max_cols {
                    break;
                }
                cline.push(self.chars[i-k][k]);
            }
            let line: String = cline.iter().collect();
            let cnt = re.captures_iter(line.as_str()).fold(0, |n, caps| {
                let c = caps.iter().next().unwrap().map(|m| m.as_str()).unwrap();
                c.len()/3 + n
            });
            println!("D2 Examining {} {}: {}: {}", i, 0, line, cnt);
            ret += cnt;
        }
        for j in 1..max_cols-3 {
            let i = max_rows-1;
            let mut cline = Vec::new();
            for k in 0..(max_cols+max_rows) {
                if k > i || j+k >= max_cols {
                    break;
                }
                cline.push(self.chars[i-k][j+k]);
            }
            let line: String = cline.iter().collect();
            let cnt = re.captures_iter(line.as_str()).fold(0, |n, caps| {
                let c = caps.iter().next().unwrap().map(|m| m.as_str()).unwrap();
                c.len()/3 + n
            });
            println!("D2 Examining {} {}: {}: {}", i, j, line, cnt);
            ret += cnt;
        }

        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Puzzle::new(&lines);
    println!("Matches: {}", c.xmas());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        let sample: Vec<String> = "
..X...
.SAMX.
.A..A.
XMAS.S
.X....
".lines().map(String::from).collect();

        let c = Puzzle::new(&sample);
        assert_eq!(c.xmas(), 4);
    }

    #[test]
    fn test_6() {
        let sample: Vec<String> = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
".lines().map(String::from).collect();

        let c = Puzzle::new(&sample);
        assert_eq!(c.xmas(), 18);
    }

    #[test]
    fn test_7() {
        let sample: Vec<String> = "
....XXMASS
.SAMXMS..A
...S..A..M
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
".lines().map(String::from).collect();

        let c = Puzzle::new(&sample);
        assert_eq!(c.xmas(), 19);
    }
}
