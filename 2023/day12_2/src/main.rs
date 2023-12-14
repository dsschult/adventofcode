use std::fs::read_to_string;

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
    damage_groups: Vec<u8>,
}


fn valid_row(springs_slice: &[char], damage_slice: &[u8]) -> (Option<bool>, usize, usize) {
    let mut dmg = damage_slice.iter();
    let mut cnt = 0;
    let mut springs_pos = 0;
    let mut damage_pos = 0;
    for (i,c) in springs_slice.iter().enumerate() {
        match *c {
            '#' => {
                cnt += 1;
            },
            '.' => if cnt != 0 {
                match dmg.next() {
                    None => {
                        //println!("no more dmg to pop");
                        return (Some(false), 0, 0);
                    },
                    Some(d) => if *d != cnt {
                        //println!("dmg count does not match: {} {}", d, cnt);
                        return (Some(false), 0, 0);
                    },
                };
                cnt = 0;
                damage_pos += 1;
                springs_pos = i;
            },
            '?' => {
                return (None, springs_pos, damage_pos);
            }
            _ => {
                //println!("bad char in springs");
                return (Some(false), 0, 0);
            },
        }
    }
    if cnt != 0 {
        match dmg.next() {
            None => {
                //println!("no more dmg to pop");
                return (Some(false), 0, 0);
            },
            Some(d) => if *d != cnt {
                //println!("dmg count does not match: {} {}", d, cnt);
                return (Some(false), 0, 0);
            },
        };
    }

    let e = dmg.next() == None;
    //if !e {
        //println!("still dmg remaining: {:?}", dmg);
    //}
    (Some(e), 0, 0)
}

impl ConditionRow {
    fn new(line: &str) -> ConditionRow {
        //println!("line: {}", line);
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let dmg = parts[1].split(',').map(|x| x.parse::<u8>().unwrap()).collect();
        ConditionRow{springs: parts[0].chars().collect(), damage_groups: dmg}
    }

    fn unfold(&mut self) -> () {
        let mut s = self.springs.clone();
        let mut s2 = self.springs.clone();
        let mut s3 = self.springs.clone();
        let mut s4 = self.springs.clone();
        self.springs.push('?');
        self.springs.append(&mut s);
        self.springs.push('?');
        self.springs.append(&mut s2);
        self.springs.push('?');
        self.springs.append(&mut s3);
        self.springs.push('?');
        self.springs.append(&mut s4);
        
        let mut d = self.damage_groups.clone();
        let mut d2 = self.damage_groups.clone();
        let mut d3 = self.damage_groups.clone();
        let mut d4 = self.damage_groups.clone();
        self.damage_groups.append(&mut d);
        self.damage_groups.append(&mut d2);
        self.damage_groups.append(&mut d3);
        self.damage_groups.append(&mut d4);
    }

    fn valid_row(&self) -> Option<bool> {
        let mut dmg = self.damage_groups.iter();
        let mut cnt = 0;
        for c in self.springs.iter() {
            match *c {
                '#' => {
                    cnt += 1;
                },
                '.' => if cnt != 0 {
                    match dmg.next() {
                        None => {
                            //println!("no more dmg to pop");
                            return Some(false);
                        },
                        Some(d) => if *d != cnt {
                            //println!("dmg count does not match: {} {}", d, cnt);
                            return Some(false);
                        },
                    };
                    cnt = 0;
                },
                '?' => {
                    return None
                }
                _ => {
                    //println!("bad char in springs");
                    return Some(false);
                },
            }
        }
        if cnt != 0 {
            match dmg.next() {
                None => {
                    //println!("no more dmg to pop");
                    return Some(false);
                },
                Some(d) => if *d != cnt {
                    //println!("dmg count does not match: {} {}", d, cnt);
                    return Some(false);
                },
            };
        }

        let e = dmg.next() == None;
        if !e {
            //println!("still dmg remaining: {:?}", dmg);
        }
        Some(e)
    }

    fn arrangement_cnt_helper(&self, cp: &mut Self, positions: &Vec<usize>, pos: usize, springs_pos: usize, damage_pos: usize) -> u64 {
        let mut ret = 0;
        let i = positions[pos];
        cp.springs[i] = '.';
        match valid_row(&cp.springs[springs_pos..], &cp.damage_groups[damage_pos..]) {
            (Some(true),_,_) => {
                // this is a fully valid arrangement
                ret += 1;
            }, // else, this is an invalid arrangement, so drop
            (None,sp,dp) => {
                // this is ambiguous, continue
                ret += self.arrangement_cnt_helper(cp, positions, pos+1, springs_pos+sp, damage_pos+dp);
            },
            _ => { }
        }
        cp.springs[i..].copy_from_slice(&self.springs[i..]);
        cp.springs[i] = '#';
        match valid_row(&cp.springs[springs_pos..], &cp.damage_groups[damage_pos..]) {
            (Some(true),_,_) => {
                // this is a fully valid arrangement
                ret += 1;
            }, // else, this is an invalid arrangement, so drop
            (None,sp,dp) => {
                // this is ambiguous, continue
                ret += self.arrangement_cnt_helper(cp, positions, pos+1, springs_pos+sp, damage_pos+dp);
            },
            _ => { }
        }
        ret
    }

    fn arrangement_cnt(&self) -> u64 {
        let positions = self.springs.iter().enumerate().filter(|(_,x)| **x == '?').map(|(a,_)| a).collect::<Vec<usize>>();
        let mut cp = self.clone();
        self.arrangement_cnt_helper(&mut cp, &positions, 0, 0, 0)
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

    fn unfold(&mut self) -> () {
        for row in self.rows.iter_mut() {
            row.unfold();
        }
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

    fn arrangement_cnt_sum(&self) -> u64 {
        let mut sum = 0;
        for (i,c) in self.rows.iter().enumerate() {
            sum += c.arrangement_cnt();
            println!("cond {}. sum currently {}", i, sum);
        }
        sum
        //self.rows.iter().fold(0, |a,x| a+x.arrangement_cnt())
    }
}


fn main() {
    let lines = read_lines("input");
    let mut cond = Conditions::new(&lines);
    cond.unfold();
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

    #[test]
    fn test_question_unfold() {
        let sample: Vec<String> = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
".lines().map(String::from).collect();

        let mut cond = Conditions::new(&sample);
        cond.unfold();

        assert_eq!(cond.rows[0].springs.iter().collect::<String>(), "???.###????.###????.###????.###????.###".to_string());
        assert_eq!(cond.rows[0].damage_groups, vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]);
        
        assert_eq!(cond.arrangement_cnt_sum(), 525152);
    }
}
 