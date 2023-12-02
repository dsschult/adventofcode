use std::fmt;

fn common_factors(num: usize, num2: usize) -> Vec<usize> {
    let mut factors = Vec::new(); // creates a new vector for the factors of the number
 
    let smaller = match num < num2 {
        true => num,
        false => num2,
    };
    for i in 2..=smaller { 
        if num % i == 0 && num2 % i == 0 {
            factors.push(i);
        }
    }
    factors
}

#[derive(Debug, Clone)]
struct Chemical {
    quantity: usize,
    name: String,
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Chemical>,
    outputs: Vec<Chemical>,
    factor: usize,
}

#[derive(Debug, Clone)]
struct Nanofactory {
    reactions: Vec<Reaction>,
}

impl Chemical {
    fn from(n: &str) -> Chemical {
        Chemical{quantity: 1, name: n.to_string()}
    }
}

impl PartialEq for Chemical {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.name)
    }
}

impl Reaction {
    fn from(input: &str) -> Reaction {
        let sides: Vec<&str> = input.split("=>").collect();
        assert_eq!(sides.len(), 2);
        let products = |chemicals: &str|{
            let mut ret = Vec::new();
            for c in chemicals.split(',') {
                let parts: Vec<&str> = c.trim().split(' ').collect();
                assert_eq!(parts.len(), 2);
                ret.push(Chemical{
                    quantity: parts[0].parse::<usize>().unwrap(),
                    name: parts[1].to_string()
                });
            }
            ret
        };
        let inputs = products(sides[0]);
        let outputs = products(sides[1]);
        Reaction{inputs: inputs, outputs: outputs, factor: 1}
    }
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.inputs.len() > 1 {
            for i in 0..self.inputs.len()-1 {
                write!(f, "{}, ", self.inputs[i])?;
            }
        }
        if !self.inputs.is_empty() {
            write!(f, "{}", self.inputs.last().unwrap())?;
        }
        write!(f, " -> ")?;
        if self.outputs.len() > 1 {
            for i in 0..self.outputs.len()-1 {
                write!(f, "{}, ", self.outputs[i])?;
            }
        }
        if !self.outputs.is_empty() {
            write!(f, "{}", self.outputs.last().unwrap())?;
        }
        write!(f, " ; factor: {}", self.factor)
    }
}

impl Nanofactory {
    fn from(input: &str) -> Nanofactory {
        let mut reactions = Vec::new();
        for line in input.lines() {
            let trim_line = line.trim();
            if trim_line.len() < 1 { continue; }
            reactions.push(Reaction::from(trim_line));
        }
        Nanofactory{reactions: reactions}
    }

    fn balance_for_output(&mut self, output: Chemical) -> bool {
        println!("Balancing {}", output);
        let mut change = true;
        let mut any_change = false;
        while change {
            change = false;
            if self.count_outputs(output.clone()) < output.quantity as i32 {
                for reaction in self.reactions.iter_mut() {
                    if reaction.outputs.contains(&output) {
                        println!("BfO {}: increasing {}", output, reaction);
                        reaction.factor += 1;
                        change = true;
                    }
                }
            }
            let mut inputs: Vec<Chemical> = Vec::new();
            for reaction in self.reactions.iter() {
                if reaction.outputs.contains(&output) {
                    for c in reaction.inputs.iter() {
                        let mut add = true;
                        for i in inputs.iter() {
                            if c == i {
                                add = false;
                                break
                            }
                        }
                        if add {
                            inputs.push(Chemical{quantity: 0, name: c.name.clone()});
                        }
                    }
                }
            }
            println!("inputs: {:?}", inputs);
            for c in inputs {
                change |= self.balance_for_output(c);
            }
            if change {
                any_change = true;
            }
        }
        change
    }

    fn count_inputs(&self, input: Chemical) -> i32 {
        let mut ret = 0;
        for reaction in self.reactions.iter() {
            for c in reaction.inputs.iter() {
                if *c == input {
                    println!("input: {:?}", c);
                    ret += (c.quantity * reaction.factor) as i32;
                }
            }
            for c in reaction.outputs.iter() {
                if *c == input {
                    println!("output: {:?}", c);
                    ret -= (c.quantity * reaction.factor) as i32;
                }
            }
        }
        ret
    }

