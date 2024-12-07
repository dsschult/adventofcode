use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug, Clone)]
struct Eq {
    test_val: i64,
    numbers: Vec<i64>
}

impl Eq {
    fn eval(&self, ops: &Vec<char>) -> i64 {
        // false is +, true is *
        let mut num_it = self.numbers.iter();
        let mut ret = *num_it.next().unwrap();
        for (op, num) in ops.iter().zip(num_it) {
            ret = match op {
                '+' => ret.checked_add(*num).expect("overflow!"),
                '*' => ret.checked_mul(*num).expect("overflow!"),
                _ => panic!("bad operator"),
            }
        }
        ret
    }

    fn is_valid(&self) -> bool {
        println!("processing {}: {:?}", self.test_val, self.numbers);
        let max_op = self.numbers.len()-1;
        let mut queue = vec![Vec::new()];
        while !queue.is_empty() {
            let mut ops = queue.pop().unwrap();
            // try with either + or *
            let mut ops2 = ops.clone();
            ops.push('+');
            ops2.push('*');

            let val = self.eval(&ops);
            if ops.len() == max_op {
                if val == self.test_val {
                    println!("valid: {:?}", ops);
                    return true;
                }
            } else {
                if val <= self.test_val {
                    queue.push(ops);
                }
            }

            let val2 = self.eval(&ops2);
            if ops2.len() == max_op {
                if val2 == self.test_val {
                    println!("valid: {:?}", ops2);
                    return true;
                }
            } else {
                if val2 <= self.test_val {
                    queue.push(ops2);
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Calibrations {
    lines: Vec<Eq>,
}

impl Calibrations {
    fn new(lines: &Vec<String>) -> Calibrations {
        let mut ret = Vec::new();
        for line in lines.iter() {
            if line.trim().is_empty() {
                continue;
            }
            let (test, nums) = line.split_once(':').unwrap();
            println!("{}, {}", test, nums);
            ret.push(Eq{
                test_val: test.parse::<i64>().unwrap(),
                numbers: nums.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()
            });
        }
        Calibrations{ lines: ret }
    }

    fn total_valid_calibration(&self) -> i64 {
        self.lines.iter().filter(|x| x.is_valid()).fold(0, |ret, x| ret.checked_add(x.test_val).expect("overflow!"))
    }
}

fn main() {
    let lines = read_lines("input");
    let c = Calibrations::new(&lines);
    println!("valid: {}", c.total_valid_calibration());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
".lines().map(String::from).collect();

        let c = Calibrations::new(&sample);
        assert_eq!(c.total_valid_calibration(), 3749);
    }
}