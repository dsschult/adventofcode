use itertools::Itertools;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use futures::future::join_all;
use tokio_test::block_on;
use core::slice::Iter;

type Numeric = i64;

struct IO {
    tx: Option<mpsc::Sender<Numeric>>, // send on this channel
    rx: Option<mpsc::Receiver<Numeric>>, // receive on this channel
    history: Vec<Numeric>, // history of what was sent
}

impl IO {
    fn new() -> IO {
        let (s, r) = mpsc::channel(100);
        IO{tx: Some(s), rx: Some(r), history: Vec::new()}
    }

    async fn from(input: Numeric) -> IO {
        let (s, r) = mpsc::channel(100);
        let mut io = IO{tx: Some(s), rx: Some(r), history: Vec::new()};
        io.send_output(input).await;
        io
    }

    fn get_history_last(&self) -> Option<Numeric> {
        match self.history.last() {
            Some(x) => Some(*x),
            None => None,
        }
    }

    async fn get_input(&mut self) -> Numeric {
        match &mut self.rx {
            Some(r) => match r.recv().await {
                Some(x) => x,
                None => panic!("no more input available"),
            },
            None => panic!("input channel not available"),
        }
    }

    async fn send_output(&mut self, out: Numeric) -> () {
        self.history.push(out);
        match &mut self.tx {
            Some(t) => match t.send(out).await {
                Ok(_) => (),
                Err(_) => { println!("WARN: send_output failed"); },
            },
            None => panic!("output channel not available"),
        }
    }
}

struct IntcodeComp {
    name: usize, // name of computer
    instrs: Vec<Numeric>, // instructions / memory
    io: IO, // io
    ptr: usize, // current instruction pointer
    rel_base: Numeric, // relative base for mode 3
}

impl IntcodeComp {
    fn new(name: usize, instrs: Vec<Numeric>, io: IO) -> IntcodeComp {
        IntcodeComp{name: name, instrs: instrs, io: io, ptr: 0, rel_base: 0}
    }

    fn get_modes(instr: Numeric) -> (Numeric,Numeric,Numeric) {
        (instr % 1000 / 100,
         instr % 10000 / 1000,
         instr % 100000 / 10000,
        )
    }

    async fn run(&mut self) -> Option<Numeric> {
        let name = self.name.clone();
        while self.ptr < self.instrs.len() {
            let instr = self.instrs[self.ptr];
            let modes = IntcodeComp::get_modes(instr);
            println!("{}: instr pointer = {}, mode = {:?}", self.name, self.ptr, modes);
            println!("\t instrs: {:?}", self.instrs.get(self.ptr..self.ptr+4));
            match instr%100 {
                1 => { // add
                    self.ptr = self.three_instr(modes, |x,y| {
                        println!("{}: adding {} + {}", name, x, y);
                        x + y
                    });
                },
                2 => { // multiply
                    self.ptr = self.three_instr(modes, |x,y| {
                        println!("{}: multiplying {} * {}", name, x, y);
                        x * y
                    });
                },
                3 => { // input
                    println!("{}: getting input", name);
                    let val = self.io.get_input().await;
                    self.store(self.ptr+1, modes.0, val);
                    self.ptr += 2;
                },
                4 => { // output
                    let val = self.load(self.ptr+1, modes.0);
                    println!("{}: sending output = {}", name, val);
                    self.io.send_output(val).await;
                    self.ptr += 2;
                },
                5 => { // jump-if-true
                    self.ptr = self.jump_instr(modes, |x,y| {
                        match x != 0 {
                            true => {
                                println!("{}: jump-if-true {}", name, y);
                                assert!(y >= 0);
                                Some(y)
                            },
                            false => None
                        }
                    });
                },
                6 => { // jump-if-false
                    self.ptr = self.jump_instr(modes, |x,y| {
                        match x == 0 {
                            true => {
                                println!("{}: jump-if-false {}", name, y);
                                assert!(y >= 0);
                                Some(y)
                            },
                            false => None
                        }
                    });
                },
                7 => { // less-than
                    self.ptr = self.three_instr(modes, |x,y| {
                        println!("{}: less-than {} < {}", name, x, y);
                        match x < y { true => 1, false => 0 }
                    });
                },
                8 => { // equals
                    self.ptr = self.three_instr(modes, |x,y| {
                        println!("{}: equals {} == {}", name, x, y);
                        match x == y { true => 1, false => 0 }
                    });
                },
                9 => { // set relative base
                    self.rel_base += self.load(self.ptr+1, modes.0);
                    self.ptr += 2;
                },
                99 => break,
                _ => panic!("{}: unknown instr: {}", name, instr),
            };
        }
        if self.ptr >= self.instrs.len() {
            panic!("hit end")
        }
        println!("finishing {}", name);
        self.io.get_history_last()
    }
    
