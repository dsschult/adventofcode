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

fn parent<'a>(orbits: &'a OrbitTree, obj: &String) -> Option<&'a String> {
    let mut parent = None;
    for (key,val) in orbits.iter() {
        if val.contains(obj) {
            parent = Some(key);
            break;
        }
    }
    parent
}

fn all_parents<'a>(orbits: &'a OrbitTree, obj: &String) -> Vec<&'a String> {
    let mut parents = Vec::new();
    match parent(orbits, obj) {
        Some(p) => { parents.push(p); },
        None => (),
    };
    if !parents.is_empty() {
        loop {
            match parent(orbits, parents.last().unwrap()) {
                Some(p) => { parents.push(p); },
                None => break,
            }
        }
    }
    parents.reverse();
    parents
}

fn transfers(orbits: &OrbitTree, from: &String, to: &String) -> Numeric {
    let from_parents = all_parents(orbits, from);
    let to_parents = all_parents(orbits, to);
    let mut depth = 0;
    println!("from_parents: {:?}", from_parents);
    println!("to_parents: {:?}", to_parents);
    for (a, b) in from_parents.iter().zip(to_parents.iter()) {
        if a != b {
            println!("depth: {}", depth);
            return (from_parents.len() as Numeric) - depth + (to_parents.len() as Numeric) - depth + 2;
        }
        depth += 1;
    }
    // must be the difference between the two vectors
    if from_parents.len() > to_parents.len() {
        (from_parents.len() - to_parents.len() + 2) as Numeric
    } else {
        (to_parents.len() - from_parents.len() + 2) as Numeric
    }
}

fn transfers_orbiting(orbits: &OrbitTree, from: &String, to: &String) -> Numeric {
    let from_obj = parent(orbits, from).unwrap();
    let to_obj = parent(orbits, to).unwrap();
    transfers(orbits, from_obj, to_obj)
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let reader = BufReader::new(f);
    let mut orbits = HashMap::new();
    for line in reader.lines() {
        process_line(&mut orbits, line?);
    }
    println!("total orbits: {}", sum(&orbits));
    let t = transfers_orbiting(&orbits, &String::from("SAN"), &String::from("YOU"));
    println!("transfers between SAN and YOU: {}", t);
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
    fn test_all_parents() {
        let mut orbits = HashMap::new();
        orbits.insert(String::from("COM"), vec![String::from("B")]);
        orbits.insert(String::from("B"), vec![String::from("C")]);
        let p = all_parents(&orbits, &String::from("C"));
        assert_eq!(p, vec![&String::from("COM"), &String::from("B")]);
    }
    
    #[test]
    fn test_transfers() {
        let input = vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L"];
        let mut orbits = HashMap::new();
        for line in input {
            process_line(&mut orbits, line.to_string());
        }
        let t = transfers(&orbits, &String::from("H"), &String::from("I"));
        assert_eq!(t, 5);
        let t2 = transfers_orbiting(&orbits, &String::from("H"), &String::from("I"));
        assert_eq!(t2, 3);
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