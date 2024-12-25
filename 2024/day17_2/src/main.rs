use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type Num = usize;

#[derive(Debug, Clone)]
struct Computer {
    registerA: Num,
    registerB: Num,
    registerC: Num,
    program: Vec<Num>,
    instruction_pointer: usize,
    output: Vec<Num>,
}

impl Computer {
    fn new(lines: &Vec<String>) -> Computer {
        let mut reg_a = None;
        let mut reg_b = None;
        let mut reg_c = None;
        let mut now_program = false;
        let mut program = Vec::new();

        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                if reg_c.is_some() {
                    now_program = true;
                }
                continue;
            }
            let parts = trim_line.split(':').collect::<Vec<_>>();
            if now_program {
                program.extend(parts[1].trim().split(',').map(|x| x.parse::<Num>().unwrap()));
            } else {
                let v = parts[1].trim().parse::<Num>().unwrap();
                match parts[0] {
                    "Register A" => {
                        reg_a = Some(v);
                    },
                    "Register B" => {
                        reg_b = Some(v);
                    },
                    "Register C" => {
                        reg_c = Some(v);
                    },
                    _ => panic!("bad register")
                };  
            }
        }
        Computer{
            registerA: reg_a.unwrap(),
            registerB: reg_b.unwrap(),
            registerC: reg_c.unwrap(),
            program: program,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn get_combo_operand(&self, ptr: usize) -> Num {
        match self.program[ptr] {
            x if x >= 0 && x <= 3 => x,
            4 => self.registerA,
            5 => self.registerB,
            6 => self.registerC,
            7 => panic!("reserved"),
            _ => panic!("bad operand")
        }
    }

    fn process_instruction(&mut self, ptr: usize) -> usize {
        match self.program[ptr] {
            0 => {
                // adv - divisionA
                self.registerA = self.registerA / 2_u64.pow(self.get_combo_operand(ptr+1) as u32) as usize;
            },
            1 => {
                // bxl - bitwise xor
                self.registerB = self.registerB ^ self.program[ptr+1];
            },
            2 => {
                // bst - mod 8
                self.registerB = self.get_combo_operand(ptr+1) % 8;
            },
            3 => {
                // jnz - jump
                match self.registerA {
                    0 => { },
                    _ => {
                        return self.program[ptr+1] as usize;
                    }
                };
            },
            4 => {
                // bxc - bitwise xor B ^ C
                self.registerB = self.registerB ^ self.registerC;
            },
            5 => {
                // out - output
                self.output.push(self.get_combo_operand(ptr+1) % 8);
            },
            6 => {
                // bdv - division B
                self.registerB = self.registerA / 2_u64.pow(self.get_combo_operand(ptr+1) as u32) as usize;
            },
            7 => {
                // cdv - division B
                self.registerC = self.registerA / 2_u64.pow(self.get_combo_operand(ptr+1) as u32) as usize;
            },
            _ => panic!("unknown instruction")
        };
        ptr+2
    }

    fn run_program(&mut self) -> Vec<Num> {
        self.instruction_pointer = 0;
        self.output = Vec::new();
        let prog_len = self.program.len();

        while self.instruction_pointer < prog_len {
            //println!("running {}", self.instruction_pointer);
            self.instruction_pointer = self.process_instruction(self.instruction_pointer);
        }
        self.output.clone()
    }

    fn find_copy(&self) -> Num {
        for a in 0..usize::MAX {
            if a % 1000000 == 0 {
                println!("trying {}", a);
            }
            let mut c = self.clone();
            c.registerA = a;
            let out = c.run_program();
            if out == self.program {
                return a;
            }
        }
        panic!("no value A!");
    }
}

fn main() {
    let lines = read_lines("input");
    let mut c = Computer::new(&lines);
    let out = c.find_copy();
    println!("regA: {}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
".lines().map(String::from).collect();

        let mut c = Computer::new(&sample);
        let out = c.run_program();
        assert_eq!(out, vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn test_2() {
        let sample: Vec<String> = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
".lines().map(String::from).collect();

        let mut c = Computer::new(&sample);
        let out = c.find_copy();
        assert_eq!(out, 117440);
    }

}