    fn set_mem(&mut self, index: usize, val: Numeric) -> () {
        if index >= self.instrs.len() {
            self.instrs.resize(index+1, 0);
        }
        self.instrs[index] = val;
    }

    fn get_mem(&mut self, index: usize) -> Numeric {
        if index >= self.instrs.len() {
            self.instrs.resize(index+1, 0);
        }
        self.instrs[index]        
    }

    fn load(&mut self, index: usize, mode: Numeric) -> Numeric {
        match mode {
            0 => {
                let pos = self.get_mem(index);
                assert!(pos >= 0);
                self.get_mem(pos as usize)
            },
            1 => self.get_mem(index),
            2 => {
                let pos = self.rel_base+self.get_mem(index);
                assert!(pos >= 0);
                self.get_mem(pos as usize)
            },
            x => panic!("bad mode {}", x),
        }
    }

    fn store(&mut self, index: usize, mode: Numeric, val: Numeric) -> () {
        match mode {
            0 => {
                let pos = self.get_mem(index);
                assert!(pos >= 0);
                self.set_mem(pos as usize, val);
                println!("\t [{}] = {}", pos, self.instrs[pos as usize]);
            },
            1 => panic!("store cannot be immediate"),
            2 => {
                let pos = self.rel_base+self.get_mem(index);
                assert!(pos >= 0);
                self.set_mem(pos as usize, val);
                println!("\t [{}] = {}", pos, self.instrs[pos as usize]);
            },
            x => panic!("bad mode {}", x),
        }
    }

    fn three_instr<F>(&mut self, modes: (Numeric,Numeric,Numeric), op: F) -> usize where
            F: Fn(Numeric, Numeric) -> Numeric {
        let val1 = self.load(self.ptr+1, modes.0);
        let val2 = self.load(self.ptr+2, modes.1);
        self.store(self.ptr+3, modes.2, op(val1, val2));
        self.ptr + 4
    }

    fn jump_instr<F>(&mut self, modes: (Numeric,Numeric,Numeric), op: F) -> usize where
            F: Fn(Numeric, Numeric) -> Option<Numeric> {
        let val1 = self.load(self.ptr+1, modes.0);
        let val2 = self.load(self.ptr+2, modes.1);
        match op(val1, val2) {
            Some(x) => x as usize,
            None => self.ptr + 3
        }
    }
}

fn str_to_vec(input: &str) -> Vec<Numeric> {

    let mut instrs = Vec::new();
    for e in input.split(",") {
        instrs.push(e.parse::<Numeric>().unwrap());
    }
    instrs
}

async fn get_thrust_impl(instrs: &Vec<Numeric>, combinations: &Vec<Numeric>) -> Option<Numeric> {
    let mut program_vec = Vec::new();
    for _ in combinations {
        program_vec.push(instrs.clone());
    }

    let mut io: Vec<IO> = Vec::new();
    let mut tx = None;
    for i in 0..combinations.len() {
        let (mut s, r) = mpsc::channel(2);
        match s.send(combinations[i]).await {
            Ok(_) => (),
            Err(e) => panic!("error setting combination: {}", e),
        };
        io.push(IO{tx: None, rx: Some(r), history: Vec::new()});
        if i == 0 {
            tx = Some(s);
        } else {
            io[i-1].tx = Some(s);
        }
    }
    match &mut tx {
        Some(t) => t.send(0).await.unwrap(),
        None => (),
    };
    io[combinations.len()-1].tx = tx;

    let names = 0..combinations.len();

    let mut computers = Vec::new();
    for (name, (prog, io)) in names.zip(program_vec.drain(..).zip(io.drain(..))) {
        computers.push(IntcodeComp::new(name, prog, io));
    }
    let mut fut = Vec::new();
    for c in computers.iter_mut() {
        fut.push(c.run());
    }
    // run and return the last program's result
    let val = join_all(fut).await;
    println!("programs finished");
    val[combinations.len()-1]
}

