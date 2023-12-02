use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[derive(Debug, Clone, Copy)]
struct Choice {
    red: u8,
    green: u8,
    blue: u8
}

impl Choice {
    fn new(line: &str) -> Choice {
        let mut ret = Choice{ red: 0, green: 0, blue: 0 };
        for part in line.split(",") {
            let p: Vec<&str> = part.trim().split(" ").collect();
            let num = p[0].parse::<u8>().unwrap();
            //println!("color: {} num: {}", p[1], num);
            match p[1] {
                "red" => { ret.red = num; },
                "green" => { ret.green = num; },
                "blue" => { ret.blue = num; },
                _ => panic!("unknown color")
            };
        }
        ret
    }
}


#[derive(Debug)]
struct Game {
    gameid: u8,
    rounds: Vec<Choice>
}

impl Game {
    fn new(line: &String) -> Game {
        let parts: Vec<&str> = line.trim().split(":").collect();
        let id = parts[0].trim().split(" ").collect::<Vec<_>>()[1].parse::<u8>().unwrap();
        let rounds = parts[1].split(";").map(Choice::new).collect();
        println!("Rounds: {:?}", rounds);
        Game { gameid: id, rounds: rounds }
    }

    fn valid_bag(&self, bag: &Choice) -> bool {
        for c in self.rounds.iter() {
            if c.red > bag.red || c.green > bag.green || c.blue > bag.blue {
                return false
            }
        }
        true
    }
}

#[derive(Debug)]
struct GameSet {
    bag: Choice,
    games: Vec<Game>
}

impl GameSet {
    fn add(&mut self, g: Game) {
        self.games.push(g);
    }

    fn valid_games(&self, bag: &Choice) -> Vec<bool> {
        self.games.iter().map(|x| x.valid_bag(bag)).collect()
    }

    fn valid_games_sum(&self, bag: &Choice) -> usize {
        let mut ret = 0;
        for game in self.games.iter() {
            if game.valid_bag(bag) {
                println!("game {} is valid", game.gameid);
                ret += game.gameid as usize;
            } else {
                println!("game {} is INvalid", game.gameid);
            }
        }
        ret
    }
}

fn calc(lines: &Vec<String>) -> GameSet {
    let mut g = GameSet { bag: Choice { red: 0, green: 0, blue: 0 }, games: Vec::new() };
    for line in lines {
        g.add(Game::new(line))
    }
    g
}


fn main() {
    let ret = calc(&read_lines("input"));
    let bag = Choice{red: 12, green: 13, blue: 14};
    let valid_games = ret.valid_games(&bag);
    println!("{}", ret.games.len());
    let mut valid = 0;
    for (i,v) in valid_games.into_iter().enumerate() {
        println!("game {} is {}", i+1, v);
        if v { valid += 1; }
    }
    println!("{}", valid);
    let sum = ret.valid_games_sum(&bag);
    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
".lines().map(String::from).collect();

        let ret = calc(&sample);
        println!("gameset: {:?}", ret);

        let bag = Choice{red: 12, green: 13, blue: 14};
        let valid_games = ret.valid_games(&bag);
        let expected = vec![true, true, false, false, true];
        assert!(valid_games == expected);
        assert!(ret.valid_games_sum(&bag) == 8);
    }
}