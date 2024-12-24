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

    fn get_path(&self) -> Vec<Pos> {
        let mut pos = self.start;
        let mut last_pos = self.start;
        let mut history = Vec::new();
        let max_row = self.map.len() as Num -1;
        let max_col = self.map[0].len() as Num -1;
        while pos != self.end {
            history.push(pos);
            let next_pos = {
                if pos.0 != 0 && (pos.0-1, pos.1) != last_pos && !self.map[pos.0 as usize -1][pos.1 as usize ] {
                    // try up
                    (pos.0-1, pos.1)
                } else if pos.0 != max_row && (pos.0+1, pos.1) != last_pos && !self.map[pos.0 as usize +1][pos.1 as usize ] {
                    // try down
                    (pos.0+1, pos.1)
                } else if pos.1 != 0 && (pos.0, pos.1-1) != last_pos && !self.map[pos.0 as usize ][pos.1 as usize -1] {
                    // try left
                    (pos.0, pos.1-1)
                } else if pos.1 != max_col && (pos.0, pos.1+1) != last_pos && !self.map[pos.0 as usize ][pos.1 as usize +1] {
                    // try right
                    (pos.0, pos.1+1)
                } else {
                    panic!("no next pos")
                }
            };
            last_pos = pos;
            pos = next_pos;
        }
        history
    }

    fn cnt_steps_cheats(&self, cheat_moves: usize, save: usize) -> usize {
        println!("trying for {} cheats to save {} steps", cheat_moves, save);
        let path = self.get_path();
        let min = path.len();
        println!("min steps: {}", min);
        let max = min - save;

        let mut steps_hash = HashMap::new();
        let mut queue = Vec::new();
        for i in 0..min {
            steps_hash.insert(path[i], min-i);
            queue.push((None, None, i, path[i]));
        }
        steps_hash.insert(self.end, 0);

        let mut history_cheats = HashSet::new();
        let max_row = self.map.len() as Num -1;
        let max_col = self.map[0].len() as Num -1;
        let mut cheat_mins = HashSet::new();
        while !queue.is_empty() {
            let (cheat1, cheat2, steps, pos) = queue.pop().unwrap();

            if steps > max {
                continue;
            }

            if pos == self.end {
                if match (cheat1, cheat2) {
                    (Some((cheat_pos,_)), Some(cheat_pos2)) => {
                        cheat_mins.insert((cheat_pos,cheat_pos2))
                    },
                    (Some((cheat_pos,_)), None) => {
                        cheat_mins.insert((cheat_pos,pos))
                    },
                    _ => panic!("must use cheats!")
                } {
                    //println!("steps: {}, cheat1: {:?}, cheat2: {:?}", steps, cheat1, cheat2);
                }
                continue;
            }

            if history_cheats.contains(&(cheat1, cheat2, steps, pos)) {
                // we've been here before
                continue;
            }
            history_cheats.insert((cheat1, cheat2, steps, pos));

            match (cheat1, cheat2) {
                (Some(_), Some(_)) => {
                    // skip to end
                    let new_steps = steps + steps_hash.get(&pos).unwrap();
                    queue.push((cheat1, cheat2, new_steps, self.end));
                    continue;
                },
                _ => {
                    // try cheating again
                    let (cheat1_next, end_cheat) = match cheat1 {
                        Some((p,s)) => (Some((p, s+1)), s+1 == cheat_moves as i32),
                        None => (Some((pos, 1)), false)
                    };
                    let mut try_pos = |p: Pos| {
                        match (end_cheat, self.map[p.0 as usize][p.1 as usize ]) {
                            (false, true) => {
                                queue.push((cheat1_next, None, steps+1, (p.0, p.1)));
                            },
                            (false, false) => {
                                queue.push((cheat1_next, None, steps+1, (p.0, p.1)));
                                // end early
                                queue.push((cheat1_next, Some((p.0, p.1)), steps+1, (p.0, p.1)));
                            },
                            (true, false) => {
                                // end cheat
                                queue.push((cheat1_next, Some((p.0, p.1)), steps+1, (p.0, p.1)));
                            },
                            _ => { }  // cheat failed
                        };
                    };
                    if pos.0 != 0 {
                        // try up
                        try_pos((pos.0-1, pos.1));
                    }
                    if pos.0 != max_row {
                        // try down
                        try_pos((pos.0+1, pos.1));
                    }
                    if pos.1 != 0 {
                        // try left
                        try_pos((pos.0, pos.1-1));
                    }
                    if pos.1 != max_col {
                        // try right
                        try_pos((pos.0, pos.1+1));
                    }
                }
            };
        }
        cheat_mins.len()
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Track::new(&lines);
    println!("cnt: {:?}", c.cnt_steps_cheats(20, 100));
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
        assert_eq!(c.cnt_steps_cheats(2, 64), 1);
        assert_eq!(c.cnt_steps_cheats(2, 40), 2);
        assert_eq!(c.cnt_steps_cheats(2, 38), 3);
        assert_eq!(c.cnt_steps_cheats(2, 36), 4);
        assert_eq!(c.cnt_steps_cheats(2, 20), 5);
        assert_eq!(c.cnt_steps_cheats(2, 12), 5+3);
        assert_eq!(c.cnt_steps_cheats(2, 10), 5+3+2);
        assert_eq!(c.cnt_steps_cheats(2, 8), 5+3+2+4);
        assert_eq!(c.cnt_steps_cheats(2, 6), 5+3+2+4+2);
        assert_eq!(c.cnt_steps_cheats(2, 4), 5+3+2+4+2+14);
        assert_eq!(c.cnt_steps_cheats(2, 2), 5+3+2+4+2+14+14);
    }

    #[test]
    fn test_2() {
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
        assert_eq!(c.cnt_steps_cheats(20, 76), 3);
        assert_eq!(c.cnt_steps_cheats(20, 74), 7);
        assert_eq!(c.cnt_steps_cheats(20, 72), 29);
        assert_eq!(c.cnt_steps_cheats(20, 70), 41);
        assert_eq!(c.cnt_steps_cheats(20, 68), 55);
    }

    #[test]
    fn test_3a() {
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
        assert_eq!(c.cnt_steps_cheats(20, 66), 3+4+22+12+14+12);
        assert_eq!(c.cnt_steps_cheats(20, 64), 3+4+22+12+14+12+19);
    }

    #[test]
    fn test_3b() {
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
        assert_eq!(c.cnt_steps_cheats(20, 62), 3+4+22+12+14+12+19+20);
    }

    #[test]
    fn test_4a() {
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
        assert_eq!(c.cnt_steps_cheats(20, 60), 3+4+22+12+14+12+19+20+23);
    }

    #[test]
    fn test_4b() {
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
        assert_eq!(c.cnt_steps_cheats(20, 58), 3+4+22+12+14+12+19+20+23+25);
    }

    #[test]
    fn test_part1() {        
        let lines = read_lines("input");
        let c = Track::new(&lines);
        assert_eq!(c.cnt_steps_cheats(2, 100), 1406);
    }
}