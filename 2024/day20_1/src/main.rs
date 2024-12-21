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
type Pos = (Num, Num);

#[derive(Debug, Clone)]
struct Track {
    map: Vec<Vec<bool>>,
    start: Pos,
    end: Pos,
}

impl Track {
    fn new(lines: &Vec<String>) -> Track {
        let mut map = Vec::new();
        let mut start = None;
        let mut end = None;
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            let mut i = 0;
            for c in trim_line.chars() {
                match c {
                    'S' => {
                        start = Some((map.len() as Num, i));
                    },
                    'E' => {
                        end = Some((map.len() as Num, i));
                    },
                    _ => { }
                };
                i += 1;
            }
            map.push(trim_line.chars().map(|x| x == '#').collect::<Vec<_>>());
        }
        Track{
            map: map,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn min_steps(&self) -> usize {
        let mut min = None;
        let mut queue = vec![(0, self.start)];
        let mut history = HashMap::new();
        let max_row = self.map.len() as Num -1;
        let max_col = self.map[0].len() as Num -1;
        while !queue.is_empty() {
            let (steps, pos) = queue.pop().unwrap();

            match min {
                Some(x) if x <= steps => {
                    continue;
                },
                _ => { }
            };

            if pos == self.end {
                //println!("steps: {}", steps);
                min = Some(steps);
                continue
            }

            match history.get(&pos) {
                Some(x) if *x <= steps => {
                    continue;
                },
                _ => { }
            };
            history.insert(pos, steps);

            if pos.0 != 0 && !self.map[pos.0 as usize -1][pos.1 as usize ] {
                // try up
                queue.push((steps+1, (pos.0-1, pos.1)));
            }
            if pos.0 != max_row && !self.map[pos.0 as usize +1][pos.1 as usize ] {
                // try down
                queue.push((steps+1, (pos.0+1, pos.1)));
            }
            if pos.1 != 0 && !self.map[pos.0 as usize ][pos.1 as usize -1] {
                // try left
                queue.push((steps+1, (pos.0, pos.1-1)));
            }
            if pos.1 != max_col && !self.map[pos.0 as usize ][pos.1 as usize +1] {
                // try right
                queue.push((steps+1, (pos.0, pos.1+1)));
            }
        }
        min.unwrap()
    }

    fn cnt_steps_cheats(&self, save: usize) -> usize {
        println!("trying for cheats to save {} steps", save);
        let min = self.min_steps();
        println!("min steps: {}", min);
        let max = min - save;

        let mut queue = vec![(None, None, 0, self.start)];
        let mut history = HashMap::new();
        let mut history_cheats = HashMap::new();
        let max_row = self.map.len() as Num -1;
        let max_col = self.map[0].len() as Num -1;
        let mut cheat_mins = HashSet::new();
        while !queue.is_empty() {
            let (cheat1, cheat2, steps, pos) = queue.pop().unwrap();

            if steps > max {
                continue;
            }

            if pos == self.end {
                println!("steps: {}, cheat1: {:?}, cheat2: {:?}", steps, cheat1, cheat2);
                match (cheat1, cheat2) {
                    (Some(cheat_pos), Some(cheat_pos2)) => {
                        cheat_mins.insert((cheat_pos,cheat_pos2));
                    },
                    _ => { }
                }
                continue;
            }

            if cheat1.is_some() && cheat2.is_none() {
                // try cheating again
                if pos.0 != 0 && !self.map[pos.0 as usize -1][pos.1 as usize ] {
                    // try up
                    queue.push((cheat1, Some(pos), steps+1, (pos.0-1, pos.1)));
                }
                if pos.0 != max_row && !self.map[pos.0 as usize +1][pos.1 as usize ] {
                    // try down
                    queue.push((cheat1, Some(pos), steps+1, (pos.0+1, pos.1)));
                }
                if pos.1 != 0 && !self.map[pos.0 as usize ][pos.1 as usize -1] {
                    // try left
                    queue.push((cheat1, Some(pos), steps+1, (pos.0, pos.1-1)));
                }
                if pos.1 != max_col && !self.map[pos.0 as usize ][pos.1 as usize +1] {
                    // try right
                    queue.push((cheat1, Some(pos), steps+1, (pos.0, pos.1+1)));
                }
                continue;
            }

            match (cheat1, cheat2) {
                (Some(cheat_pos), Some(cheat_pos2)) => {
                    match history_cheats.get(&(cheat_pos, cheat_pos2, pos)) {
                        Some(x) if *x < steps => {
                            continue;
                        },
                        _ => { }
                    };
                    history_cheats.insert((cheat_pos, cheat_pos2, pos), steps);
                },
                _ => {
                    match history.get(&pos) {
                        Some(x) if *x < steps => {
                            continue;
                        },
                        _ => { }
                    };
                    history.insert(pos, steps);
                }
            };
            
            if pos.0 != 0 && !self.map[pos.0 as usize -1][pos.1 as usize ] {
                // try up
                queue.push((cheat1, cheat2, steps+1, (pos.0-1, pos.1)));
            }
            if pos.0 != max_row && !self.map[pos.0 as usize +1][pos.1 as usize ] {
                // try down
                queue.push((cheat1, cheat2,  steps+1, (pos.0+1, pos.1)));
            }
            if pos.1 != 0 && !self.map[pos.0 as usize ][pos.1 as usize -1] {
                // try left
                queue.push((cheat1, cheat2,  steps+1, (pos.0, pos.1-1)));
            }
            if pos.1 != max_col && !self.map[pos.0 as usize ][pos.1 as usize +1] {
                // try right
                queue.push((cheat1, cheat2,  steps+1, (pos.0, pos.1+1)));
            }
            
            if cheat1.is_none() {
                // try cheating
                if pos.0 != 0 && self.map[pos.0 as usize -1][pos.1 as usize ] {
                    // try up
                    queue.push((Some(pos), None, steps+1, (pos.0-1, pos.1)));
                }
                if pos.0 != max_row && self.map[pos.0 as usize +1][pos.1 as usize ] {
                    // try down
                    queue.push((Some(pos), None, steps+1, (pos.0+1, pos.1)));
                }
                if pos.1 != 0 && self.map[pos.0 as usize ][pos.1 as usize -1] {
                    // try left
                    queue.push((Some(pos), None, steps+1, (pos.0, pos.1-1)));
                }
                if pos.1 != max_col && self.map[pos.0 as usize ][pos.1 as usize +1] {
                    // try right
                    queue.push((Some(pos), None, steps+1, (pos.0, pos.1+1)));
                }
            }
        }
        println!("{:?}", cheat_mins);
        cheat_mins.len()
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Track::new(&lines);
    println!("cnt: {:?}", c.cnt_steps_cheats(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
".lines().map(String::from).collect();

        let c = Track::new(&sample);
        assert_eq!(c.cnt_steps_cheats(64), 1);
        assert_eq!(c.cnt_steps_cheats(40), 2);
        assert_eq!(c.cnt_steps_cheats(38), 3);
        assert_eq!(c.cnt_steps_cheats(36), 4);
        assert_eq!(c.cnt_steps_cheats(20), 5);
    }
}