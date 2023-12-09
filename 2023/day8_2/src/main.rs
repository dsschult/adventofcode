use std::fs::read_to_string;
use std::collections::HashMap;
use num::integer::lcm;
use num::integer::Integer;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug)]
struct Map<'a> {
    steps: &'a str,
    starting_nodes: Vec<&'a str>,
    ending_nodes: Vec<&'a str>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl Map<'_> {
    fn new(lines: &Vec<String>) -> Map {
        let steps = lines[0].trim();
        let mut starts = Vec::new();
        let mut ends = Vec::new();
        let mut nodes = HashMap::new();
        for line in lines[2..].iter() {
            let parts = line.split('=').collect::<Vec<_>>();
            let key = parts[0].trim();
            if key.ends_with('A') {
                starts.push(key);
            } else if key.ends_with('Z') {
                ends.push(key);
            }
            let parts2 = parts[1].trim()[1..parts[1].len()-2].split(", ").collect::<Vec<_>>();
            nodes.insert(key, (parts2[0], parts2[1]));
        }
        Map{ steps: steps, starting_nodes: starts, ending_nodes: ends, nodes: nodes }
    }

    fn solve(&self) -> u64 {
        let mut ret = 0;
        let mut cur = self.starting_nodes.to_vec();
        //for key in cur.iter() {
        //    ret.push(vec![*key]);
        //}
        let steps = self.steps.chars().collect::<Vec<_>>();
        let mut step_index = 0;
        while cur.iter().filter(|x| !x.ends_with('Z')).count() > 0 {
            if cur.iter().filter(|x| x.ends_with('Z')).count() > 2 {
                println!("step {} cur {:?}", ret, cur);
            }
            cur = cur.iter().map(|key| {
                match self.nodes.get(key) {
                    Some((left,right)) => match steps[step_index] {
                        'L' => *left,
                        'R' => *right,
                        _ => panic!("step is not L or R"),
                    },
                    None => panic!("bad node {}", key),
                }
            }).collect::<Vec<_>>();
            cur.sort();
            cur.dedup();
            ret += 1;
            step_index += 1;
            if step_index >= steps.len() {
                step_index = 0;
            }
        }
        ret
    }
    
    fn solve2(&self, start: &str) -> Vec<u64> {
        // find all cycles to ending nodes
        let mut ret = Vec::new();
        let steps = self.steps.chars().collect::<Vec<_>>();
        for end in self.ending_nodes.iter() {
            let mut cur = start;
            let mut step_index = 0;
            let mut count = 0;
            while cur != *end && count < 10000000 {
                cur = match self.nodes.get(cur) {
                    Some((left,right)) => match steps[step_index] {
                        'L' => *left,
                        'R' => *right,
                        _ => panic!("step is not L or R"),
                    },
                    None => panic!("bad node {}", cur),
                };
                count += 1;
                step_index += 1;
                if step_index >= steps.len() {
                    step_index = 0;
                }
            }
            if cur == *end {
                ret.push(count);
            }
        }
        ret
    }
    fn solve_cycles(&self) -> u64 {
        let mut all_cycles = Vec::new();
        for s in self.starting_nodes.iter() {
            all_cycles.append(&mut self.solve2(s));
        }
        all_cycles.sort();
        all_cycles.dedup();
        println!("cycle lengths: {:?}", all_cycles);
        let mut ret = all_cycles[0];
        for a in all_cycles.iter() {
            println!("lcm cur: {}", ret);
            ret = ret.lcm(a);
        }
        ret
    }
}


fn main() {
    let lines = read_lines("input");

    let map = Map::new(&lines);
    //println!("map: {:?}", map);

    let solution = map.solve_cycles();
    println!("steps: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
".lines().map(String::from).collect();


        let map = Map::new(&sample);
        println!("map: {:?}", map);

        let solution = map.solve();
        println!("sol: {:?}", solution);
        //assert_eq!(solution, vec!["AAA", "CCC", "ZZZ"]);
        assert_eq!(solution, 6);

        let sol2 = map.solve_cycles();
        assert_eq!(sol2, 6);
    }
}
