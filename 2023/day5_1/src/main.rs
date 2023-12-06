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
struct Mapping {
    start: u32,
    mapping: u32,
    len: u32,
}

impl Mapping {
    fn new(line: &str) -> Self {
        let parts: Vec<_> = line.split(' ').filter(|s| !s.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect();
        Self{ start: parts[1], mapping: parts[0], len: parts[2] }
    }

    fn lookup(&self, n: u32) -> Option<u32> {
        if n >= self.start && n < self.start+self.len {
            Some(self.mapping + (n-self.start))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    fn new(lines: &Vec<String>) -> Self {
        let mut ret = Self {
            seeds: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new()
        };
        let mut section = None;
        let mut vals = Vec::new();
        for line in lines.iter() {
            if line.is_empty() {
                match section {
                    None => { },
                    Some("seed") => { ret.seed_to_soil = vals; println!("seed!"); },
                    Some("soil") => { ret.soil_to_fertilizer = vals; println!("soil!"); },
                    Some("fertilizer") => { ret.fertilizer_to_water = vals; println!("fert!"); },
                    Some("water") => { ret.water_to_light = vals; println!("water!"); },
                    Some("light") => { ret.light_to_temperature = vals; println!("light!"); },
                    Some("temperature") => { ret.temperature_to_humidity = vals; println!("temp!"); },
                    Some("humidity") => { ret.humidity_to_location = vals; println!("humid!"); },
                    _ => panic!("unknown section")
                }
                section = None;
                vals = Vec::new();
            }
            else if line.contains(':') {
                let parts: Vec<_> = line.split(':').collect();
                if parts[0] == "seeds" {
                    ret.seeds = parts[1].split(' ').filter(|s| !s.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()).collect();
                } else if parts[0].ends_with(" map") {
                    section = Some(parts[0].split(' ').collect::<Vec<_>>()[0].split('-').collect::<Vec<_>>()[0]);
                } else {
                    println!("line: {}", line);
                    println!("parts[0]: {}", parts[0]);
                    panic!("unknown line");
                }
            } else {
                // line is a range-start, range-start, len
                vals.push(Mapping::new(line));
            }
        }
        match section {
            None => { },
            Some("seed") => { ret.seed_to_soil = vals; },
            Some("soil") => { ret.soil_to_fertilizer = vals; },
            Some("fertilizer") => { ret.fertilizer_to_water = vals; },
            Some("water") => { ret.water_to_light = vals; },
            Some("light") => { ret.light_to_temperature = vals; },
            Some("temperature") => { ret.temperature_to_humidity = vals; },
            Some("humidity") => { ret.humidity_to_location = vals; },
            _ => panic!("unknown section")
        }
        ret
    }

    fn convert_seed_soil(&self, n: u32) -> u32 {
        for m in self.seed_to_soil.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_soil_fertilizer(&self, n: u32) -> u32 {
        for m in self.soil_to_fertilizer.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_fertilizer_water(&self, n: u32) -> u32 {
        for m in self.fertilizer_to_water.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_water_light(&self, n: u32) -> u32 {
        for m in self.water_to_light.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_light_temperature(&self, n: u32) -> u32 {
        for m in self.light_to_temperature.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_temperature_humidity(&self, n: u32) -> u32 {
        for m in self.temperature_to_humidity.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_humidity_location(&self, n: u32) -> u32 {
        for m in self.humidity_to_location.iter() {
            match m.lookup(n) {
                Some(x) => { return x; },
                None => { }
            };
        }
        n
    }

    fn convert_seed_location(&self, n: u32) -> u32 {
        self.convert_humidity_location(
            self.convert_temperature_humidity(
                self.convert_light_temperature(
                    self.convert_water_light(
                        self.convert_fertilizer_water(
                            self.convert_soil_fertilizer(
                                self.convert_seed_soil(n)
                            )
                        )
                    )
                )
            )
        )
    }

    fn get_seed_locations(&self) -> Vec<u32> {
        self.seeds.iter().map(|x| self.convert_seed_location(*x)).collect()
    }
}

fn main() {
    let lines = read_lines("input");

    let almanac = Almanac::new(&lines);
    let locs = almanac.get_seed_locations();
    println!("min location: {}", locs.iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample: Vec<String> = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
".lines().map(String::from).collect();

        let m = Mapping::new("52 50 48");
        println!("m={:?}", m);
        assert_eq!(m.lookup(49), None);
        assert_eq!(m.lookup(50), Some(52));
        assert_eq!(m.lookup(97), Some(99));
        assert_eq!(m.lookup(98), None);

        let almanac = Almanac::new(&sample);

        println!("SS: {:?}", almanac.seed_to_soil);
        assert_eq!(almanac.convert_seed_soil(1), 1);
        assert_eq!(almanac.convert_seed_soil(50), 52);
        assert_eq!(almanac.convert_seed_soil(97), 99);
        assert_eq!(almanac.convert_seed_soil(98), 50);
        assert_eq!(almanac.convert_seed_soil(99), 51);

        assert_eq!(almanac.convert_seed_location(79), 82);
        assert_eq!(almanac.convert_seed_location(14), 43);
        assert_eq!(almanac.convert_seed_location(55), 86);
        assert_eq!(almanac.convert_seed_location(13), 35);
        
    }
}
