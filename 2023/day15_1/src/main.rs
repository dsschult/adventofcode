use std::fs::read_to_string;

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
            ret += holiday_hash(part);
        }
        ret
    }
}


fn main() {
    let lines = read_lines("input");
    
    let s = InitSeq::new(lines[0].as_str());
    println!("{}", s.sum_hash());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hash() {
        let s = "HASH";
        assert_eq!(holiday_hash(s), 52);
    }

    #[test]
    fn test_basic() {
        let sample = String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        let s = InitSeq::new(sample.as_str());
        assert_eq!(s.sum_hash(), 1320);
        
    }
}
