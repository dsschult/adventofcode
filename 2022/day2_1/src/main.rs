use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Copy, Clone, PartialEq)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}


struct Round {
    player1: Throw,
    player2: Throw
}

impl Round {
    fn from_str(input: &str) -> Round {
        let parts: Vec<_> = input.split(" ").collect();
        let player1 = match parts[0] {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            _ => panic!("unknown throw")
        };
        let player2 = match parts[1] {
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Scissors,
            _ => panic!("unknown throw")
        };
        Round{player1: player1, player2: player2}
    }

    fn score(&self) -> u8 {
        let mut ret = self.player2 as u8;
        if self.player1 == self.player2 {
            ret += 3; // draw
        } else {
            ret += match (self.player1, self.player2) {
                (Throw::Rock, Throw::Paper) |
                (Throw::Paper, Throw::Scissors) |
                (Throw::Scissors, Throw::Rock) => 6, // win
                _ => 0, // loss
            };
        }
        ret
    }
}

struct RockPaperScissors {
    rounds: Vec<Round>
}

impl RockPaperScissors {
    fn new(lines: &Vec<String>) -> RockPaperScissors {
        let mut rounds = Vec::new();
        for line in lines.iter() {
            if line.trim().len() > 0 {
                rounds.push(Round::from_str(line));
            }
        }
        RockPaperScissors{rounds: rounds}
    }

    fn score(&self) -> u16 {
        self.rounds.iter().map(|x| x.score() as u16).reduce(|a,b| a+b).unwrap()
    }
}



fn main() {
    let game = RockPaperScissors::new(&read_lines("input"));
    println!("Total score: {}", game.score());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "A Y
B X
C Z
".lines().map(String::from).collect();

        let game = RockPaperScissors::new(&sample);
        assert!(game.rounds[0].score() == 8);
        assert!(game.rounds[1].score() == 1);
        assert!(game.rounds[2].score() == 6);
        assert!(game.score() == 15);
    }
}