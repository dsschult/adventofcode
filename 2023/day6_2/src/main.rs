use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_time_dist(lines: &Vec<String>) -> (u64, u64) {
    let time = lines[0].split(':').collect::<Vec<_>>()[1].chars().filter(|c| !c.is_whitespace()).collect::<String>().trim().parse::<u64>().unwrap();
    let distance = lines[1].split(':').collect::<Vec<_>>()[1].chars().filter(|c| !c.is_whitespace()).collect::<String>().trim().parse::<u64>().unwrap();

    (time, distance)
}

fn calc(speed: u64, total_time: u64) -> u64 {
    speed * (total_time - speed)
}

fn ways_to_beat_record(total_time: u64, record: u64) -> u64 {
    let mut ret = 0;
    for i in 0..total_time {
        if calc(i, total_time) > record {
            ret += 1
        }
    }
    ret
}

fn main() {
    let lines = read_lines("input");
    let (t,d) = get_time_dist(&lines);
    let records = ways_to_beat_record(t,d);
    println!("records: {}", records);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "Time:      7  15   30
Distance:  9  40  200
".lines().map(String::from).collect();

        let (t,d) = get_time_dist(&sample);
        assert_eq!(t, 71530);
        assert_eq!(d, 940200);

        assert_eq!(ways_to_beat_record(71530, 940200), 71503);
    }
}
