use std::fs::read_to_string;

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

fn match_xmas(piece: Vec<Vec<&char>>) -> bool {
    *piece[1][1] == 'A' && (
    *piece[0][0] == 'M' && *piece[2][2] == 'S' ||
    *piece[0][0] == 'S' && *piece[2][2] == 'M') && (
    *piece[2][0] == 'M' && *piece[0][2] == 'S' ||
    *piece[2][0] == 'S' && *piece[0][2] == 'M')
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
        println!("max_rows: {}, max_cols: {}", max_rows, max_cols);

        for i in 0..max_rows-2 {
            for j in 0..max_cols-2 {
                let mut piece = Vec::new();
                piece.push(self.chars[i][j..j+3].iter().collect::<Vec<_>>());
                piece.push(self.chars[i+1][j..j+3].iter().collect::<Vec<_>>());
                piece.push(self.chars[i+2][j..j+3].iter().collect::<Vec<_>>());
                println!("piece: {:?}", piece);
                if match_xmas(piece) {
                    ret += 1;
                }
            }
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
M.S
.A.
M.S
".lines().map(String::from).collect();

        let c = Puzzle::new(&sample);
        assert_eq!(c.xmas(), 1);
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
        assert_eq!(c.xmas(), 9);
    }

    #[test]
    fn test_7() {
        let sample: Vec<String> = "
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
".lines().map(String::from).collect();

        let c = Puzzle::new(&sample);
        assert_eq!(c.xmas(), 9);
    }
}
