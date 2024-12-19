use std::fs::read_to_string;
use std::collections::HashMap;
use eqsolver::multivariable::MultiVarNewtonFD;
use eqsolver::multivariable::GaussNewtonFD;
use nalgebra::{vector, Vector3};
use num_bigint::{BigInt, ToBigInt};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = BigInt;

type Pair = (Num, Num);

#[derive(Debug, Clone)]
struct Machine {
    button_a: Pair,
    button_b: Pair,
    prize: Pair
}

impl Machine {
    fn new(lines: Vec<&str>) -> Machine {
        let mut a = None;
        let mut b = None;
        let mut prize = None;
        let mut bigprize = 1000000.to_bigint().unwrap();
        bigprize *= 10000000.to_bigint().unwrap();
        for line in lines {
            let parts1 = line.split(':').collect::<Vec<_>>();
            let parts2 = parts1[1].split(',').collect::<Vec<_>>();
            match parts1[0] {
                "Button A" => {
                    let x = parts2[0].trim()[1..].parse::<Num>().unwrap();
                    let y = parts2[1].trim()[1..].parse::<Num>().unwrap();
                    a = Some((x,y));
                },
                "Button B" => {
                    let x = parts2[0].trim()[1..].parse::<Num>().unwrap();
                    let y = parts2[1].trim()[1..].parse::<Num>().unwrap();
                    b = Some((x,y));
                },
                "Prize" => {
                    let x = parts2[0].trim()[2..].parse::<Num>().unwrap();
                    let y = parts2[1].trim()[2..].parse::<Num>().unwrap();
                    prize = Some((x+bigprize.clone(), y+bigprize.clone()));
                },
                _ => { panic!("bad machine label"); }
            };
        }
        Machine{ button_a: a.unwrap(), button_b: b.unwrap(), prize: prize.unwrap() }
    }

    fn min_tokens(&self) -> Option<Num> {

        let THREE = 3.to_bigint().unwrap();
        let numerator = (self.button_a.0.clone() - self.button_b.0.clone() * THREE.clone()) * self.prize.1.clone() - (self.button_a.1.clone() - self.button_b.1.clone() * THREE.clone()) * self.prize.0.clone();
        let denom = (self.button_a.0.clone() - self.button_b.0.clone() * THREE.clone()) * self.button_b.1.clone() - (self.button_a.1.clone() - self.button_b.1.clone() * THREE.clone()) * self.button_b.0.clone();

        let t = numerator.clone() / denom.clone();
        let r = numerator % denom;
        if r != BigInt::ZERO {
            println!("solution: None");
            None
        } else {
            println!("solution: {:?}", t);
            Some(t)
        }
        /*println!("solution: {:?}", solution);
        println!("solution: {:?}", solution2);

        

        let frac = solution[2].fract();
        let num_buttons = solution[0] + solution[1];
        if frac < 0.00001 {
            Some(solution[2] as Num)
        } else if frac > 0.999999 {
            Some(solution[2].ceil() as Num)
        } else {
            None
        }*/
    }
}

#[derive(Debug, Clone)]
struct Machines {
    data: Vec<Machine>
}

impl Machines {
    fn new(lines: &Vec<String>) -> Machines {
        let mut ret = Vec::new();
        let mut machine = Vec::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                match machine.len() {
                    0 => { },
                    3 => {
                        ret.push(Machine::new(machine));
                        machine = Vec::new();
                    },
                    _ => { panic!("1 or 2 lines of machine!"); }
                };
                continue;
            }
            machine.push(trim_line);
        }
        match machine.len() {
            0 => { },
            3 => {
                ret.push(Machine::new(machine));
                machine = Vec::new();
            },
            _ => { panic!("1 or 2 lines of machine!"); }
        };
        Machines{ data: ret }
    }

    fn min_tokens(&self) -> Num {
        let mut ret = BigInt::ZERO;
        for machine in self.data.iter() {
            match machine.min_tokens() {
                None => { },
                Some(x) => {
                    ret += x;
                }
            };
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let m = Machines::new(&lines);
    println!("tokens: {}", m.min_tokens());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
".lines().map(String::from).collect();

        let machines = Machines::new(&sample);
        assert_eq!(machines.data.len(), 4);

        assert_eq!(machines.min_tokens(), 480.to_bigint().unwrap());
    }
}