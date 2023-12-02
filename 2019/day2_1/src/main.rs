

fn run(instrs: &mut Vec<u32>) -> u32 {
    let mut i = 0;
    while i < instrs.len() {
        let instr = instrs[i];
        match instr {
            1 => {
                let pos1 = instrs[i+1] as usize;
                let pos2 = instrs[i+2] as usize;
                let pos3 = instrs[i+3] as usize;
                println!("adding {} + {}", instrs[pos1], instrs[pos2]);
                instrs[pos3] = instrs[pos1] + instrs[pos2];
                println!("\t = {}", instrs[pos3]);
            },
            2 => {
                let pos1 = instrs[i+1] as usize;
                let pos2 = instrs[i+2] as usize;
                let pos3 = instrs[i+3] as usize;
                println!("multplying {} + {}", instrs[pos1], instrs[pos2]);
                instrs[pos3] = instrs[pos1] * instrs[pos2];
                println!("\t = {}", instrs[pos3]);
            },
            99 => break,
            _ => panic!("unknown instr: {}", instr),
        };
        i += 4;
    }
    if i >= instrs.len() {
        panic!("hit end")
    }
    instrs[0]
}

fn str_to_vec(input: &str) -> Vec<u32> {

    let mut instrs = Vec::new();
    for e in input.split(",") {
        instrs.push(e.parse::<u32>().unwrap());
    }

    instrs
}

fn main() -> () {
    let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,9,23,1,23,13,27,1,10,27,31,2,31,13,35,1,10,35,39,2,9,39,43,2,43,9,47,1,6,47,51,1,10,51,55,2,55,13,59,1,59,10,63,2,63,13,67,2,67,9,71,1,6,71,75,2,75,9,79,1,79,5,83,2,83,13,87,1,9,87,91,1,13,91,95,1,2,95,99,1,99,6,0,99,2,14,0,0";

    let mut instrs = str_to_vec(input);
    
    // set alarm
    instrs[1] = 12;
    instrs[2] = 2;

    run(&mut instrs);

    println!("pos 0 = {}", instrs[0]);
}


mod tests {
    use super::*;

    #[test]
    fn test_str_to_vec() {
        let input = "1,2,3,4";
        assert_eq!(str_to_vec(input), vec![1,2,3,4]);
    }

    #[test]
    fn test_run() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let out = run(&mut str_to_vec(input));
        assert_eq!(out, 3500);
    }
}