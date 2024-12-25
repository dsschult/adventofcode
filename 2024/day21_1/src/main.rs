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

type Num = u8;
type Pos = (Num, Num);

#[derive(Debug, Clone)]
enum Keypad {
    NumKeypad {
        keys: Vec<Vec<char>>,
    },
    DirKeypad {
        keys: Vec<Vec<char>>,
    }
}

impl Keypad {
    fn get_num_keypad() -> Keypad {
        Keypad::NumKeypad{
            keys: vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec![' ', '0', 'A'],
            ]
        }
    }

    fn get_dir_keypad() -> Keypad {
        Keypad::DirKeypad{
            keys: vec![
                vec![' ', '^', 'A'],
                vec!['<', 'v', '>']
            ]
        }
    }

    fn get_keys(&self) -> &Vec<Vec<char>> {
        match self {
            Keypad::NumKeypad{keys: k} => k,
            Keypad::DirKeypad{keys: k} => k,
        }
    }

    fn find_pos(&self, c: char) -> Pos {
        let keys = self.get_keys();
        for i in 0..keys.len() {
            let row = &keys[i as usize];
            for j in 0..row.len() {
                if row[j as usize] == c {
                    return (i as Num, j as Num);
                }
            }
        }
        panic!("cannot find {}", c);
    }

    fn find_moves(&self, start: char, end: char) -> Vec<String> {
        let keys = self.get_keys();
        let max_row = keys.len() as Num - 1;
        let max_col = keys[0].len() as Num - 1;
        let mut hist = HashMap::new();
        let mut queue = vec![(String::new(), self.find_pos(start))];
        let mut ret: Vec<String> = Vec::new();

        while !queue.is_empty() {
            let (mut steps, pos) = queue.pop().unwrap();

            match ret.is_empty() {
                true => { },
                false => {
                    if ret[0].len() <= steps.len() {
                        continue;
                    }
                }
            };

            if keys[pos.0 as usize][pos.1 as usize] == end {
                steps.push('A');
                ret.push(steps);
                continue;
            }

            match hist.get(&pos) {
                Some(len) if *len < steps.len() => {
                    continue;
                },
                _ => {}
            };
            hist.insert(pos, steps.len());

            if pos.0 != 0 && keys[(pos.0-1) as usize][pos.1 as usize] != ' ' {
                let mut next_steps = steps.clone();
                next_steps.push('^');
                queue.push((next_steps, (pos.0-1, pos.1)));
            }
            if pos.0 != max_row && keys[(pos.0+1) as usize][pos.1 as usize] != ' ' {
                let mut next_steps = steps.clone();
                next_steps.push('v');
                queue.push((next_steps, (pos.0+1, pos.1)));
            }
            if pos.1 != 0 && keys[pos.0 as usize][(pos.1-1) as usize] != ' ' {
                let mut next_steps = steps.clone();
                next_steps.push('<');
                queue.push((next_steps, (pos.0, pos.1-1)));
            }
            if pos.1 != max_col && keys[pos.0 as usize][(pos.1+1) as usize] != ' ' {
                let mut next_steps = steps.clone();
                next_steps.push('>');
                queue.push((next_steps, (pos.0, pos.1+1)));
            }
        }
        let min = ret.iter().map(|x| x.len()).min().unwrap();
        ret.into_iter().filter(|x| x.len() == min).collect::<Vec<_>>()
    }
}

struct KeypadIter<'a> {
    keys: &'a Vec<Vec<char>>,
    max_row: Num,
    max_col: Num,
    hist: HashMap<Pos, usize>,
    queue: Vec<(String, Pos)>,
    end: char,
}

impl Iterator for KeypadIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug, Clone)]
struct Codes {
    data: Vec<String>,
    keypads: Vec<Keypad>,
}

fn get_numeric(s: &String) -> usize {
    println!("get_numeric({})", s);
    s.trim_end_matches('A').parse::<usize>().unwrap()
}

impl Codes {
    fn new(lines: &Vec<String>) -> Codes {
        Codes{
            data: lines.iter().map(|x| x.trim().to_string()).filter(|x| x.len() > 0).collect::<Vec<_>>(),
            keypads: vec![
                Keypad::get_num_keypad(),
                Keypad::get_dir_keypad(),
                Keypad::get_dir_keypad(),
            ],
        }
    }

