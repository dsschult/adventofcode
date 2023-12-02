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

#[derive(Clone, Debug, PartialEq)]
struct PointSteps {
    p: Point,
    steps: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for PointSteps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.p, self.steps)
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

fn cross_path(p1: Vec<Point>, p2: Vec<Point>) -> Option<PointSteps> {
    let mut min_dist = 999999999;
    let mut ii = 0;
    for i in 1..p1.len() {
        for j in 1..p2.len() {
            if p1[i] == p2[j] {
                //let d = manhattan_distance(Point{x:0,y:0}, p1[i].clone());
                let steps = i + j;
                if steps < min_dist {
                    ii = i;
                    min_dist = steps;
                }
            }
        }
    }
    match ii {
        0 => None,
        x => Some(PointSteps{p:p1[x].clone(), steps:min_dist}),
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
            println!("distance: {}", manhattan_distance(Point{x:0,y:0}, cross.p));
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
        let line1 = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let line2 = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        let cross = cross_path(str_to_path(line1), str_to_path(line2));
        match cross {
            Some(x) => assert_eq!(x.steps, 610),
            None => panic!("no cross"),
        }
    }
}