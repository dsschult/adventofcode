use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

enum Facing {
    Up,
    Right,
    Left,
    Down
}

impl Facing {
    fn next_facing(&self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    objects: Vec<Vec<bool>>,
    guard: (i32, i32)
}

impl Map {
    fn new(lines: &Vec<String>) -> Map {
        let mut ret = Vec::new();
        let mut guard = None;

        for line in lines.iter() {
            let mut row = Vec::new();
            if line.trim().is_empty() {
                continue;
            }
            for c in line.chars() {
                match c {
                    '^' => {
                        guard = Some((ret.len() as i32, row.len() as i32));
                        row.push(false);
                    },
                    '#' => {
                        row.push(true);
                    },
                    _ => {
                        row.push(false);
                    }
                };
            }
            ret.push(row);
        }
        Map{ objects: ret, guard: guard.expect("no guard!") }
    }

    fn walk_guard(&self) -> usize {
        let mut guard_pos = Vec::new();
        let rows = self.objects.len();
        let cols = self.objects[0].len();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(false);
            }
            guard_pos.push(row);
        }
        guard_pos[self.guard.0 as usize][self.guard.1 as usize] = true;
        let mut current_pos = self.guard.clone();
        let mut facing = Facing::Up;

        loop {
            let next_pos = match facing {
                Facing::Up => (current_pos.0-1, current_pos.1),
                Facing::Right => (current_pos.0, current_pos.1+1),
                Facing::Down => (current_pos.0+1, current_pos.1),
                Facing::Left => (current_pos.0, current_pos.1-1)
            };
            if next_pos.0 < 0 || next_pos.0 >= rows as i32 || next_pos.1 < 0 || next_pos.1 >= cols as i32 {
                // guard is out!
                break;
            }
            if self.objects[next_pos.0 as usize][next_pos.1 as usize] {
                println!("hit object at {:?}", next_pos);
                facing = facing.next_facing();
            } else {
                guard_pos[next_pos.0 as usize][next_pos.1 as usize] = true;
                current_pos = next_pos;
            }
        }
        for row in guard_pos.iter() {
            println!("{}", row.iter().map(|x| match x { true => 'X', _ => '.'}).collect::<String>());
        }

        guard_pos.into_iter().fold(0, |r, col| r + col.into_iter().filter(|x| *x).count())
    }
}

fn main() {
    let lines = read_lines("input");
    let map = Map::new(&lines);
    println!("Guard: {}", map.walk_guard());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
".lines().map(String::from).collect();

        let map = Map::new(&sample);
        assert_eq!(map.walk_guard(), 41);
    }
}