    fn get_keys_for_code(&self, code: &String) -> String {
        let mut min_solution: Option<String> = None;
        let max_keypad = self.keypads.len() - 1;
        let mut memoize_keypads = HashMap::new();
        let mut find_moves_memoized = |keypad_index: usize, start: char, end: char| -> Vec<String> {
            match memoize_keypads.get(&(keypad_index, start, end)) {
                None => {
                    let ret = self.keypads[keypad_index].find_moves(start, end);
                    memoize_keypads.insert((keypad_index, start, end), ret.clone());
                    ret
                },
                Some(ret) => ret.to_vec(),
            }
        };

        let mut code2 = code.clone();
        let code3 = code2.split_off(1);
        let last_char = code2.chars().next().unwrap();
        // (keypad index, code str, last_char, result string)
        let mut queue = Vec::new();
        for ret in find_moves_memoized(0, 'A', last_char).into_iter() {
            queue.push((0, code3.clone(), last_char, ret));
        }

        while !queue.is_empty() {
            let (keypad_index, code_str, last_char, cur_str) = queue.pop().unwrap();

            match min_solution {
                Some(ref s) if s.len() <= cur_str.len() => {
                    continue;
                },
                _ => {},
            }

            match code_str.is_empty() {
                true => {
                    match keypad_index == max_keypad {
                        true => {
                            println!("solution len {}", cur_str.len());
                            println!("  str: {}", cur_str);
                            min_solution = Some(cur_str);
                            continue;
                        },
                        false => {
                            let new_keypad_index = keypad_index + 1;
                            println!("+keypad {}, len {}", new_keypad_index, cur_str.len());
                            println!("  str: {}", cur_str);
                            let mut code2 = cur_str.clone();
                            let new_code_str = code2.split_off(1);
                            let last_char = code2.chars().next().unwrap();
                            for ret in find_moves_memoized(new_keypad_index, 'A', last_char).into_iter() {
                                queue.push((new_keypad_index, new_code_str.clone(), last_char, ret));
                            }
                        }
                    };
                },
                false => {
                    //println!("keypad {}, code remaining len {}, sol len {}", keypad_index, code_str.len(), cur_str.len());
                    //println!("  str: {}", cur_str);
                    let mut code2 = code_str.clone();
                    let new_code_str = code2.split_off(1);
                    let next_char = code2.chars().next().unwrap();
                    for ret in find_moves_memoized(keypad_index, last_char, next_char).into_iter() {
                        queue.push((keypad_index, new_code_str.clone(), next_char, cur_str.clone() + ret.as_str()));
                    }
                }
            };
        }
        match min_solution {
            Some(s) => s,
            None => panic!("no solution!"),
        }
    }

    fn calc_complexity(&self) -> usize {
        let mut ret = 0;
        for code in self.data.iter() {
            let keys = self.get_keys_for_code(&code);
            let numeric = get_numeric(code);
            println!("code {} keys {}", code, keys);
            println!("keylen {} numeric {}", keys.len(), numeric);
            ret += keys.len() * numeric;
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Codes::new(&lines);
    println!("complexity: {}", c.calc_complexity());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn min_keys_for_str(s: String, start_char: char, keypad: &Keypad) -> Vec<String> {
        let mut ret = Vec::new();
        let mut last_char = start_char;
        for c in s.chars() {
            //println!("moving from {} to {}", last_char, c);
            let possibles = keypad.find_moves(last_char, c);
            if ret.is_empty() {
                ret = possibles;
            } else {
                let mut ret2 = Vec::new();
                for r1 in ret.into_iter() {
                    for p in possibles.iter() {
                        ret2.push(r1.clone() + p);
                    }
                }
                ret = ret2;
            }
            last_char = c;
        }
        let min = ret.iter().map(|x| x.len()).min().unwrap();
        ret.into_iter().filter(|x| x.len() == min).collect::<Vec<_>>()
    }

    #[test]
    fn test_1() {
        let ret = min_keys_for_str(String::from("029A"), 'A', &Keypad::get_num_keypad());
        assert_eq!(ret.len(), 3);
        let set_ret = ret.into_iter().collect::<HashSet<_>>();
        assert_eq!(set_ret, HashSet::from([
            String::from("<A^A>^^AvvvA"),
            String::from("<A^A^>^AvvvA"),
            String::from("<A^A^^>AvvvA")
        ]));
    }

    #[test]
    fn test_2() {
        let ret = min_keys_for_str(String::from("<A^A>^^AvvvA"), 'A', &Keypad::get_dir_keypad());
        let set_ret = ret.into_iter().collect::<HashSet<_>>();
        assert_eq!(set_ret.contains(&String::from("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")), true);
    }

    #[test]
    fn test_3() {
        let ret = min_keys_for_str(String::from("v<<A>>^A<A>AvA<^AA>A<vAAA>^A"), 'A', &Keypad::get_dir_keypad());
        let set_ret = ret.into_iter().collect::<HashSet<_>>();
        assert_eq!(set_ret.contains(&String::from("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")), true);
    }

    #[test]
    fn test_10() {
        let sample: Vec<String> = "
029A
980A
179A
456A
379A
".lines().map(String::from).collect();

        let c = Codes::new(&sample);
        assert_eq!(c.calc_complexity(), 126384);
    }
}