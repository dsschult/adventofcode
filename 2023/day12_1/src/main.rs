use std::fs::read_to_string;
use std::collections::VecDeque;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug, Clone)]
struct ConditionRow {
    springs: Vec<char>,
    damage_groups: Vec<i64>,
}

impl ConditionRow {
    fn new(line: &str) -> ConditionRow {
        println!("line: {}", line);
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let dmg = parts[1].split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        ConditionRow{springs: parts[0].chars().collect(), damage_groups: dmg}
    }

    fn valid_row(&self) -> Option<bool> {
        let mut dmg = self.damage_groups.iter().collect::<VecDeque<_>>();
        let mut cnt = 0;
        let mut start_group = false;
        for c in self.springs.iter() {
            match *c {
                '#' => {
                    if start_group { cnt += 1; }
                    else { start_group = true; cnt = 1; }
                },
                '.' => if start_group {
                    match dmg.pop_front() {
                        None => {
                            println!("no more dmg to pop");
                            return Some(false);
                        },
                        Some(d) => if *d != cnt {
                            println!("dmg count does not match: {} {}", d, cnt);
                            return Some(false);
                        },
                    };
                    start_group = false;
                    cnt = 0;
                },
                '?' => {
                    return None
                }
                _ => {
                    println!("bad char in springs");
                    return Some(false);
                },
            }
        }
        if start_group {
            match dmg.pop_front() {
                None => {
                    println!("no more dmg to pop");
                    return Some(false);
                },
                Some(d) => if *d != cnt {
                    println!("dmg count does not match: {} {}", d, cnt);
                    return Some(false);
                },
            };
        }

        let e = dmg.is_empty();
        if !e {
            println!("still dmg remaining: {:?}", dmg);
        }
        Some(e)
    }

    fn arrangement_cnt(&self) -> i64 {
        let mut ret = 0;
        let positions = self.springs.iter().enumerate().filter(|(_,x)| **x == '?').map(|(a,_)| a).collect::<Vec<usize>>();
        let mut possibilities = vec![self.clone()];
        for i in positions.iter() {
            let mut new_poss = Vec::new();
            for mut p in possibilities.into_iter() {
                let mut cp = p.clone();
                p.springs[*i] = '#';
                match p.valid_row() {
                    Some(x) => if x {
                        // this is a fully valid arrangement
                        ret += 1;
                    }, // else, this is an invalid arrangement, so drop
                    None => {
                        // this is ambiguous, continue
                        new_poss.push(p);
                    }
                }
                cp.springs[*i] = '.';
                match cp.valid_row() {
                    Some(x) => if x {
                        // this is a fully valid arrangement
                        ret += 1;
                    }, // else, this is an invalid arrangement, so drop
                    None => {
                        // this is ambiguous, continue
                        new_poss.push(cp);
                    }
                }
            }
            possibilities = new_poss;
        }
        ret
    }
}


#[derive(Debug, Clone)]
struct Conditions {
    rows: Vec<ConditionRow>
}

impl Conditions {
    fn new(lines: &Vec<String>) -> Conditions {
        Conditions{rows: lines.iter().filter(|x| !x.trim().is_empty()).map(|x| ConditionRow::new(x)).collect()}
    }

    fn valid(&self) -> bool {
        for row in self.rows.iter() {
            match row.valid_row() {
                Some(x) => if !x {
                    println!("row invalid: {:?}", row);
                    return false;
                },
                None => { return false; }
            }
        }
        true
    }

    fn arrangement_cnt_sum(&self) -> i64 {
        self.rows.iter().fold(0, |a,x| a+x.arrangement_cnt())
    }
}


fn main() {
    let lines = read_lines("input");
    let cond = Conditions::new(&lines);
    println!("arrangement sum: {}", cond.arrangement_cnt_sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let sample: Vec<String> = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1
".lines().map(String::from).collect();

        let cond = Conditions::new(&sample);
        assert_eq!(cond.valid(), true);
    }

    #[test]
    fn test_question() {
        let sample: Vec<String> = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
".lines().map(String::from).collect();

        let cond = Conditions::new(&sample);
        assert_eq!(cond.valid(), false);

        assert_eq!(cond.rows[0].arrangement_cnt(), 1);
        assert_eq!(cond.rows[1].arrangement_cnt(), 4);
        assert_eq!(cond.rows[2].arrangement_cnt(), 1);
        assert_eq!(cond.rows[3].arrangement_cnt(), 1);
        assert_eq!(cond.rows[4].arrangement_cnt(), 4);
        assert_eq!(cond.rows[5].arrangement_cnt(), 10);

        assert_eq!(cond.arrangement_cnt_sum(), 21);
    }
}
 