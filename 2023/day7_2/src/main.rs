use std::fs::read_to_string;
use counter::Counter;
use std::cmp::Ordering;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

const CARD_STRENGTH : &str = "J23456789TQKA";

fn card_strength(a: &char, b: &char) -> Ordering {
    match CARD_STRENGTH.find(*b) {
        Some(x) => x.cmp(&CARD_STRENGTH.find(*a).unwrap()),
        None => panic!("unknown card: {}", b)
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum HAND<'a> {
    FiveKind(&'a str) = 6,
    FourKind(&'a str) = 5,
    FullHouse(&'a str) = 4,
    ThreeKind(&'a str) = 3,
    TwoPair(&'a str) = 2,
    OnePair(&'a str) = 1,
    HighCard(&'a str) = 0,
}
impl HAND<'_> {
    fn new(cards: &str) -> HAND {
        let char_counts = cards.chars().filter(|x| *x != 'J').collect::<Counter<_>>().most_common_tiebreaker(card_strength);
        let jokers = cards.chars().filter(|x| *x == 'J').count();
        if jokers == 5 {
            // special case
            return HAND::FiveKind(cards);
        }
        //println!("char counts: {:?}", char_counts);
        match char_counts[0].1 + jokers {
            5 => HAND::FiveKind(cards),
            4 => HAND::FourKind(cards),
            3 => match char_counts[1] {
                (_, 2) => HAND::FullHouse(cards),
                _ => HAND::ThreeKind(cards),
            },
            2 => match char_counts[1] {
                (_, 2) => HAND::TwoPair(cards),
                _ => HAND::OnePair(cards)
            },
            1 => HAND::HighCard(cards),
            _ => panic!("unknown hand")
        }
    }

    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn cards(&self) -> &str {
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

impl Ord for HAND<'_> {
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

impl PartialOrd for HAND<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc(hand_bids: &Vec<(HAND, u64)>) -> u64 {
    hand_bids.iter().enumerate().fold(0, |prev, x| prev + (x.0 as u64 + 1) * x.1.1)
}

fn main() {
    let lines = read_lines("input");
    
    let mut hand_bids = lines.iter().map(|x| {
        let mut parts = x.split_whitespace();
        (HAND::new(parts.next().unwrap()), parts.next().unwrap().parse::<u64>().unwrap())
    }).collect::<Vec<_>>();
    
    hand_bids.sort_by(|a, b| a.0.cmp(&b.0));

    let winnings = calc(&hand_bids);
    println!("winnings: {}", winnings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(HAND::new("23345"), HAND::OnePair("23345"));
        assert_eq!(HAND::new("55555"), HAND::FiveKind("55555"));
        assert_eq!(HAND::new("55335"), HAND::FullHouse("55335"));
        assert_eq!(HAND::new("25335"), HAND::TwoPair("25335"));
        assert_eq!(HAND::new("23456"), HAND::HighCard("23456"));
        assert_eq!(HAND::new("55554"), HAND::FourKind("55554"));
        assert_eq!(HAND::new("T5554"), HAND::ThreeKind("T5554"));

        assert!(HAND::FiveKind("55555") > HAND::HighCard("A5432"));
        assert!(HAND::ThreeKind("55532") > HAND::TwoPair("8877A"));
        assert!(HAND::TwoPair("8844T") < HAND::TwoPair("8877K"));
        assert!(HAND::TwoPair("88T44") > HAND::TwoPair("8877K"));
        assert!(HAND::HighCard("87654") < HAND::HighCard("T9654"));
        assert!(HAND::HighCard("87532") > HAND::HighCard("86532"));
        assert!(HAND::HighCard("86543") == HAND::HighCard("86543"));

        // jokers
        assert_eq!(HAND::new("T555J"), HAND::FourKind("T555J"));
        assert_eq!(HAND::new("T5J5J"), HAND::FourKind("T5J5J"));



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
        
        assert_eq!(hand_bids[0], (HAND::new("32T3K"), 765));

        hand_bids.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(hand_bids[0], (HAND::new("32T3K"), 765));
        assert_eq!(hand_bids[1], (HAND::new("KK677"), 28));
        assert_eq!(hand_bids[2], (HAND::new("T55J5"), 684));
        assert_eq!(hand_bids[3], (HAND::new("QQQJA"), 483));
        assert_eq!(hand_bids[4], (HAND::new("KTJJT"), 220));

        let winnings = calc(&hand_bids);
        assert_eq!(winnings, 5905);
    }   
}
