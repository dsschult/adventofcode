use itertools::Itertools;

type Numeric = i32;

fn get_modes(instr: Numeric) -> (Numeric,Numeric,Numeric) {
    (instr % 1000 / 100,
     instr % 10000 / 1000,
     instr % 100000 / 10000,
    )
}

struct IO {
    input:Vec<Numeric>,
    output:Vec<Numeric>,
}

impl IO {
    fn get_input(&mut self) -> Numeric {
        if self.input.is_empty() {
            panic!("no more input available");
        }
        let ret = *self.input.first().unwrap();
        self.input = self.input.split_off(1);
        ret
    }

    fn send_output(&mut self, out: Numeric) -> () {
        self.output.push(out);
    }
}

fn set_vec(v: &mut Vec<Numeric>, index: usize) -> &mut Numeric {
    if index >= v.len() {
        v.resize(index+1, 99);
    }
    &mut v[index]
}

fn three_instr<F>(instrs: &mut Vec<Numeric>, i: usize,
                  modes: (Numeric,Numeric,Numeric), op: F) -> usize where
    F: Fn(Numeric, Numeric) -> Numeric {
    let val1 = match modes.0 {
        0 => {
            assert!(instrs[i+1] >= 0);
            instrs[instrs[i+1] as usize]
        },
        1 => instrs[i+1],
        x => panic!("bad mode {}", x),
    };
    let val2 = match modes.1 {
        0 => {
            assert!(instrs[i+2] >= 0);
            instrs[instrs[i+2] as usize]
        },
        1 => instrs[i+2],
        x => panic!("bad mode {}", x),
    };
    match modes.2 {
        0 => {
            let pos3 = instrs[i+3] as usize;
            *set_vec(instrs,pos3) = op(val1, val2);
            println!("\t = {}", instrs[pos3]);
        },
        1 => panic!("cannot be immediate"),
        x => panic!("bad mode {}", x),
    };
    i + 4
}

fn jump_instr<F>(instrs: &mut Vec<Numeric>, i: usize,
                modes: (Numeric,Numeric,Numeric), op: F) -> usize where
    F: Fn(Numeric, Numeric) -> Option<Numeric> {
    let val1 = match modes.0 {
        0 => {
            assert!(instrs[i+1] >= 0);
            instrs[instrs[i+1] as usize]
        },
        1 => instrs[i+1],
        x => panic!("bad mode {}", x),
    };
    let val2 = match modes.1 {
        0 => {
            assert!(instrs[i+2] >= 0);
            instrs[instrs[i+2] as usize]
        },
        1 => instrs[i+2],
        x => panic!("bad mode {}", x),
    };
    match op(val1, val2) {
        Some(x) => x as usize,
        None => i + 3
    }
}

fn run(instrs: &mut Vec<Numeric>, io: &mut IO) -> Numeric {
    let mut i = 0;
    while i < instrs.len() {
        let instr = instrs[i];
        let modes = get_modes(instr);
        match instr%100 {
            1 => { // add
                i = three_instr(instrs, i, modes, |x,y| {
                    println!("adding {} + {}", x, y);
                    x + y
                });
            },
            2 => { // multiply
                i = three_instr(instrs, i, modes, |x,y| {
                    println!("multiplying {} * {}", x, y);
                    x * y
                });
            },
            3 => { // input
                println!("getting input");
                match modes.0 {
                    0 => {
                        let pos1 = instrs[i+1] as usize;
                        *set_vec(instrs,pos1) = io.get_input();
                        println!("\t = {}", instrs[pos1]);
                    },
                    1 => panic!("cannot be immediate"),
                    x => panic!("bad mode {}", x),
                };
                i += 2;
            },
            4 => { // output
                let val = match modes.0 {
                    0 => {
                        assert!(instrs[i+1] >= 0);
                        instrs[instrs[i+1] as usize]
                    },
                    1 => instrs[i+1],
                    x => panic!("bad mode {}", x),
                };
                println!("sending output = {}", val);
                io.send_output(val);
                i += 2;
            },
            5 => { // jump-if-true
                i = jump_instr(instrs, i, modes, |x,y| {
                    match x != 0 {
                        true => {
                            println!("jump-if-true {}", y);
                            assert!(y >= 0);
                            Some(y)
                        },
                        false => None
                    }
                });
            },
            6 => { // jump-if-false
                i = jump_instr(instrs, i, modes, |x,y| {
                    match x == 0 {
                        true => {
                            println!("jump-if-false {}", y);
                            assert!(y >= 0);
                            Some(y)
                        },
                        false => None
                    }
                });
            },
            7 => { // less-than
                i = three_instr(instrs, i, modes, |x,y| {
                    println!("less-than {} < {}", x, y);
                    match x < y { true => 1, false => 0 }
                });
            },
            8 => { // equals
                i = three_instr(instrs, i, modes, |x,y| {
                    println!("equals {} == {}", x, y);
                    match x == y { true => 1, false => 0 }
                });
            },
            99 => break,
            _ => panic!("unknown instr: {}", instr),
        };
    }
    if i >= instrs.len() {
        panic!("hit end")
    }
    instrs[0]
}

fn str_to_vec(input: &str) -> Vec<Numeric> {

    let mut instrs = Vec::new();
    for e in input.split(",") {
        instrs.push(e.parse::<Numeric>().unwrap());
    }

    instrs
}

