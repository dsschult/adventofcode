use std::fs::read_to_string;
use itertools::Itertools;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone)]
struct Rule {
    page1: i32,
    page2: i32
}

impl Rule {
    fn active(&self, update: &Vec<i32>) -> bool {
        update.contains(&self.page1) && update.contains(&self.page2)
    }

    fn passes(&self, update: &Vec<i32>) -> bool {
        let i1 = update.iter().position(|&x| x == self.page1);
        let i2 = update.iter().position(|&x| x == self.page2);
        match (i1, i2) {
            (Some(x), Some(y)) => x < y,
            _ => true
        }
    }
}

#[derive(Debug, Clone)]
struct Updates {
    data: Vec<Vec<i32>>
}

impl Updates {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn middles_added(&self) -> i32 {
        self.data.iter().fold(0, |v, u| {
            let mid = match u.len() % 2 {
                0 => u.len()/2-1,
                1 => u.len()/2,
                _ => panic!("bad mid")
            };
            println!("mid: {} {}", mid, u[mid]);
            v + u[mid]
        })
    }
}

#[derive(Debug, Clone)]
struct Manual {
    rules: Vec<Rule>,
    updates: Updates
}

impl Manual {
    fn new(lines: &Vec<String>) -> Manual {
        let mut rules = Vec::new();
        let mut now_updates = false;
        let mut updates = Vec::new();
        
        for line in lines.iter() {
            if line.trim().is_empty() {
                if rules.len() > 0 {
                    // rules have been processed, move to updates
                    now_updates = true;
                }
                continue;
            }
            if now_updates {
                let pages = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
                updates.push(pages);
            } else {
                let pages = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
                if pages.len() < 2 {
                    panic!("not enough pages in update {:?}", pages);
                }
                rules.push(Rule{ page1: pages[0], page2: pages[1] });
            }
        }
        Manual{ rules: rules, updates: Updates{ data: updates } }
    }

    fn correct_updates(&self) -> Updates {
        let mut ret = Vec::new();
        for row in self.updates.data.iter() {
            let passes = self.rules.iter().fold(true, |val, rule| {
                val & rule.passes(row)
            });
            if passes {
                ret.push(row.clone());
            }
        }
        Updates{ data: ret }
    }

    fn incorrect_updates_reordered(&self) -> Updates {
        let mut ret = Vec::new();
        for row in self.updates.data.iter() {
            let passes = self.rules.iter().fold(true, |val, rule| {
                val & rule.passes(row)
            });
            if !passes {
                // reorder
                let mut new_row = Vec::new();
                let active_rules = self.rules.iter().filter(|x| x.active(row)).collect::<Vec<_>>();
                println!("num total rules: {}", self.rules.len());
                println!("num active rules: {}", active_rules.len());
                for page in row.iter() {
                    let mut passes = false;
                    for i in 0..new_row.len()+1 {
                        let mut new_new_row = new_row.clone();
                        println!("trying to insert page {} at location {}", page, i);
                        new_new_row.insert(i, *page);
                        passes = active_rules.iter().all(|rule| rule.passes(&new_new_row));
                        if passes {
                            new_row = new_new_row;
                            break;
                        }
                    }
                    if !passes {
                        panic!("count not insert page {}", page);
                    }
                }
                ret.push(new_row);
                println!("reordered!");
            }
        }
        Updates{ data: ret }
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Manual::new(&lines);
    let u = c.incorrect_updates_reordered();
    println!("Middles: {}", u.middles_added());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
".lines().map(String::from).collect();

        let c = Manual::new(&sample);
        let u = c.correct_updates();
        assert_eq!(u.len(), 3);
        println!("correct: {:?}", u);
        assert_eq!(u.middles_added(), 143);

        let u2 = c.incorrect_updates_reordered();
        println!("correct2: {:?}", u2);
        assert_eq!(u2.len(), 3);
        assert_eq!(u2.middles_added(), 123);
    }
}