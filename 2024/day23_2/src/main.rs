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
    nodes: HashMap<Num, HashSet<Num>>,
    interconnects: HashSet<Vec<Num>>,
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
            nodes: HashMap::new(),
            interconnects: HashSet::new()
        }
    }

    fn make_interconnects(&mut self) {
        for (a,b) in self.links.iter() {
            match self.nodes.get_mut(&a) {
                Some(set) => { set.insert(*b); },
                None => { self.nodes.insert(*a, HashSet::from([*b])); }
            };
            match self.nodes.get_mut(&b) {
                Some(set) => { set.insert(*a); },
                None => { self.nodes.insert(*b, HashSet::from([*a])); }
            };
        }

        let mut changes = true;
        let mut interconnects = HashSet::new();
        for (n1, friends) in self.nodes.iter() {
            for n2 in friends.iter() {
                if n1 < n2 {
                    interconnects.insert(vec![*n1, *n2]);
                } else {
                    interconnects.insert(vec![*n1, *n2]);
                }
            }
        }
        while changes {
            changes = false;
            for (n1, friends) in self.nodes.iter() {
                let mut new_interconnects = HashSet::new();
                for i in interconnects.iter() {
                    if friends.is_superset(&i.iter().map(|x| *x).collect::<HashSet<_>>()) {
                        let mut c = i.clone();
                        c.push(*n1);
                        c.sort();
                        println!("found larger interconnect: {:?}", c.iter().map(|x| decode(*x)).collect::<Vec<_>>());
                        new_interconnects.insert(c);
                    }
                }
                if !new_interconnects.is_empty() {
                    let len = interconnects.len();
                    interconnects = interconnects.union(&new_interconnects).map(|x| x.clone()).collect();
                    changes = len < interconnects.len();
                }
            }
        }
        self.interconnects = interconnects;
    }

    fn largest_set(&self) -> HashSet<Num> {
        self.interconnects.iter().fold(Vec::new(), |ret: Vec<Num>, x| {
            if x.len() > ret.len() {
                x.clone()
            } else {
                ret
            }
        }).into_iter().collect()
    }
}

fn main() {
    let lines = read_lines("input");
    let mut n = Network::new(&lines);
    n.make_interconnects();
    let mut set =  n.largest_set().iter().map(|x| decode(*x)).collect::<Vec<_>>();
    set.sort();
    println!("set: {:?}", set.join(","));
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
        n.make_interconnects();
        assert_eq!(n.largest_set(), HashSet::from([encode("co"), encode("de"), encode("ka"), encode("ta")]));
    }
}