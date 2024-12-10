use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

type IdSize = u32;

#[derive(Debug, Clone)]
struct File {
    id: Option<IdSize>,  // None is free space
    len: IdSize
}

#[derive(Debug, Clone)]
struct Filesystem {
    map: String,
    files: Vec<File>
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
        Filesystem{ map: ret, files: Vec::new() }
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

        while start_ptr < self.files.len()-1 {
            if self.files[start_ptr].id == None {
                let mut end_ptr = self.files.len()-1;
                while start_ptr < end_ptr {
                    if self.files[end_ptr].id != None && self.files[end_ptr].len <= self.files[start_ptr].len {
                        let new_len = self.files[start_ptr].len - self.files[end_ptr].len;
                        self.files[start_ptr].id = self.files[end_ptr].id;
                        self.files[end_ptr].id = None;
                        if new_len > 0 {
                            self.files[start_ptr].len = self.files[end_ptr].len;
                            self.files.insert(start_ptr+1, File{ id: None, len: new_len });
                        }
                        break;
                    }
                    end_ptr -= 1;
                }
            }
            start_ptr += 1;
        }
    }

    fn checksum(&self) -> usize {
        let mut disk = Vec::new();
        for f in self.files.iter() {
            if f.len > 0 {
                disk.extend(vec![f.id; f.len as usize]);
            }
        }

        let mut ret = 0;
        for i in 0..disk.len() {
            ret += match disk[i] {
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

        println!("{:?}", fs.files);
        fs.defrag();
        println!("{:?}", fs.files);
        assert_eq!(fs.checksum(), 2858);
    }
}