fn get_thrust(instrs: &Vec<Numeric>, combinations: &Vec<Numeric>) -> Option<Numeric> {
    let mut pool = Runtime::new().unwrap();
    pool.block_on(get_thrust_impl(instrs, combinations))
}

#[tokio::main]
async fn main() -> () {
    let input = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,0,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,396,1029,1101,0,356,1023,1101,401,0,1028,1101,24,0,1008,1101,33,0,1019,1101,35,0,1010,1102,359,1,1022,1102,32,1,1001,1101,37,0,1004,1101,0,31,1009,1101,0,30,1003,1101,28,0,1002,1102,1,36,1014,1102,20,1,1012,1101,21,0,1000,1101,0,22,1015,1102,23,1,1013,1102,1,1,1021,1102,1,39,1007,1102,26,1,1017,1101,0,38,1016,1101,0,437,1024,1102,432,1,1025,1101,0,421,1026,1101,0,29,1005,1101,27,0,1011,1102,1,0,1020,1101,0,25,1018,1101,0,414,1027,1102,34,1,1006,109,6,2108,33,-3,63,1005,63,201,1001,64,1,64,1105,1,203,4,187,1002,64,2,64,109,14,21108,40,40,-6,1005,1014,221,4,209,1105,1,225,1001,64,1,64,1002,64,2,64,109,-21,2102,1,3,63,1008,63,28,63,1005,63,251,4,231,1001,64,1,64,1106,0,251,1002,64,2,64,109,12,2101,0,-3,63,1008,63,21,63,1005,63,275,1001,64,1,64,1105,1,277,4,257,1002,64,2,64,109,-10,1207,1,27,63,1005,63,293,1105,1,299,4,283,1001,64,1,64,1002,64,2,64,109,9,21108,41,42,3,1005,1013,315,1105,1,321,4,305,1001,64,1,64,1002,64,2,64,109,-12,1202,6,1,63,1008,63,37,63,1005,63,347,4,327,1001,64,1,64,1105,1,347,1002,64,2,64,109,29,2105,1,-4,1105,1,365,4,353,1001,64,1,64,1002,64,2,64,109,-17,2108,32,-9,63,1005,63,387,4,371,1001,64,1,64,1105,1,387,1002,64,2,64,109,17,2106,0,1,4,393,1105,1,405,1001,64,1,64,1002,64,2,64,109,1,2106,0,-1,1001,64,1,64,1106,0,423,4,411,1002,64,2,64,109,-13,2105,1,9,4,429,1106,0,441,1001,64,1,64,1002,64,2,64,109,3,21107,42,41,-1,1005,1017,461,1001,64,1,64,1106,0,463,4,447,1002,64,2,64,109,-4,21107,43,44,1,1005,1015,481,4,469,1106,0,485,1001,64,1,64,1002,64,2,64,109,-6,21101,44,0,6,1008,1014,47,63,1005,63,505,1106,0,511,4,491,1001,64,1,64,1002,64,2,64,109,-6,1208,-1,32,63,1005,63,529,4,517,1105,1,533,1001,64,1,64,1002,64,2,64,109,11,1205,7,545,1106,0,551,4,539,1001,64,1,64,1002,64,2,64,109,11,21102,45,1,-7,1008,1017,48,63,1005,63,575,1001,64,1,64,1106,0,577,4,557,1002,64,2,64,109,-8,1206,5,593,1001,64,1,64,1105,1,595,4,583,1002,64,2,64,109,7,1206,-3,609,4,601,1106,0,613,1001,64,1,64,1002,64,2,64,109,-10,2101,0,-6,63,1008,63,39,63,1005,63,635,4,619,1106,0,639,1001,64,1,64,1002,64,2,64,109,-9,1208,0,39,63,1005,63,655,1106,0,661,4,645,1001,64,1,64,1002,64,2,64,109,4,2107,25,0,63,1005,63,681,1001,64,1,64,1105,1,683,4,667,1002,64,2,64,109,-5,2107,31,-2,63,1005,63,701,4,689,1106,0,705,1001,64,1,64,1002,64,2,64,109,19,1205,-1,719,4,711,1105,1,723,1001,64,1,64,1002,64,2,64,109,-17,1201,3,0,63,1008,63,24,63,1005,63,745,4,729,1106,0,749,1001,64,1,64,1002,64,2,64,109,13,21102,46,1,-3,1008,1015,46,63,1005,63,771,4,755,1105,1,775,1001,64,1,64,1002,64,2,64,109,-13,1207,4,32,63,1005,63,793,4,781,1106,0,797,1001,64,1,64,1002,64,2,64,109,7,2102,1,-9,63,1008,63,27,63,1005,63,821,1001,64,1,64,1105,1,823,4,803,1002,64,2,64,109,-18,1201,8,0,63,1008,63,25,63,1005,63,847,1001,64,1,64,1106,0,849,4,829,1002,64,2,64,109,23,21101,47,0,2,1008,1019,47,63,1005,63,871,4,855,1106,0,875,1001,64,1,64,1002,64,2,64,109,-22,1202,5,1,63,1008,63,19,63,1005,63,899,1001,64,1,64,1106,0,901,4,881,4,64,99,21102,27,1,1,21102,1,915,0,1105,1,922,21201,1,25165,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,942,1,0,1105,1,922,22102,1,1,-1,21201,-2,-3,1,21101,0,957,0,1105,1,922,22201,1,-1,-2,1106,0,968,21201,-2,0,-2,109,-3,2105,1,0";
    let v = str_to_vec(input);
    let io = IO::from(1).await;
    match IntcodeComp::new(0, v, io).run().await {
        Some(output) => println!("output: {}", output),
        None => println!("no output"),
    };
}


