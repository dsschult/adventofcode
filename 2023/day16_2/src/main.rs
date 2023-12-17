use std::fs::read_to_string;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Beam {
    pos: (i32,i32),
    vec: (i32,i32),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
    rows: Vec<Vec<char>>,
    row_len: i32,
    col_len: i32,
}

impl Grid {
    fn new(lines: &Vec<String>) -> Grid {
        let r = lines.iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let r_len = r.len() as i32;
        let c_len = r[0].len() as i32;
        Grid{ rows: r, row_len: r_len, col_len: c_len }
    }

    fn valid_beam(&self, b: Beam) -> Option<Beam> {
        //println!("valid_beam(): rowlen={} collen={} beam={:?}", self.row_len, self.col_len, b.pos);
        match b.pos.0 < 0 || b.pos.0 >= self.row_len || b.pos.1 < 0 || b.pos.1 >= self.col_len {
            true => None,
            false => Some(b),
        }
    }

    fn beam_next(&self, beam: Beam) -> (Option<Beam>, Option<Beam>) {
        match self.rows[beam.pos.0 as usize][beam.pos.1 as usize] {
            '.' => { // continue "straight"
                (self.valid_beam(Beam{ pos: (beam.pos.0 + beam.vec.0, beam.pos.1 + beam.vec.1), vec: beam.vec }), None)
            },
            '/' => match beam.vec { // mirror
                (0,1) => { // moving "right" to "up"
                    (self.valid_beam(Beam{ pos: (beam.pos.0 - 1, beam.pos.1), vec: (-1, 0) }), None)
                },
                (-1,0) => { // moving "up" to "right"
                    (self.valid_beam(Beam{ pos: (beam.pos.0, beam.pos.1 + 1), vec: (0, 1) }), None)
                },
                (1,0) => { // moving "down" to "left"
                    (self.valid_beam(Beam{ pos: (beam.pos.0, beam.pos.1 - 1), vec: (0, -1) }), None)
                },
                (0,-1) => { // moving "left" to "down"
                    (self.valid_beam(Beam{ pos: (beam.pos.0 + 1, beam.pos.1), vec: (1, 0) }), None)
                },
                _ => panic!("invalid vec"),
            },
            '\\' => match beam.vec { // mirror
                (0,1) => { // moving "right" to "down"
                    (self.valid_beam(Beam{ pos: (beam.pos.0 + 1, beam.pos.1), vec: (1, 0) }), None)
                },
                (-1,0) => { // moving "up" to "left"
                    (self.valid_beam(Beam{ pos: (beam.pos.0, beam.pos.1 - 1), vec: (0, -1) }), None)
                },
                (1,0) => { // moving "down" to "right"
                    (self.valid_beam(Beam{ pos: (beam.pos.0, beam.pos.1 + 1), vec: (0, 1) }), None)
                },
                (0,-1) => { // moving "left" to "up"
                    (self.valid_beam(Beam{ pos: (beam.pos.0 - 1, beam.pos.1), vec: (-1, 0) }), None)
                },
                _ => panic!("invalid vec"),
            },
            '-' => match beam.vec { // splitter
                (0,1) | (0,-1) => { // pointy end
                    (self.valid_beam(Beam{ pos: (beam.pos.0 + beam.vec.0, beam.pos.1 + beam.vec.1), vec: beam.vec }), None)
                },
                (-1,0) | (1, 0) => { // split left and right
                    let b2 = self.valid_beam(Beam{ pos: (beam.pos.0, beam.pos.1 + 1), vec: (0, 1) });
                    match self.valid_beam(Beam{ pos: (beam.pos.0, beam.pos.1 - 1), vec: (0, -1) }) {
                        Some(b) => (Some(b), b2),
                        None => (b2, None),
                    }
                },
                _ => panic!("invalid vec"),
            },
            '|' => match beam.vec { // splitter
                (-1,0) | (1, 0) => { // pointy end
                    (self.valid_beam(Beam{ pos: (beam.pos.0 + beam.vec.0, beam.pos.1 + beam.vec.1), vec: beam.vec }), None)
                },
                (0,1) | (0,-1) => { // split up and down
                    let b2 = self.valid_beam(Beam{ pos: (beam.pos.0 + 1, beam.pos.1), vec: (1, 0) });
                    match self.valid_beam(Beam{ pos: (beam.pos.0 - 1, beam.pos.1), vec: (-1, 0) }) {
                        Some(b) => (Some(b), b2),
                        None => (b2, None),
                    }
                },
                _ => panic!("invalid vec"),
            },
            _ => panic!("invalid char"),
        }
    }

