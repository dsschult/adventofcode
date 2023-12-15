use std::fs::read_to_string;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Pos = (i64, i64);

struct Universe {
    galaxies: Vec<Pos>,
}

impl Universe {
    fn new(lines: &Vec<String>) -> Universe {
        let mut galaxies = Vec::new();
        for (i,row) in lines.iter().enumerate() {
            for (j,col) in row.chars().enumerate() {
                if col == '#' {
                    galaxies.push((i as i64, j as i64));
                }
            }
        }
        Universe{galaxies: galaxies}
    }

    fn expand(&mut self, factor: i64) -> () {
        let max_rows = self.galaxies.iter().fold(0, |a,x| if x.0 > a {x.0} else {a});
        let max_cols = self.galaxies.iter().fold(0, |a,x| if x.1 > a {x.1} else {a});
        let mut empty_rows = (0..max_rows).collect::<HashSet<_>>();
        let mut empty_cols = (0..max_cols).collect::<HashSet<_>>();
        for (r,c) in self.galaxies.iter() {
            empty_rows.remove(&r);
            empty_cols.remove(&c);
        }
        println!("expanding rows: {:?}", empty_rows);
        println!("expanding cols: {:?}", empty_cols);

        let mut galaxies_new = Vec::new();
        for (r,c) in self.galaxies.iter() {
            let mut row_expansion = 0;
            for row in empty_rows.iter() {
                if *r > *row {
                    println!("expanding {},{} for row {}", r,c,row);
                    row_expansion += factor-1;
                }
            }
            let mut col_expansion = 0;
            for col in empty_cols.iter() {
                if *c > *col {
                    println!("expanding {},{} for col {}", r,c,col);
                    col_expansion += factor-1;
                }
            }
            let new_pos = (*r+row_expansion, *c+col_expansion);
            if row_expansion > 0 || col_expansion > 0 {
                println!("expanded {},{} to {},{}", r,c,new_pos.0,new_pos.1);
            }
            galaxies_new.push(new_pos);
        }
        self.galaxies = galaxies_new;
    }

    fn min_dist_sum(&self) -> i64 {
        let mut ret = 0;
        for i in 0..self.galaxies.len()-1 {
            for j in i+1..self.galaxies.len() {
                let (r1,c1) = self.galaxies[i].clone();
                let (r2,c2) = self.galaxies[j];
                let steps = if c1 <= c2 {
                    r2-r1 + c2-c1
                } else {
                    r2-r1 + c1-c2
                };
                println!("between {} and {} is {} steps", i, j, steps);
                ret += steps;
            }
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let mut u = Universe::new(&lines);
    u.expand(1000000);
    println!("Steps: {}", u.min_dist_sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let sample: Vec<String> = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
".lines().map(String::from).collect();

        let mut u = Universe::new(&sample);
        u.expand(2);

        assert_eq!(u.min_dist_sum(), 374);
    }

    #[test]
    fn test_10() {
        let sample: Vec<String> = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
".lines().map(String::from).collect();

        let mut u = Universe::new(&sample);
        u.expand(10);
        assert_eq!(u.min_dist_sum(), 1030);

        let mut u3 = Universe::new(&sample);
        u3.expand(100);
        assert_eq!(u3.min_dist_sum(), 8410);
    }
}
