use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type IdSize = u32;

type Block = Option<IdSize>;

#[derive(Debug, Clone)]
struct File {
    id: Option<IdSize>,  // None is free space
    len: IdSize
}

#[derive(Debug, Clone)]
struct Filesystem {
    map: String,
    files: Vec<File>,
    disk: Vec<Block>
}

impl Filesystem {
    fn new(lines: &Vec<String>) -> Filesystem {
        let mut ret = String::new();
        for line in lines.iter() {
            let trim_line = line.trim();
            if trim_line.is_empty() {
                continue;
            }
            ret.push_str(trim_line);
        }
        Filesystem{ map: ret, files: Vec::new(), disk: Vec::new() }
    }

    fn expand_map(&mut self) {
        let mut id: IdSize = 0;
        let mut file_or_free = true;
        for c in self.map.chars() {
            let len = c.to_digit(10).unwrap() as IdSize;
            let id = match file_or_free {
                true => {
                    let r = id;
                    id += 1;
                    Some(r)
                },
                false => {
                    None
                }
            };
            self.files.push(File{ id: id, len: len });
            self.disk.extend(vec![id; len as usize]);
            file_or_free = match id {
                Some(_) => {
                    false
                },
                None => {
                    true
                }
            };
        }
    }

    fn defrag(&mut self) {
        let mut start_ptr = 0;
        let mut end_ptr = self.disk.len()-1;

        while start_ptr < end_ptr {
            match (self.disk[start_ptr], self.disk[end_ptr]) {
                (None, None) => {
                    end_ptr -= 1;
                },
                (None, Some(_)) => {
                    self.disk.swap(start_ptr, end_ptr);
                },
                (Some(_), None) => {
                    start_ptr += 1;
                    end_ptr -= 1;
                },
                (Some(_), Some(_)) => {
                    start_ptr += 1;
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut ret = 0;
        for i in 0..self.disk.len() {
            ret += match self.disk[i] {
                Some(id) => {
                    id as usize * i
                },
                None => 0
            };
        }
        ret
    }
}

fn main() {
    let lines = read_lines("input");
    let mut fs = Filesystem::new(&lines);
    fs.expand_map();
    fs.defrag();
    println!("checksum: {}", fs.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sample: Vec<String> = "
12345
".lines().map(String::from).collect();

        let mut fs = Filesystem::new(&sample);
        fs.expand_map();
        assert_eq!(fs.files.len(), 5);
        assert_eq!(fs.files[0].id, Some(0));
        assert_eq!(fs.files[1].id, None);
        assert_eq!(fs.files[2].id, Some(1));
        assert_eq!(fs.files[3].id, None);
        assert_eq!(fs.files[4].id, Some(2));
    }

    #[test]
    fn test_10() {
        let sample: Vec<String> = "
2333133121414131402
".lines().map(String::from).collect();

        let mut fs = Filesystem::new(&sample);
        fs.expand_map();
        assert_eq!(fs.files.len(), 19);

        fs.defrag();
        assert_eq!(fs.checksum(), 1928);
    }
}