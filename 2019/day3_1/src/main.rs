use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

type Numeric = i32;

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: Numeric,
    y: Numeric,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn str_to_path(input: String) -> Vec<Point> {
    let mut points = vec![Point{x:0,y:0}];
    let mut last_point = Point{x:0,y:0};
    for e in input.split(",") {
        let num_str = e.get(1..).unwrap().trim_end();
        let num = match num_str.parse::<Numeric>() {
            Err(_) => panic!("bad digit: {}", num_str),
            Ok(n) => n,
        };
        for _ in 0..num {
            match e.chars().nth(0).unwrap() {
                'R' => last_point.x += 1,
                'L' => last_point.x -= 1,
                'U' => last_point.y += 1,
                'D' => last_point.y -= 1,
                d => panic!("bad direction: {}", d),
            }
            points.push(last_point.clone());
        }
    }
    points
}

fn manhattan_distance(p1: Point, p2: Point) -> Numeric {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn cross_path(p1: Vec<Point>, p2: Vec<Point>) -> Option<Point> {
    let mut min_dist:Numeric = 999999999;
    let mut ii = 0;
    for i in 1..p1.len() {
        for j in 1..p2.len() {
            if p1[i] == p2[j] {
                let d = manhattan_distance(Point{x:0,y:0}, p1[i].clone());
                if d < min_dist {
                    ii = i;
                    min_dist = d;
                }
            }
        }
    }
    match ii {
        0 => None,
        x => Some(p1[x].clone()),
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line1 = String::new();
    let mut line2 = String::new();
    reader.read_line(&mut line1)?;
    reader.read_line(&mut line2)?;
    match cross_path(str_to_path(line1), str_to_path(line2)) {
        Some(cross) => {
            println!("cross: {}", cross);
            println!("distance: {}", manhattan_distance(Point{x:0,y:0}, cross));
        },
        None => panic!("no crossing"),
    }
    
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_str_to_path() {
        let input = String::from("R1,U2,L3,D4");
        let p = vec![Point{x:0,y:0}, Point{x:1,y:0},
            Point{x:1,y:1}, Point{x:1,y:2},
            Point{x:0,y:2}, Point{x:-1,y:2}, Point{x:-2,y:2},
            Point{x:-2,y:1}, Point{x:-2,y:0}, Point{x:-2,y:-1}, Point{x:-2,y:-2}];
        assert_eq!(str_to_path(input), p);
    }
    
    #[test]
    fn test_manhattan_distance() {
        let p1 = Point{x: 0, y: 0};
        let p2 = Point{x: 3, y: 3};
        assert_eq!(manhattan_distance(p1, p2), 6);
    }

    #[test]
    fn test_cross_path() {
        let line1 = String::from("R8,U5,L5,D3");
        let line2 = String::from("U7,R6,D4,L4");
        let cross = cross_path(str_to_path(line1), str_to_path(line2));
        assert_eq!(cross, Some(Point{x:3,y:3}));
        let dist = manhattan_distance(Point{x:0,y:0}, cross.unwrap());
        assert_eq!(dist, 6);
    }
}