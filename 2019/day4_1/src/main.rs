
fn adjacent(s: &String) -> bool {
    let mut prev = 'x';
    for c in s.chars() {
        if prev != 'x' && prev == c {
            return true
        }
        prev = c;
    }
    false
}

fn increase(s: &String) -> bool {
    let mut prev = 0;
    for n in s.chars() {
        let d = n.to_digit(10).unwrap();
        if d < prev {
            return false
        }
        prev = d;
    }
    true
}

fn valid(s: String) -> bool {
    s.len() == 6 && adjacent(&s) && increase(&s)
}

fn main() {
    let start = 134792;
    let end = 675810;
    let mut num_valid = 0;
    for n in start ..= end {
        if valid(n.to_string()) {
            num_valid += 1;
        }
    }
    println!("num valid: {}", num_valid);
}

mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        assert!(adjacent(&String::from("111111")));
        assert!(adjacent(&String::from("223450")));
        assert!(!adjacent(&String::from("123789")));
    }

    #[test]
    fn test_increase() {
        assert!(increase(&String::from("111111")));
        assert!(!increase(&String::from("223450")));
        assert!(increase(&String::from("123789")));
    }

    #[test]
    fn test_valid() {
        assert!(valid(String::from("111111")));
        assert!(!valid(String::from("223450")));
        assert!(!valid(String::from("123789")));
    }
}