fn get_thrust(instrs: &Vec<Numeric>, combinations: &Vec<i32>) -> i32 {
    let mut val = 0i32;
    for phase in combinations {
        let mut program = instrs.clone();
        let mut io = IO{input:vec![*phase, val], output:Vec::new()};
        run(&mut program, &mut io);
        if io.output.len() < 1 {
            panic!("no output for combination {:?}", combinations);
        }
        val = io.output[0];
    }
    val
}

fn main() -> () {
    let input = "3,8,1001,8,10,8,105,1,0,0,21,38,55,80,97,118,199,280,361,442,99999,3,9,101,2,9,9,1002,9,5,9,1001,9,4,9,4,9,99,3,9,101,5,9,9,102,2,9,9,1001,9,5,9,4,9,99,3,9,1001,9,4,9,102,5,9,9,101,4,9,9,102,4,9,9,1001,9,4,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,101,3,9,9,4,9,99,3,9,101,5,9,9,1002,9,2,9,101,3,9,9,1002,9,5,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99";
    let instrs = str_to_vec(input);

    let phase_combinations = (0i32..5i32).permutations(5);
    println!("all combinations {:?}", phase_combinations);
    let mut max_val = 0i32;
    let mut max_combination = Vec::new();
    for combination in phase_combinations {
        println!("checking combination {:?}", combination);
        let val = get_thrust(&instrs, &combination);
        if val > max_val {
            max_val = val;
            max_combination = combination;
        }
    }

    println!("max_val: {}", max_val);
    println!("phases: {:?}", max_combination);
}


mod tests {
    use super::*;

    #[test]
    fn test_get_thrust() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let instrs = str_to_vec(input);
        let combination = str_to_vec("4,3,2,1,0");
        let t = get_thrust(&instrs, &combination);
        assert_eq!(t, 43210);
    }

    #[test]
    fn test_get_thrust2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let instrs = str_to_vec(input);
        let combination = str_to_vec("0,1,2,3,4");
        let t = get_thrust(&instrs, &combination);
        assert_eq!(t, 54321);
    }

    #[test]
    fn test_get_thrust3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let instrs = str_to_vec(input);
        let combination = str_to_vec("1,0,4,3,2");
        let t = get_thrust(&instrs, &combination);
        assert_eq!(t, 65210);
    }

    #[test]
    fn test_str_to_vec() {
        let input = "1,2,3,4";
        assert_eq!(str_to_vec(input), vec![1,2,3,4]);
    }

    #[test]
    fn test_get_modes() {
        assert_eq!(get_modes(1002),(0,1,0));
    }

    #[test]
    fn test_set_vec() {
        let mut v = vec![1,2,3,4];
        *set_vec(&mut v, 100) = 10;
    }

    #[test]
    fn test_run_day2() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut v = str_to_vec(input);
        let mut io = IO{input:vec![1], output:Vec::new()};
        let out = run(&mut v, &mut io);
        assert_eq!(out, 3500);
    }

    #[test]
    fn test_run_day4() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut v = str_to_vec(input);
        let mut io = IO{input:vec![7], output:Vec::new()};
        run(&mut v, &mut io);
        assert_eq!(io.output[0], 999);
    }

    #[test]
    fn test_run_day4a() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut v = str_to_vec(input);
        let mut io = IO{input:vec![8], output:Vec::new()};
        run(&mut v, &mut io);
        assert_eq!(io.output[0], 1000);
    }

    #[test]
    fn test_run_day4b() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut v = str_to_vec(input);
        let mut io = IO{input:vec![9], output:Vec::new()};
        run(&mut v, &mut io);
        assert_eq!(io.output[0], 1001);
    }

    #[test]
    fn test_run_day5b() {
        let input = "3,225,1,225,6,6,1100,1,238,225,104,0,2,171,209,224,1001,224,-1040,224,4,224,102,8,223,223,1001,224,4,224,1,223,224,223,102,65,102,224,101,-3575,224,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,9,82,224,1001,224,-738,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1101,52,13,224,1001,224,-65,224,4,224,1002,223,8,223,1001,224,6,224,1,223,224,223,1102,82,55,225,1001,213,67,224,1001,224,-126,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1,217,202,224,1001,224,-68,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1002,176,17,224,101,-595,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1102,20,92,225,1102,80,35,225,101,21,205,224,1001,224,-84,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1101,91,45,225,1102,63,5,225,1101,52,58,225,1102,59,63,225,1101,23,14,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,1002,223,2,223,1006,224,329,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,344,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,359,1001,223,1,223,8,677,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1107,677,226,224,102,2,223,223,1006,224,389,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,419,1001,223,1,223,1007,677,677,224,102,2,223,223,1006,224,434,1001,223,1,223,107,226,226,224,1002,223,2,223,1005,224,449,1001,223,1,223,1008,677,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,677,226,224,1002,223,2,223,1005,224,479,1001,223,1,223,108,677,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,524,101,1,223,223,107,677,226,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,226,226,224,102,2,223,223,1005,224,554,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,569,101,1,223,223,108,677,226,224,102,2,223,223,1006,224,584,1001,223,1,223,7,677,677,224,1002,223,2,223,1005,224,599,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,614,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,1107,226,226,224,102,2,223,223,1005,224,644,1001,223,1,223,1108,677,677,224,1002,223,2,223,1005,224,659,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";
        let mut v = str_to_vec(input);
        let mut io = IO{input:vec![5], output:Vec::new()};
        run(&mut v, &mut io);
        assert_eq!(io.output[0], 3629692);
    }
}