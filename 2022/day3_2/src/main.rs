use std::fs::read_to_string;
use counter::Counter;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug)]
struct Rucksack<'a> {
    compartment1: &'a str,
    compartment2: &'a str
}

impl Rucksack<'_> {
    fn new(line: &str) -> Rucksack {
        assert!(line.len() % 2 == 0);
        let (first, last) = line.split_at(line.len()/2);
        Rucksack{ compartment1: first, compartment2: last}
    }

    fn in_both(&self) -> char {
        let counts = self.compartment1.chars().collect::<Counter<_>>();
        let intersect = counts & self.compartment2.chars().collect::<Counter<_>>();
        println!("intersect: {:?}", intersect);

        if intersect.len() > 1 {
            panic!("more than one type in common");
        }
        match intersect.iter().next() {
            Some((x,_)) => *x,
            None => panic!("no types in common")
        }
    }

    fn counts(&self) -> Counter<char> {
        let mut c = self.compartment1.chars().collect::<Counter<_>>();
        c += self.compartment2.chars().collect::<Counter<_>>();
        c
    }
}

#[derive(Debug)]
struct Group<'a> {
    rucksacks: Vec<Rucksack<'a>>
}

impl Group<'_> {
    fn new<'a>(lines: &'a [String]) -> Group<'a> {
        Group{ rucksacks: lines.iter().map(|x| Rucksack::new(x)).collect() }
    }

    fn get_badge(&self) -> char {
        let intersect = self.rucksacks.iter().map(|x| x.counts()).reduce(|a,b| a&b).unwrap();

        if intersect.len() > 1 {
            panic!("more than one type in common");
        }
        match intersect.iter().next() {
            Some((x,_)) => *x,
            None => panic!("no types in common")
        }
    }
}

fn make_groups<'a>(lines: &'a Vec<String>) -> Vec<Group<'a>> {
    assert!(lines.len() % 3 == 0);
    let mut ret = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        ret.push(Group::new(&lines[i..i+3]))
    }
    ret
}


fn convert_to_prio(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 65 + 27
    } else {
        c as u32 - 97 + 1
    }
}

fn main() {
    let lines = read_lines("input");
    let rucksacks: Vec<_> = lines.iter().map(|x| Rucksack::new(x)).collect();

    let sum_prio = rucksacks.iter().map(|x| convert_to_prio(x.in_both())).reduce(|a,b| a+b).unwrap();
    println!("sum prio: {}", sum_prio);
    
    let groups = make_groups(&lines);
    let sum_prio2 = groups.iter().map(|x| convert_to_prio(x.get_badge())).reduce(|a,b| a+b).unwrap();
    println!("sum prio 2: {}", sum_prio2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
".lines().map(String::from).collect();

        assert_eq!(convert_to_prio('a'), 1);
        assert_eq!(convert_to_prio('A'), 27);

        let rucksacks: Vec<_> = sample.iter().map(|x| Rucksack::new(x)).collect();

        assert_eq!(rucksacks[0].compartment1, "vJrwpWtwJgWr");
        assert_eq!(rucksacks[0].compartment2, "hcsFMMfFFhFp");
        assert_eq!(rucksacks[0].in_both(), 'p');
        assert_eq!(rucksacks[1].in_both(), 'L');
        assert_eq!(rucksacks[2].in_both(), 'P');
        assert_eq!(rucksacks[3].in_both(), 'v');
        assert_eq!(rucksacks[4].in_both(), 't');
        assert_eq!(rucksacks[5].in_both(), 's');

        let sum_prio = rucksacks.iter().map(|x| convert_to_prio(x.in_both())).reduce(|a,b| a+b).unwrap();
        assert_eq!(sum_prio, 157);

        let groups = make_groups(&sample);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].get_badge(), 'r');
        assert_eq!(groups[1].get_badge(), 'Z');
        let sum_prio2 = groups.iter().map(|x| convert_to_prio(x.get_badge())).reduce(|a,b| a+b).unwrap();
        assert_eq!(sum_prio2, 70);
    }
}