    fn count_outputs(&self, input: Chemical) -> i32 {
        let mut ret = 0;
        for reaction in self.reactions.iter() {
            for c in reaction.inputs.iter() {
                if *c == input {
                    ret -= (c.quantity * reaction.factor) as i32;
                }
            }
            for c in reaction.outputs.iter() {
                if *c == input {
                    ret += (c.quantity * reaction.factor) as i32;
                }
            }
        }
        ret
    }
}

impl fmt::Display for Nanofactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for reaction in self.reactions.iter() {
            write!(f, "{}\n", reaction)?;
        }
        write!(f, "")
    }
}

fn main() {
    let input = "
1 FVBHS, 29 HWPND => 4 CPXDX
5 TNWDG, 69 VZMS, 1 GXSD, 48 NCLZ, 3 RSRZ, 15 HWPND, 25 SGPK, 2 SVCQ => 1 FUEL
1 PQRLB, 1 TWPMQ => 4 QBXC
9 QBXC => 7 RNHQ
12 VZMS => 6 MGQRZ
6 QBVG, 10 XJWX => 6 BWLZ
4 MVGN => 6 BHZH
2 LKTWD => 7 FVBHS
2 BWFK => 7 TFPQ
15 VZBJ, 9 TSVN, 2 BWLZ => 2 TNWDG
10 KVFL, 2 BWLZ, 1 VGSBF => 4 KBFJV
12 TXCR, 2 JMBG => 4 DCFD
5 VMDT, 6 JKPFT, 3 RJKJD => 7 LGWM
1 LDFGW => 2 DHRBP
129 ORE => 8 LDFGW
9 DNVRJ => 8 BMNGX
7 NLPB => 6 NCLZ
1 VMDT, 6 DCFD => 9 SGRXC
1 LDFGW, 2 VRHFB => 8 QHGQC
10 VGSBF, 5 WVMG, 6 BWLZ => 3 BWFK
4 KVFL, 1 TSVN => 6 SVCQ
2 VZBJ, 3 SWJZ => 3 QZLC
5 JMBG, 1 PQRLB => 3 CJLH
13 LKTWD, 6 TFPQ => 3 WVRXR
20 QHGQC, 10 NSPVD => 5 VGSBF
5 TFPQ, 1 DHRBP, 2 KVFL => 8 NLPB
2 KBFJV, 1 CJLH, 20 RNHQ, 1 BWLZ, 13 MNBK, 1 BHZH, 1 PKRJF => 8 RSRZ
154 ORE => 2 VRHFB
2 NHRCK => 7 DNVRJ
2 VRHFB, 4 XJWX => 4 NHRCK
1 TFPQ, 12 JMBG => 5 MNBK
8 TMFS => 2 VZMS
175 ORE => 2 TMFS
1 LBZN, 2 SWJZ, 3 VGSBF => 8 BLDN
7 KFJD, 5 WVRXR, 5 RJKJD => 6 MVGN
3 RJKJD, 1 TXCR => 8 KVFL
3 QHGQC, 1 MGQRZ, 10 VGSBF => 8 LKTWD
178 ORE => 1 XJWX
1 QBXC, 1 BWFK => 6 TSVN
1 NHRCK, 2 DHRBP => 4 VZBJ
1 LDFGW, 2 NHRCK, 10 BWLZ => 8 TWPMQ
28 TWPMQ => 4 RJKJD
10 SVCQ, 1 KVFL => 6 CZNMG
3 VZMS, 3 MGQRZ => 3 WVMG
19 MGQRZ => 8 KFJD
3 WVMG => 6 PQRLB
31 SVCQ, 1 TXCR => 8 VMDT
20 KFJD, 5 CPXDX, 2 BLDN, 2 PQWJX, 12 TFPQ, 2 BHZH, 2 MVGN => 9 SGPK
7 QZLC => 8 JMBG
1 PQRLB => 1 HWPND
9 VMDT, 5 CZNMG, 3 CPXDX, 1 MVGN, 8 VSMTK, 2 SGRXC, 1 MNBK, 8 LGWM => 7 GXSD
2 NSPVD => 8 QBVG
20 CZNMG => 4 PQWJX
1 LDFGW => 4 NSPVD
16 KBFJV, 22 BLDN => 2 VSMTK
10 BWLZ => 9 LBZN
1 BWLZ => 3 SWJZ
1 HWPND => 9 TXCR
12 CJLH, 9 LGWM, 3 BHZH => 6 PKRJF
5 BMNGX => 7 JKPFT";
    
    let mut factory = Nanofactory::from(&input);
    factory.balance_for_output(Chemical::from("FUEL"));
    let ore = factory.count_inputs(Chemical::from("ORE"));
    println!("need {} ORE", ore);
}

