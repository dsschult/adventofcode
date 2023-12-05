use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug)]
struct Card {
    index: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

impl Card {
    fn new(line: &str) -> Card {
        let parts: Vec<_> = line.split(':').collect();
        let parts2: Vec<_> = parts[1].split('|').collect();
        let i = parts[0].trim().split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>()[1].parse::<u32>().unwrap();
        let mut w: Vec<u32> = parts2[0].trim().split(' ').filter(|s| !s.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect();
        let mut n: Vec<u32> = parts2[1].trim().split(' ').filter(|s| !s.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect();
        w.sort();
        n.sort();
        Card{ index: i, winning_numbers: w, numbers: n}
    }

    fn value(&self) -> u32 {
        let mut ret = 0;
        for n in self.numbers.iter() {
            for w in self.winning_numbers.iter() {
                if *n == *w {
                    if ret == 0 {
                        ret = 1;
                    } else {
                        ret *= 2;
                    }
                    break;
                }
            }
        }
        ret
    }

    fn matching_numbers(&self) -> u32 {
        let mut ret = 0;
        for n in self.numbers.iter() {
            for w in self.winning_numbers.iter() {
                if *n == *w {
                    ret += 1;
                    break;
                }
            }
        }
        ret
    }
}


#[derive(Debug)]
struct CardHolder {
    cards: Vec<Card>
}

impl CardHolder {
    fn new(lines: &Vec<String>) -> CardHolder {
        CardHolder{ cards: lines.iter().filter(|s| !s.is_empty()).map(|x| Card::new(&x.as_str())).collect() }
    }

    fn calc_card_count(&self) -> Vec<u32> {
        let mut copies: Vec<u32> = vec![1; self.cards.len()];
        for (i,c) in self.cards.iter().enumerate() {
            let matches = c.matching_numbers();
            for j in (i+1)..(i+matches as usize+1) {
                copies[j] += copies[i];
            }
        }
        copies
    }
}


fn main() {
    let lines = read_lines("input");

    let cards = CardHolder::new(&lines);
    let copies = cards.calc_card_count();
    let total = copies.iter().map(|x| *x).reduce(|a,b| a+b).unwrap();
    println!("total cards: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
".lines().map(String::from).collect();

        let c1 = Card::new(&sample[0].as_str());
        assert_eq!(c1.index, 1);
        let mut expected = vec![41,48,83,86,17];
        expected.sort();
        assert_eq!(c1.winning_numbers, expected);
        assert_eq!(c1.matching_numbers(), 4);
        
        let cards = CardHolder::new(&sample);
        let copies = cards.calc_card_count();
        assert_eq!(copies, vec![1, 2, 4, 8, 14, 1]);
        let total = copies.iter().map(|x| *x).reduce(|a,b| a+b).unwrap();
        assert_eq!(total, 30);
    }
}