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
    galaxies: Vec<Pos>
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

    fn expand(&mut self) -> () {
        let max_rows = self.galaxies.iter().fold(0, |a,x| if x.0 > a {x.0} else {a});
        let max_cols = self.galaxies.iter().fold(0, |a,x| if x.1 > a {x.1} else {a});
        let mut empty_rows = (0..max_rows).collect::<HashSet<_>>();
        let mut empty_cols = (0..max_cols).collect::<HashSet<_>>();
        for (r,c) in self.galaxies.iter() {
            empty_rows.remove(&r);
            empty_cols.remove(&c);
        }
        let mut empty_rows2 = empty_rows.into_iter().collect::<Vec<_>>();
        empty_rows2.sort();
        for row in empty_rows2.into_iter().rev() {
            println!("expanding row {}", row);
            self.galaxies = self.galaxies.iter().map(|(r,c)| if *r > row { (*r+1, *c) } else {(*r,*c)}).collect::<Vec<_>>();
        }
        let mut empty_cols2 = empty_cols.into_iter().collect::<Vec<_>>();
        empty_cols2.sort();
        for col in empty_cols2.into_iter().rev() {
            println!("expanding col {}", col);
            self.galaxies = self.galaxies.iter().map(|(r,c)| if *c > col { (*r, *c+1) } else {(*r,*c)}).collect::<Vec<_>>();
        }
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
    u.expand();
    println!("Steps: {}", u.min_dist_sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
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
        u.expand();

        assert_eq!(u.min_dist_sum(), 374);
        assert!(false);
    }
}
