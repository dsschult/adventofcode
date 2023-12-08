use std::fs::read_to_string;
use counter::Counter;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

const CARD_STRENGTH : &str = "23456789TJQKA";

fn card_strength(a: &char, b: &char) -> Ordering {
    match CARD_STRENGTH.find(*b) {
        Some(x) => x.cmp(&CARD_STRENGTH.find(*a).unwrap()),
        None => panic!("unknown card: {}", b)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum HAND {
    FiveKind(String) = 6,
    FourKind(String) = 5,
    FullHouse(String) = 4,
    ThreeKind(String) = 3,
    TwoPair(String) = 2,
    OnePair(String) = 1,
    HighCard(String) = 0,
}
impl HAND {
    fn new(cards: &str) -> HAND {
        let mut sorted_cards_vec = cards.chars().collect::<Vec<_>>();
        sorted_cards_vec.sort_by(card_strength);

        let char_counts = cards.chars().collect::<Counter<_>>().most_common_tiebreaker(card_strength);
        //println!("char counts: {:?}", char_counts);
        match char_counts[0] {
            (c, 5) => HAND::FiveKind(vec![c; 5].into_iter().collect::<String>()),
            (c, 4) => HAND::FourKind(vec![c, c, c, c, char_counts[1].0].into_iter().collect::<String>()),
            (c, 3) => match char_counts[1] {
                (c2, 2) => HAND::FullHouse(vec![c, c, c, c2, c2].into_iter().collect::<String>()),
                _ => {
                    let mut s = vec![c, c, c];
                    for c3 in sorted_cards_vec.into_iter() {
                        if c3 != c { s.push(c3); }
                    }
                    HAND::ThreeKind(s.into_iter().collect::<String>())
                },
            },
            (c, 2) => match char_counts[1] {
                (c2, 2) => {
                    let mut s = vec![c, c, c2, c2];
                    for c3 in sorted_cards_vec.into_iter() {
                        if c3 != c && c3 != c2 { s.push(c3); }
                    }
                    HAND::TwoPair(s.into_iter().collect::<String>())
                },
                _ => {
                    let mut s = vec![c, c];
                    for c3 in sorted_cards_vec.into_iter() {
                        if c3 != c { s.push(c3); }
                    }
                    HAND::OnePair(s.into_iter().collect::<String>())
                }
            },
            (_, 1) => HAND::HighCard(sorted_cards_vec.into_iter().collect::<String>()),
            _ => panic!("unknown hand")
        }
    }

    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn cards(&self) -> &String {
        match self {
            HAND::FiveKind(x) |
            HAND::FourKind(x) |
            HAND::FullHouse(x) |
            HAND::ThreeKind(x) |
            HAND::TwoPair(x) |
            HAND::OnePair(x) |
            HAND::HighCard(x) => x,
        }
    }
}

impl Ord for HAND {
    fn cmp(&self, other: &Self) -> Ordering {
        let sv = self.discriminant();
        let ov = other.discriminant();
        match sv.cmp(&ov) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in self.cards().chars().zip(other.cards().chars()) {
                    let s = card_strength(&b, &a);
                    if s != Ordering::Equal {
                        return s;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for HAND {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc(hand_bids: &Vec<(HAND, u64)>) -> u64 {
    hand_bids.iter().enumerate().fold(0, |prev, x| prev + (x.0 as u64 + 1) * x.1.1)
}

fn main() {
    let lines = read_lines("input");

    let mut str_bids = lines.iter().map(|x| {
        let mut parts = x.split_whitespace();
        let mut p0 = parts.next().unwrap().chars().collect::<Vec<_>>();
        p0.sort_by(card_strength);
        (p0.into_iter().collect::<String>(), parts.next().unwrap().parse::<u64>().unwrap())
    }).collect::<Vec<_>>();
    
    let mut hand_bids = lines.iter().map(|x| {
        let mut parts = x.split_whitespace();
        (HAND::new(parts.next().unwrap()), parts.next().unwrap().parse::<u64>().unwrap())
    }).collect::<Vec<_>>();
    
    let mut f = File::create("output").expect("Unable to create file");
    
    hand_bids.sort_by(|a, b| a.0.cmp(&b.0));
    for h in hand_bids.iter() {
        let mut m = false;
        for hh in str_bids.iter() {
            if h.1 == hh.1 {
                println!("match {:?} = {:?}", h.0.cards(), hh.0);
                m = true;
                break;
            }
        }
        if !m {
            println!("not found! {:?}", h);
        }
        f.write_all(h.0.cards().as_bytes()).expect("Unable to write data");
        f.write_all(" ".as_bytes()).expect("Unable to write data");
        f.write_all(h.1.to_string().as_bytes()).expect("Unable to write data");
        f.write_all("\n".as_bytes()).expect("Unable to write data");
    }
    

    let winnings = calc(&hand_bids);
    println!("winnings: {}", winnings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(HAND::new("23345"), HAND::OnePair("33542".to_string()));
        assert_eq!(HAND::new("55555"), HAND::FiveKind("55555".to_string()));
        assert_eq!(HAND::new("55335"), HAND::FullHouse("55533".to_string()));
        assert_eq!(HAND::new("25335"), HAND::TwoPair("55332".to_string()));
        assert_eq!(HAND::new("23456"), HAND::HighCard("65432".to_string()));
        assert_eq!(HAND::new("55554"), HAND::FourKind("55554".to_string()));
        assert_eq!(HAND::new("T5554"), HAND::ThreeKind("555T4".to_string()));

        assert!(HAND::FiveKind("55555".to_string()) > HAND::HighCard("A5432".to_string()));
        assert!(HAND::ThreeKind("55532".to_string()) > HAND::TwoPair("8877A".to_string()));
        assert!(HAND::TwoPair("8844T".to_string()) < HAND::TwoPair("8877K".to_string()));
        assert!(HAND::HighCard("87654".to_string()) < HAND::HighCard("T9654".to_string()));
        assert!(HAND::HighCard("87532".to_string()) > HAND::HighCard("86532".to_string()));
        assert!(HAND::HighCard("86543".to_string()) == HAND::HighCard("86543".to_string()));

        let sample: Vec<String> = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
".lines().map(String::from).collect();

        let hands = sample.iter().map(|x| HAND::new(x.split_whitespace().next().unwrap())).collect::<Vec<_>>();
        assert_eq!(hands[0], HAND::new("32T3K"));

        let mut hand_bids = sample.iter().map(|x| {
            let mut parts = x.split_whitespace();
            (HAND::new(parts.next().unwrap()), parts.next().unwrap().parse::<u64>().unwrap())
        }).collect::<Vec<_>>();
        
        assert_eq!(hand_bids[0], (HAND::new("33KT2"), 765));

        hand_bids.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(hand_bids[0], (HAND::new("33KT2"), 765));
        assert_eq!(hand_bids[1], (HAND::new("KTJJT"), 220));
        assert_eq!(hand_bids[2], (HAND::new("KK677"), 28));
        assert_eq!(hand_bids[3], (HAND::new("T55J5"), 684));
        assert_eq!(hand_bids[4], (HAND::new("QQQJA"), 483));

        let winnings = calc(&hand_bids);
        assert_eq!(winnings, 6440);
    }   
}
