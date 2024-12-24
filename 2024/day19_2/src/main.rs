use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;
use trie_rs::{Trie, TrieBuilder, inc_search::Answer};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = u16;

#[derive(Debug, Clone)]
struct Branding {
    pattern_index: HashMap<String, Num>,
    patterns: Trie<u8>,
    designs: Vec<String>,
}

impl Branding {
    fn new(lines: &Vec<String>) -> Branding {
        let mut pattern_index = HashMap::new();
        let mut p = 0;
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
                        pattern_index.insert(pat.to_string(), p);
                        patterns.push(pat);
                        p += 1;
                    }
                    do_patterns = false;
                },
                false => {
                    designs.push(line.clone());
                }
            };
        }

        Branding{
            pattern_index: pattern_index,
            patterns: patterns.build(),
            designs: designs,
        }
    }

    fn _possible_design(&self, design: &str) -> usize {
        let mut ans_sets: HashMap<&str, usize> = HashMap::new();
        let max_len = design.len();
        let mut i = max_len;
        for i in (0..max_len).rev() {
            let word = &design[i..];
            let mut num = 0;
            let mut search = self.patterns.inc_search();
            for (c, j) in word.chars().zip(1..max_len) {
                match search.query(&(c as u8)) {
                    None => { break; }
                    Some(Answer::Prefix) => { },
                    Some(Answer::Match) | Some(Answer::PrefixAndMatch) => {
                        let remain = &word[j..];
                        if remain.is_empty() {
                            num += 1;
                        } else {
                            //println!("   looking at {}", remain);
                            match ans_sets.get(&remain) {
                                None => {}, // remain can't form a design
                                Some(v) => {
                                    num += v;
                                }
                            };
                        }
                    }
                };
            }
            println!("   analyzed {} and found {} ways", word, num);
            if num > 0 {
                ans_sets.insert(word, num);
            }
        }
        let ret = match ans_sets.get(&design) {
            Some(v) => *v,
            None => 0,
        };
        println!("design can be made {} ways", ret);
        ret
    }

    fn possible_designs(&self) -> usize {
        let mut ret = 0;
        for design in self.designs.iter() {
            ret += self._possible_design(design);
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
        assert_eq!(b.possible_designs(), 16);
    }
}