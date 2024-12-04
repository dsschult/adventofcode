use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>
}

impl Report {
    fn safe(&self) -> bool {
        let incr = self.levels.is_sorted();
        let decr = self.levels.is_sorted_by(|a,b| a >= b);

        if !(incr | decr) {
            return false;
        }

        self.levels.windows(2).fold(true, |r, a| {
            let diff = (a[0] - a[1]).abs();
            r & (diff >= 1 && diff <= 3)
        })
    }

    fn dampener(&self) -> bool {
        println!("orig levels {:?}", self.levels);
        if self.safe() {
            println!("  orig safe!");
            return true
        }
        for i in 0..self.levels.len() {
            let (a,b) = self.levels.split_at(i);
            match b.split_first() {
                None => { },
                Some((_,c)) => {
                    let mut d = Vec::new();
                    d.extend(a);
                    d.extend(c);
                    print!("  levels {:?}", d);
                    if (Report{levels: d}).safe() {
                        println!(" safe!");
                        return true
                    }
                    println!(" unsafe!");
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Reports {
    report: Vec<Report>,
}

impl Reports {
    fn new(lines: &Vec<String>) -> Reports {
        let mut r = Vec::new();
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            r.push(Report{levels: line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()});
        }
        Reports{report: r}
    }

    fn num_safe(&self) -> i32 {
        self.report.iter().fold(0, |r, a| {
            let safe = a.dampener();
            match safe {
                true => r + 1,
                false => r
            }
        })
    }
}

fn main() {
    let lines = read_lines("input");
    let ids = Reports::new(&lines);
    println!("Num Safe: {}", ids.num_safe());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        let sample: Vec<String> = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
4 3 6 7 9
1 3 6 7 6
".lines().map(String::from).collect();

        let ids = Reports::new(&sample);
        assert_eq!(ids.num_safe(), 6);
    }
}