    fn energize(&self, start_beam: &Beam) -> usize {
        // starting state
        let mut beams = vec![start_beam.clone()];
        let mut energized: Vec<Vec<bool>> = vec![vec![false; self.col_len as usize]; self.row_len as usize];
        let mut cache = HashSet::new();

        while !beams.is_empty() {
            let b = beams.pop().unwrap();
            if cache.get(&b).is_some() {
                continue;
            } else {
                cache.insert(b.clone());
            }
            //println!("energizing {:?}", b.pos);
            energized[b.pos.0 as usize][b.pos.1 as usize] = true;
            match self.beam_next(b) {
                (Some(b1), Some(b2)) => {
                    beams.push(b1);
                    beams.push(b2);
                },
                (Some(b1), None) => {
                    beams.push(b1);
                },
                _ => { },
            };
        }
        let mut ret = 0;
        for row in energized.into_iter() {
            ret += row.iter().filter(|x| **x).count();
        }
        ret
    }

    fn most_energy(&self) -> usize {
        let mut max_energy = 0;
        let mut max_beam = None;
        // top side
        for i in 0..self.col_len {
            let beam = Beam{ pos: (0,i), vec: (1,0) };
            let e = self.energize(&beam);
            if e > max_energy {
                max_energy = e;
                max_beam = Some(beam);
            }
        }
        // bottom side
        for i in 0..self.col_len {
            let beam = Beam{ pos: (self.row_len-1,i), vec: (-1,0) };
            let e = self.energize(&beam);
            if e > max_energy {
                max_energy = e;
                max_beam = Some(beam);
            }
        }
        // left side
        for i in 0..self.row_len {
            let beam = Beam{ pos: (i,0), vec: (0,1) };
            let e = self.energize(&beam);
            if e > max_energy {
                max_energy = e;
                max_beam = Some(beam);
            }
        }
        // right side
        for i in 0..self.row_len {
            let beam = Beam{ pos: (i,self.col_len-1), vec: (0,-1) };
            let e = self.energize(&beam);
            if e > max_energy {
                max_energy = e;
                max_beam = Some(beam);
            }
        }
        max_energy
        /*match max_beam {
            Some(b) => b,
            None => panic!("no beam!"),
        }*/
    }
}


fn main() {
    let lines = read_lines("input");
    
    let g = Grid::new(&lines);
    println!("{:?}", g.most_energy());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let sample: Vec<String> = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
".lines().map(String::from).collect();

        let g = Grid::new(&sample);

        assert_eq!(g.valid_beam(Beam{pos: (0,0), vec: (0,1)}), Some(Beam{pos: (0,0), vec: (0,1)}));
        assert_eq!(g.valid_beam(Beam{pos: (0,1), vec: (0,1)}), Some(Beam{pos: (0,1), vec: (0,1)}));

        assert_eq!(g.beam_next(Beam{pos: (0,0), vec: (0,1)}),
            (Some(Beam{pos: (0,1), vec: (0,1)}), None));

        assert_eq!(g.energize(&Beam{ pos: (0,0), vec: (0,1) }), 46);
        
        assert_eq!(g.most_energy(), 51);//Beam{ pos: (0,3), vec: (1,0) });
    }
}