mod tests {
    use super::*;

    #[test]
    fn test_get_thrust() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let instrs = str_to_vec(input);
        let combination = str_to_vec("4,3,2,1,0");
        let t = get_thrust(&instrs, &combination);
        assert_eq!(t, Some(43210));
    }

    #[test]
    fn test_get_thrust2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let instrs = str_to_vec(input);
        let combination = str_to_vec("0,1,2,3,4");
        let t = get_thrust(&instrs, &combination);
        assert_eq!(t, Some(54321));
    }

    #[test]
    fn test_get_thrust3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let instrs = str_to_vec(input);
        let combination = str_to_vec("1,0,4,3,2");
        let t = get_thrust(&instrs, &combination);
        assert_eq!(t, Some(65210));
    }

    #[test]
    fn test_str_to_vec() {
        let input = "1,2,3,4";
        assert_eq!(str_to_vec(input), vec![1,2,3,4]);
    }

    #[test]
    fn test_get_modes() {
        assert_eq!(IntcodeComp::get_modes(1002),(0,1,0));
    }

    #[test]
    fn test_set_mem() {
        let v = vec![1,2,3,4];
        let io = IO::new();
        let mut comp = IntcodeComp::new(0, v, io);
        comp.set_mem(100, 10);
        assert_eq!(comp.instrs[100], 10);
        assert_eq!(comp.instrs[99], 0);
    }

    #[test]
    fn test_run_day4() {
        async fn t() {
            let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let v = str_to_vec(input);
            let io = IO::from(7).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 999),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day4a() {
        async fn t() {
            let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let v = str_to_vec(input);
            let io = IO::from(8).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 1000),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day4b() {
        async fn t() {
            let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let v = str_to_vec(input);
            let io = IO::from(9).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 1001),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day5_1a() {
        async fn t() {
            let input = "3,9,8,9,10,9,4,9,99,-1,8";
            let v = str_to_vec(input);
            let io = IO::from(8).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 1),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day5_1b() {
        async fn t() {
            let input = "3,9,8,9,10,9,4,9,99,-1,8";
            let v = str_to_vec(input);
            let io = IO::from(16).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 0),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day5_2a() {
        async fn t() {
            let input = "3,9,7,9,10,9,4,9,99,-1,8";
            let v = str_to_vec(input);
            let io = IO::from(5).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 1),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day5_2b() {
        async fn t() {
            let input = "3,9,7,9,10,9,4,9,99,-1,8";
            let v = str_to_vec(input);
            let io = IO::from(8).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 0),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day5b() {
        async fn t() {
            let input = "3,225,1,225,6,6,1100,1,238,225,104,0,2,171,209,224,1001,224,-1040,224,4,224,102,8,223,223,1001,224,4,224,1,223,224,223,102,65,102,224,101,-3575,224,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,9,82,224,1001,224,-738,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1101,52,13,224,1001,224,-65,224,4,224,1002,223,8,223,1001,224,6,224,1,223,224,223,1102,82,55,225,1001,213,67,224,1001,224,-126,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1,217,202,224,1001,224,-68,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1002,176,17,224,101,-595,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1102,20,92,225,1102,80,35,225,101,21,205,224,1001,224,-84,224,4,224,1002,223,8,223,1001,224,1,224,1,224,223,223,1101,91,45,225,1102,63,5,225,1101,52,58,225,1102,59,63,225,1101,23,14,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,1002,223,2,223,1006,224,329,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,344,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,359,1001,223,1,223,8,677,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1107,677,226,224,102,2,223,223,1006,224,389,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,419,1001,223,1,223,1007,677,677,224,102,2,223,223,1006,224,434,1001,223,1,223,107,226,226,224,1002,223,2,223,1005,224,449,1001,223,1,223,1008,677,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,677,226,224,1002,223,2,223,1005,224,479,1001,223,1,223,108,677,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,509,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,524,101,1,223,223,107,677,226,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,226,226,224,102,2,223,223,1005,224,554,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,569,101,1,223,223,108,677,226,224,102,2,223,223,1006,224,584,1001,223,1,223,7,677,677,224,1002,223,2,223,1005,224,599,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,614,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,629,101,1,223,223,1107,226,226,224,102,2,223,223,1005,224,644,1001,223,1,223,1108,677,677,224,1002,223,2,223,1005,224,659,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";
            let v = str_to_vec(input);
            let io = IO::from(5).await;
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 3629692),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day7b() {
        let input = "3,8,1001,8,10,8,105,1,0,0,21,38,55,80,97,118,199,280,361,442,99999,3,9,101,2,9,9,1002,9,5,9,1001,9,4,9,4,9,99,3,9,101,5,9,9,102,2,9,9,1001,9,5,9,4,9,99,3,9,1001,9,4,9,102,5,9,9,101,4,9,9,102,4,9,9,1001,9,4,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,101,3,9,9,4,9,99,3,9,101,5,9,9,1002,9,2,9,101,3,9,9,1002,9,5,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99";
        let instrs = str_to_vec(input);

        let phase_combinations = (5..10).permutations(5);
        println!("all combinations {:?}", phase_combinations);
        let mut max_val = 0;
        let mut max_combination = Vec::new();
        for combination in phase_combinations {
            println!("checking combination {:?}", combination);
            let val = get_thrust(&instrs, &combination).unwrap();
            if val > max_val {
                max_val = val;
                max_combination = combination;
            }
        }

        assert_eq!(max_val, 19581200);
        assert_eq!(max_combination, vec![8, 9, 5, 6, 7]);
    }

    #[test]
    fn test_run_day9a_1() {
        async fn t() {
            let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
            let v = str_to_vec(input);
            let v2 = v.clone();
            let io = IO::new();
            let mut comp = IntcodeComp::new(0, v, io);
            comp.run().await;
            assert_eq!(comp.io.history, v2);
        }
        block_on(t());
    }

    #[test]
    fn test_run_day9a_2() {
        async fn t() {
            let input = "1102,34915192,34915192,7,4,7,99,0";
            let v = str_to_vec(input);
            let io = IO::new();
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out.to_string().len(), 16),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }

    #[test]
    fn test_run_day9a_3() {
        async fn t() {
            let input = "104,1125899906842624,99";
            let v = str_to_vec(input);
            let io = IO::new();
            match IntcodeComp::new(0, v, io).run().await {
                Some(out) => assert_eq!(out, 1125899906842624),
                None => panic!("no output"),
            }
        }
        block_on(t());
    }
}