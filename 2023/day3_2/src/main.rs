use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug)]
struct Matrix {
    rows: Vec<Vec<char>>
}

impl Matrix {
    fn new(lines: &Vec<String>) -> Matrix {
        let mut ret = Matrix{ rows: Vec::new() };
        for line in lines.iter() {
            if line.trim() != "" {
                let row: Vec<_> = line.chars().collect();
                ret.rows.push(row);
            }
        }
        ret
    }

    fn at(&self, x: &i32, y: &i32) -> char {
        self.rows[*x as usize][*y as usize]
    }

    fn get_symbol_positions(&self) -> Vec<(i32,i32)> {
        let mut ret = Vec::new();
        for (i,row) in self.rows.iter().enumerate() {
            for (j,c) in row.iter().enumerate() {
                if !(c.is_numeric() || *c == '.') {
                    ret.push((i as i32, j as i32));
                }
            }
        }
        ret
    }

    fn get_numbers_and_bounding(&self) -> Vec<(u32, (i32,i32), (i32,i32))> {
        // output is number, upper right bounding, lower left bounding
        let mut ret = Vec::new();
        for (i,row) in self.rows.iter().enumerate() {
            let mut start: i32 = -1;
            for (j,c) in row.iter().enumerate() {
                if c.is_numeric() {
                    if start == -1 {
                        start = j as i32;
                    }
                } else if start != -1 {
                    let num = row[start as usize..j].iter().collect::<String>().parse::<u32>().unwrap();
                    ret.push((num, (i as i32-1, start-1), (i as i32+1, j as i32)));
                    start = -1;
                }
            }
            if start != -1 {
                let j = row.len();
                let num = row[start as usize..j].iter().collect::<String>().parse::<u32>().unwrap();
                ret.push((num, (i as i32-1, start-1), (i as i32+1, j as i32)));
            }
        }
        ret
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        let mut ret = Vec::new();
        let nums = self.get_numbers_and_bounding();
        let symbols = self.get_symbol_positions();

        for (n, (i,j), (k,m)) in nums.iter() {
            for (r,c) in symbols.iter() {
                if i <= r && k >= r && j <= c && m >= c {
                    ret.push(*n);
                    break;
                }
            }
        }
        ret
    }

    fn get_gear_ratios(&self) -> Vec<u32> {
        let nums = self.get_numbers_and_bounding();
        let symbols = self.get_symbol_positions();

        let mut gears: HashMap<(i32,i32), Vec<u32>> = HashMap::new();
        
        for (n, (i,j), (k,m)) in nums.iter() {
            for (r,c) in symbols.iter() {
                if i <= r && k >= r && j <= c && m >= c {
                    // valid part
                    if self.at(r,c) == '*' {
                        gears.entry((*r,*c)).and_modify(|x| x.push(*n)).or_insert(vec![*n]);
                    }
                    break;
                }
            }
        }

        let mut ret = Vec::new();
        for (_, vals) in gears {
            if vals.len() == 2 {
                ret.push(vals[0] * vals[1]);
            }
        }
        ret
    }
}



fn main() {
    let lines = read_lines("input");
    let m = Matrix::new(&lines);
    let part_nums = m.get_part_numbers();
    let sum_parts = part_nums.into_iter().reduce(|a,b| a+b).unwrap();
    
    println!("part sum: {}", sum_parts);
    
    let gears = m.get_gear_ratios();

    let sum_gears = gears.into_iter().reduce(|a,b| a+b).unwrap();
    println!("gear sum: {}", sum_gears);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
".lines().map(String::from).collect();

        let m = Matrix::new(&sample);

        assert_eq!(m.at(&0,&0), '4');
        assert_eq!(m.at(&1,&3), '*');

        let symbols = m.get_symbol_positions();
        assert_eq!(symbols.len(), 6);
        assert_eq!(symbols[0], (1,3));

        let nums = m.get_numbers_and_bounding();
        assert_eq!(nums.len(), 10);
        assert_eq!(nums[0], (467, (-1,-1), (1,3)));
        assert_eq!(nums[1], (114, (-1,4), (1,8)));

        let part_nums = m.get_part_numbers();
        assert_eq!(part_nums.len(), 8);

        let sum_parts = part_nums.into_iter().reduce(|a,b| a+b).unwrap();
        assert_eq!(sum_parts, 4361);

        let gears = m.get_gear_ratios();
        assert_eq!(gears.len(), 2);
        assert_eq!(gears[0], 16345);
        assert_eq!(gears[1], 451490);

        let sum_gears = gears.into_iter().reduce(|a,b| a+b).unwrap();
        assert_eq!(sum_gears, 467835);
        
    }
}