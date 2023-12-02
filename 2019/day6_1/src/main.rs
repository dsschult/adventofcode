use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

type Numeric = u32;
type OrbitTree = HashMap<String, Vec<String>>;

fn get_objects(line: String) -> (String, String) {
    let v: Vec<&str> = line.splitn(2, ")").collect();
    if v.len() < 2 {
        panic!("can't find objects for {}", line);
    }
    (v[0].to_string(), v[1].to_string())
}

fn process_line(orbits: &mut OrbitTree, line: String) -> () {
    let (left, right) = get_objects(line);
    let find_left = orbits.get_mut(&left);
    if find_left.is_some() {
        find_left.unwrap().push(right);
    } else {
        orbits.insert(left, vec![right]);
    }
}

fn sum_recursive(orbits: &OrbitTree, key: &String, level: Numeric) -> Numeric {
    let mut total = level;
    match orbits.get(key) {
        Some(objects) => {
            for obj in objects {
                total += sum_recursive(orbits, obj, level+1);
            }
        },
        None => {
            println!("cannot find {}", key);
        }
    }
    total
}

fn sum(orbits: &OrbitTree) -> Numeric {
    sum_recursive(orbits, &String::from("COM"), 0)
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut orbits = HashMap::new();
    for line in reader.lines() {
        process_line(&mut orbits, line?);
    }
    println!("total orbits: {}", sum(&orbits));
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_get_objects() {
        assert_eq!(get_objects("COM)B".to_string()), ("COM".to_string(),"B".to_string()));
    }

    #[test]
    fn test_sum() {
        let mut orbits = HashMap::new();
        orbits.insert(String::from("COM"), vec![String::from("B")]);
        assert_eq!(sum(&orbits), 1);
    }

    #[test]
    fn test_run() {
        let input = vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L"];
        let mut orbits = HashMap::new();
        for line in input {
            process_line(&mut orbits, line.to_string());
        }
        println!("{:?}", orbits);
        assert_eq!(sum(&orbits), 42);
    }

    #[test]
    fn test_run2() {
        let input = vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","J)K","E)J","K)L"];
        let mut orbits = HashMap::new();
        for line in input {
            process_line(&mut orbits, line.to_string());
        }
        assert_eq!(sum(&orbits), 42);
    }
}