use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn holiday_hash(s: &str) -> i64 {
    let mut ret = 0;
    for c in s.chars() {
        ret += c as i64;
        ret *= 17;
        ret %= 256;
    }
    ret
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u8,
}


#[derive(Debug, Clone, Eq, PartialEq)]
struct InitSeq<'a> {
    list: Vec<&'a str>    
}

impl InitSeq<'_> {
    fn new(line: &str) -> InitSeq {
        InitSeq{list: line.split(',').collect::<Vec<_>>() }
    }

    fn sum_hash(&self) -> i64 {
        let mut ret = 0;
        for part in self.list.iter() {
            ret += holiday_hash(&part);
        }
        ret
    }

    fn calc(&self) -> i64 {
        let mut boxes = vec![Vec::new(); 256];
        for part in self.list.iter() {
            let (label, rest) = part.split_at(match part.find('=') { Some(i) => i, None => match part.find('-') { Some(i) => i, None => panic!("cannot find split for label")}});
            let key = holiday_hash(label) as usize;
            match rest.chars().next().unwrap() {
                '-' => match boxes[key].iter().position(|x: &Lens| x.label == label) {
                    Some(i) => { boxes[key].remove(i); },
                    None => {},
                },
                '=' => {
                    let lens = Lens{label: &label, focal_len: rest[1..].parse::<u8>().unwrap()};
                    match boxes[key].iter().position(|x: &Lens| x.label == label) {
                        Some(i) => { boxes[key][i] = lens; },
                        None => { boxes[key].push(lens); },
                    }
                },
                _ => panic!("bad sep: {:?}", part),
            }
            println!("{:?}", boxes);
        }
        let mut ret = 0;
        for (i,b) in boxes.iter().enumerate() {
            let box_num = i as i64;
            for (j,lens) in b.iter().enumerate() {
                let slot_num = j as i64+1;
                ret += (1+box_num) * slot_num * lens.focal_len as i64;
            }
        }
        ret
    }
}


fn main() {
    let lines = read_lines("input");
    
    let s = InitSeq::new(lines[0].as_str());
    println!("{}", s.calc());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hash() {
        let s = "HASH";
        assert_eq!(holiday_hash(&s), 52);
    }

    #[test]
    fn test_basic() {
        let sample = String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        let s = InitSeq::new(sample.as_str());
        assert_eq!(s.sum_hash(), 1320);

        assert_eq!(s.calc(), 145);
    }
}
