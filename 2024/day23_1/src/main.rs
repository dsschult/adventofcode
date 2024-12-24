use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = u16;
type Pair = (Num, Num);
type Set = (Num, Num, Num);

fn encode(s: &str) -> Num {
    let b = s.as_bytes();
    ((b[0] as Num) << 8) + b[1] as Num
}

fn decode(b: Num) -> String {
    String::from_utf8(vec![(b / 256) as u8, (b % 256) as u8]).unwrap()
}


#[derive(Debug, Clone)]
struct Network {
    links: Vec<Pair>,
    interconnects: HashSet<Set>
}

impl Network {
    fn new(lines: &Vec<String>) -> Network {
        let mut data = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            let parts = trim_line.split('-').map(|x| encode(x)).collect::<Vec<_>>();
            data.push((parts[0], parts[1]));
        }

        Network{
            links: data,
            interconnects: HashSet::new(),
        }
    }

    fn make_interconnects(&mut self, letter: char) {
        let search = letter as Num;
        let len = self.links.len();
        for x in 0..len {
            for y in x+1..len {
                for z in y+1..len {
                    let (c1, c2) = self.links[x];
                    let (c3, c4) = self.links[y];
                    let (c5, c6) = self.links[z];
                    if (c1 >> 8) != search && (c2 >> 8) != search && (c3 >> 8) != search &&
                       (c4 >> 8) != search && (c5 >> 8) != search && (c6 >> 8) != search {
                        continue;
                    }
                    let set = HashSet::from([c1, c2, c3, c4, c5, c6]);
                    if set.len() == 3 {
                        let mut iter = set.iter();
                        let a = *iter.next().unwrap();
                        let b = *iter.next().unwrap();
                        let c = *iter.next().unwrap();
                        if self.interconnects.contains(&(a,b,c)) ||
                           self.interconnects.contains(&(b,c,a)) || 
                           self.interconnects.contains(&(c,a,b)) || 
                           self.interconnects.contains(&(a,c,b)) || 
                           self.interconnects.contains(&(b,a,c)) || 
                           self.interconnects.contains(&(c,b,a)) {
                            continue;
                        }
                        println!("making interconnect for {},{},{}", decode(a), decode(b), decode(c));
                        self.interconnects.insert((a,b,c));
                    }
                }
            }
        }
    }

    fn sets(&self) -> usize {
        self.interconnects.len()
    }
}

fn main() {
    let lines = read_lines("input");
    let mut n = Network::new(&lines);
    n.make_interconnects('t');
    println!("count: {}", n.sets());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        let sample: Vec<String> = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
".lines().map(String::from).collect();

        let mut n = Network::new(&sample);
        n.make_interconnects('t');
        assert_eq!(n.sets(), 7);
    }
}