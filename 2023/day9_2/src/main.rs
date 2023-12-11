use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn to_values(line: &str) -> Vec<i64> {
    line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn derive(vals: &Vec<i64>) -> Vec<i64> {
    if vals.len() == 0 {
        vals.to_vec()
    } else {
        let mut ret = Vec::new();
        for i in 1..vals.len() {
            ret.push(vals[i] - vals[i-1]);
        }
        ret
    }
}

#[derive(Debug)]
struct DerivativeTree {
    values: Vec<Vec<i64>>
}

impl DerivativeTree {
    fn new(line: &str) -> Self {
        let mut ret = vec![to_values(line)];
        let mut r = derive(&ret[ret.len()-1]);
        while r.len() > 0 && !r.iter().all(|x| *x == 0) {
            ret.push(r);
            r = derive(&ret[ret.len()-1]);
        }
        ret.push(r);
        DerivativeTree{ values: ret }
    }

    fn extrapolate(&self) -> i64 {
        let mut val = 0;
        for level in (0..self.values.len()-1).rev() {
            let d = &self.values[level];
            val += d[d.len()-1];
        }
        val
    }

    fn extrapolate_and_add(&mut self) -> i64 {
        let mut val = 0;
        let mut new_values = self.values.to_vec();
        new_values[self.values.len()-1].push(0);
        for level in (0..self.values.len()-1).rev() {
            let d = &self.values[level];
            val += d[d.len()-1];
            new_values[level].push(val);
        }
        self.values = new_values;
        val
    }

    fn extrapolate_rev(&self) -> i64 {
        let mut val = 0;
        for level in (0..self.values.len()-1).rev() {
            let d = &self.values[level];
            val = d[0] - val;
        }
        val
    }

    fn extrapolate_rev_and_add(&mut self) -> i64 {
        let mut val = 0;
        let mut new_values = self.values.to_vec();
        new_values[self.values.len()-1].insert(0, 0);
        for level in (0..self.values.len()-1).rev() {
            let d = &self.values[level];
            val = d[0] - val;
            new_values[level].insert(0, val);
        }
        self.values = new_values;
        val
    }
}



fn main() {
    let lines = read_lines("input");
    let values2 = lines.iter().map(|x| DerivativeTree::new(x)).collect::<Vec<_>>();
    let sum = values2.iter().fold(0, |a, x| a + x.extrapolate_rev());
    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
".lines().map(String::from).collect();


        let mut values = sample.iter().map(|x| DerivativeTree::new(x)).collect::<Vec<_>>();
        println!("derivs: {:?}", values);

        let new_vals = values.iter_mut().map(|x| x.extrapolate_rev_and_add()).collect::<Vec<_>>();

        println!("derivs: {:?}", values);
        println!("new_vals: {:?}", new_vals);
        assert_eq!(new_vals, vec![-3, 0, 5]);
        
        let values2 = sample.iter().map(|x| DerivativeTree::new(x)).collect::<Vec<_>>();
        let sum = values2.iter().fold(0, |a, x| a + x.extrapolate_rev());
        println!("sum: {:?}", sum);
        assert_eq!(sum, 2);

    }
}
