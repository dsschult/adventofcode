use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = i64;

type Pair = (Num, Num);

enum Quadrant {
    One,
    Two,
    Three,
    Four
}

#[derive(Debug, Clone)]
struct Robot {
    position: Pair,
    velocity: Pair
}

impl Robot {
    fn new(line: &str, quad_size: &Pair) -> Robot {
        let parts1 = line.split_whitespace().collect::<Vec<_>>();
        let parts2 = parts1[0].split('=').collect::<Vec<_>>()[1].split(',').collect::<Vec<_>>();
        let p = (parts2[0].parse::<Num>().unwrap() - quad_size.0, parts2[1].parse::<Num>().unwrap() - quad_size.1);
        let parts3 = parts1[1].split('=').collect::<Vec<_>>()[1].split(',').collect::<Vec<_>>();
        let v = (parts3[0].parse::<Num>().unwrap(), parts3[1].parse::<Num>().unwrap());
        Robot{ position: p, velocity: v }
    }

    fn make_move(&mut self, quad_size: &Pair) {
        let mut x = self.position.0 + self.velocity.0;
        let mut y = self.position.1 + self.velocity.1;
        if x < quad_size.0*-1 {
            x += quad_size.0 * 2 + 1;
        } else if x > quad_size.0 {
            x -= quad_size.0 * 2 + 1;
        }
        if y < quad_size.1*-1 {
            y += quad_size.1 * 2 + 1;
        } else if y > quad_size.1 {
            y -= quad_size.1 * 2 + 1;
        }
        self.position = (x,y);
    }

    fn quadrant(&self) -> Option<Quadrant> {
        match self.position {
            (x,y) if x < 0 && y < 0 => Some(Quadrant::One),
            (x,y) if x > 0 && y < 0 => Some(Quadrant::Two),
            (x,y) if x < 0 && y > 0 => Some(Quadrant::Three),
            (x,y) if x > 0 && y > 0 => Some(Quadrant::Four),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
struct Floor {
    robots: Vec<Robot>,
    quad_size: Pair
}

impl Floor {
    fn new(lines: &Vec<String>, quad_size: Pair) -> Floor {
        let mut ret = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            ret.push(Robot::new(trim_line, &quad_size));
        }
        Floor{ robots: ret, quad_size: quad_size }
    }

    fn move_robots(&mut self, n: usize) {
        for _ in 0..n {
            for robot in self.robots.iter_mut() {
                robot.make_move(&self.quad_size);
            }
        }
    }

    fn safety_factor(&self) -> Num {
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for robot in self.robots.iter() {
            match robot.quadrant() {
                Some(Quadrant::One) => {
                    q1 += 1;
                },
                Some(Quadrant::Two) => {
                    q2 += 1;
                },
                Some(Quadrant::Three) => {
                    q3 += 1;
                },
                Some(Quadrant::Four) => {
                    q4 += 1;
                },
                _ => {}
            };
        }
        println!("q1={} q2={} q3={} q4={}", q1, q2, q3, q4);
        q1 * q2 * q3 * q4
    }

    fn print(&self) {
        println!("[");
        for robot in self.robots.iter() {
            println!("    {:?}", robot);
        }
        println!("]");
    }
}

fn main() {
    let lines = read_lines("input");
    let quad_size: Pair = (50, 51);
    let mut f = Floor::new(&lines, quad_size);
    f.move_robots(100);
    println!("factor: {}", f.safety_factor());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
".lines().map(String::from).collect();

        let quad_size: Pair = (5,3);
        let mut f = Floor::new(&sample, quad_size);
        f.print();
        assert_eq!(f.safety_factor(), 4*0*2*2);

        f.move_robots(1);
        f.print();
        f.move_robots(1);
        f.print();
        f.move_robots(98);
        assert_eq!(f.safety_factor(), 12);
    }
}