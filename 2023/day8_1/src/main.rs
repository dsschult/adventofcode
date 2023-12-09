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
struct Map<'a> {
    steps: &'a str,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl Map<'_> {
    fn new(lines: &Vec<String>) -> Map {
        let steps = lines[0].trim();
        let mut nodes = HashMap::new();
        for line in lines[2..].iter() {
            let parts = line.split('=').collect::<Vec<_>>();
            let parts2 = parts[1].trim()[1..parts[1].len()-2].split(", ").collect::<Vec<_>>();
            nodes.insert(parts[0].trim(), (parts2[0], parts2[1]));
        }
        Map{ steps: steps, nodes: nodes }
    }

    fn solve(&self) -> Vec<&str> {
        let mut ret = vec!["AAA"];
        let mut cur = "AAA";
        let steps = self.steps.chars().collect::<Vec<_>>();
        let mut step_index = 0;
        while cur != "ZZZ" {
            if !self.nodes.contains_key(cur) {
                panic!("bad node {}", cur);
            }
            cur = match steps[step_index] {
                'L' => self.nodes[cur].0,
                'R' => self.nodes[cur].1,
                _ => panic!("step is not L or R"),
            };
            ret.push(cur);
            step_index += 1;
            if step_index >= steps.len() {
                step_index = 0;
            }
        }
        ret
    }
}


fn main() {
    let lines = read_lines("input");

    let map = Map::new(&lines);
    //println!("map: {:?}", map);

    let solution = map.solve();
    println!("steps: {}", solution.len()-1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
".lines().map(String::from).collect();


        let map = Map::new(&sample);
        println!("map: {:?}", map);

        let solution = map.solve();
        assert_eq!(solution, vec!["AAA", "CCC", "ZZZ"]);
        assert_eq!(solution.len()-1, 2);
    }

    #[test]
    fn test_sample2() {
        let sample: Vec<String> = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
".lines().map(String::from).collect();


        let map = Map::new(&sample);
        println!("map: {:?}", map);

        let solution = map.solve();
        assert_eq!(solution, vec!["AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "ZZZ"]);
        assert_eq!(solution.len()-1, 6);
    }
}