mod tests {
    use super::*;

    #[test]
    fn test_common_factors() {
        assert_eq!(common_factors(20,30), vec![2,5,10]);
        assert_eq!(common_factors(10,5), vec![5]);
        assert_eq!(common_factors(3,3), vec![3]);
    }

    #[test]
    fn test_single_eq() {
        let input = "1 A, 2 B, 3 C => 2 D";
        let factory = Nanofactory::from(&input);
        assert_eq!(factory.count_inputs(Chemical::from("A")), 1);
        assert_eq!(factory.count_inputs(Chemical::from("B")), 2);
        assert_eq!(factory.count_inputs(Chemical::from("C")), 3);
        assert_eq!(factory.count_outputs(Chemical::from("D")), 2);
    }

    #[test]
    fn test_day14a() {
        let input = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let factory = Nanofactory::from(&input);
        assert_eq!(factory.count_inputs(Chemical::from("ORE")), 11);
        assert_eq!(factory.count_inputs(Chemical::from("A")), 7*4-10);
        assert_eq!(factory.count_inputs(Chemical::from("B")), 0);
        assert_eq!(factory.count_inputs(Chemical::from("C")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("D")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("E")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("FUEL")), 1);
    }

    #[test]
    fn test_day14b() {
        let input = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let mut factory = Nanofactory::from(&input);
        factory.balance_for_output(Chemical::from("FUEL"));
        println!("factory out state: \n{}", factory);
        assert_eq!(factory.count_inputs(Chemical::from("ORE")), 31);
        assert_eq!(factory.count_outputs(Chemical::from("A")), 2);
        assert_eq!(factory.count_outputs(Chemical::from("B")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("C")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("D")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("E")), 0);
        assert_eq!(factory.count_outputs(Chemical::from("FUEL")), 1);
    }

    #[test]
    fn test_day14c() {
        let input = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let mut factory = Nanofactory::from(&input);
        factory.balance_for_output(Chemical::from("FUEL"));
        println!("factory out state: \n{}", factory);
        assert_eq!(factory.count_inputs(Chemical::from("ORE")), 165);
        assert_eq!(factory.count_outputs(Chemical::from("FUEL")), 1);
    }

    #[test]
    fn test_day14d() {
        let input = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let mut factory = Nanofactory::from(&input);
        factory.balance_for_output(Chemical::from("FUEL"));
        println!("factory out state: \n{}", factory);
        assert_eq!(factory.count_inputs(Chemical::from("ORE")), 13312);
        assert_eq!(factory.count_outputs(Chemical::from("FUEL")), 1);
    }

    #[test]
    fn test_day14e() {
        let input = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let mut factory = Nanofactory::from(&input);
        factory.balance_for_output(Chemical::from("FUEL"));
        println!("factory out state: \n{}", factory);
        assert_eq!(factory.count_inputs(Chemical::from("ORE")), 180697);
        assert_eq!(factory.count_outputs(Chemical::from("FUEL")), 1);
    }

    #[test]
    fn test_day14f() {
        let input = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let mut factory = Nanofactory::from(&input);
        factory.balance_for_output(Chemical::from("FUEL"));
        println!("factory out state: \n{}", factory);
        assert_eq!(factory.count_inputs(Chemical::from("ORE")), 2210736);
        assert_eq!(factory.count_outputs(Chemical::from("FUEL")), 1);
    }
}