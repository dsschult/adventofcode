use std::fs::read_to_string;
use std::collections::HashSet;
use trie_rs::{Trie, TrieBuilder, inc_search::Answer};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = i64;

#[derive(Debug, Clone)]
struct Branding {
    patterns: Trie<u8>,
    designs: Vec<String>,
}

impl Branding {
    fn new(lines: &Vec<String>) -> Branding {
        let mut patterns = TrieBuilder::new();
        let mut do_patterns = true;
        let mut designs = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            match do_patterns {
                true => {
                    for pat in trim_line.split(", ") {
                        patterns.push(pat);
                    }
                    do_patterns = false;
                },
                false => {
                    designs.push(line.clone());
                }
            };
        }
        
        Branding{
            patterns: patterns.build(),
            designs: designs,
        }
    }

    fn possible_designs(&self) -> usize {
        let mut ret = 0;
        for design in self.designs.iter() {
            let mut queue = vec![(0, Vec::new())];
            let mut hist = HashSet::new();
            while !queue.is_empty() {
                let (design_index, patterns) = queue.pop().unwrap();
                if hist.contains(&design_index) {
                    continue;
                }
                hist.insert(design_index);
                let design_remaining = design.get(design_index..).unwrap();
                if design_remaining.is_empty() {
                    println!("complete: {} = {:?}", design, patterns);
                    ret += 1;
                    break;
                } else {
                    println!("design: {}|{}", design.get(..design_index).unwrap(), design_remaining);
                    let mut search = self.patterns.inc_search();
                    for (i,c) in (1..design_remaining.len()+1).zip(design_remaining.chars()) {
                        //println!("investigating {} {}", i, c);
                        match search.query(&(c as u8)) {
                            None => { break; }
                            Some(Answer::Prefix) => { },
                            Some(Answer::Match) | Some(Answer::PrefixAndMatch) => {
                                let p = design_remaining[..i].to_string();
                                //println!("using pattern: {} for {}", p, design_remaining);
                                let mut patterns2 = patterns.clone();
                                patterns2.push(p);
                                queue.push((design_index + i, patterns2));
                            }
                        };
                    }
                }
            }
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let b = Branding::new(&lines);
    println!("sum: {}", b.possible_designs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        let sample: Vec<String> = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
".lines().map(String::from).collect();

        let b = Branding::new(&sample);
        assert_eq!(b.possible_designs(), 6);
    }
}