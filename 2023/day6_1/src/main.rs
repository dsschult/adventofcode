use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_time_dist(lines: &Vec<String>) -> HashMap<u32, u32> {
    let mut ret = HashMap::new();

    let times: Vec<_> = lines[0].split(':').collect::<Vec<_>>()[1].split(' ').filter(|s| !s.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect();
    let distances: Vec<_> = lines[1].split(':').collect::<Vec<_>>()[1].split(' ').filter(|s| !s.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect();

    assert_eq!(times.len(), distances.len());

    for (t,d) in times.iter().zip(distances.iter()) {
        ret.insert(*t, *d);
    }
    ret
}

fn calc(speed: u32, total_time: u32) -> u32 {
    speed * (total_time - speed)
}

fn ways_to_beat_record(total_time: u32, record: u32) -> u32 {
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
    let td = get_time_dist(&lines);
    let records = td.iter().map(|(t,d)| ways_to_beat_record(*t,*d)).reduce(|a,b| a*b).unwrap();
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

        let td = get_time_dist(&sample);
        assert_eq!(td.len(), 3);
        assert_eq!(td[&7], 9);
        assert_eq!(td[&15], 40);
        assert_eq!(td[&30], 200);

        assert_eq!(calc(0, 7), 0);
        assert_eq!(calc(1, 7), 6);
        assert_eq!(calc(2, 7), 10);
        assert_eq!(calc(3, 7), 12);
        assert_eq!(calc(4, 7), 12);
        assert_eq!(calc(5, 7), 10);
        assert_eq!(calc(6, 7), 6);
        assert_eq!(calc(7, 7), 0);

        assert_eq!(ways_to_beat_record(7, td[&7]), 4);
        assert_eq!(ways_to_beat_record(15, td[&15]), 8);
        assert_eq!(ways_to_beat_record(30, td[&30]), 9);
    }
}
