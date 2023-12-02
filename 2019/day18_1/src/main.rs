use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    name: char,
    edges: Vec<(char,u32)>
}

#[derive(Debug, Clone)]
struct Maze {
    data: Vec<Vec<char>>,
    path: Vec<(char,(usize,usize),u32)>,
    position: (usize,usize),
    path_len: u32,
    num_total_keys: usize,
}

impl Maze {
    fn from(input: &str) -> Maze {
        // first, render the maze in 2d so we can move around
        let mut maze: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let line = line.trim();
            if line.len() < 1 {
                continue;
            }
            maze.push(line.chars().collect());
        }

        // find the origin and number of keys
        let mut origin = (0,0);
        let mut num_keys = 0;
        for (r,row) in maze.iter().enumerate() {
            for (c,col) in row.iter().enumerate() {
                if *col == '@' {
                    origin = (r,c);
                }
                if col.is_ascii_lowercase() {
                    num_keys += 1;
                }
            }
        }

        Maze{data: maze, path: Vec::new(), position: origin, path_len: 0,
             num_total_keys: num_keys}
    }

    #[inline]
    fn get_at_pos(&self) -> char {
        self.data[self.position.0][self.position.1]
    }

    #[inline]
    fn set_at_pos(&mut self, c: char) -> () {
        self.data[self.position.0][self.position.1] = c;
    }

    /// Clear a path of @ from the maze
    fn clear_path(&mut self) -> () {
        for row in self.data.iter_mut() {
            for c in row.iter_mut() {
                if *c == '@' {
                    *c = '.';
                }
            }
        }
    }

    /// Remove a door from the maze
    fn remove_door(&mut self, door: char) -> () {
        //println!("remove door {}", door);
        for row in self.data.iter_mut() {
            for c in row.iter_mut() {
                if *c == door {
                    *c = '.';
                    return
                }
            }
        }
    }

    /// Create a path segment from the current path being recorded
    fn create_path_segment(&mut self) -> () {
        let c = self.get_at_pos();
        assert!(c.is_ascii_lowercase());
        self.path.push((c, self.position.clone(), self.path_len));
        self.remove_door(c.to_ascii_uppercase());
        self.set_at_pos('.');
        self.path_len = 0;
        self.clear_path();
    }

    fn keys(&self) -> Vec<char> {
        self.path.iter().map(|x| x.0).collect()
    }

    fn total_path_len(&self) -> u32 {
        let mut ret = self.path_len;
        for segment in self.path.iter() {
            ret += segment.2;
        }
        ret
    }
}

#[inline]
fn move_up(input: &Maze) -> Option<Maze> {
    if input.position.0 > 0 {
        let val = input.data[input.position.0-1][input.position.1];
        if val == '.' || val.is_ascii_lowercase() {
            // legal move
            let mut m = input.clone();
            m.set_at_pos('@');
            m.position.0 -= 1;
            m.path_len += 1;
            return Some(m);
        }
    }
    None
}

#[inline]
fn move_down(input: &Maze) -> Option<Maze> {
    if input.position.0+1 < input.data.len() {
        let val = input.data[input.position.0+1][input.position.1];
        if val == '.' || val.is_ascii_lowercase() {
            // legal move
            let mut m = input.clone();
            m.set_at_pos('@');
            m.position.0 += 1;
            m.path_len += 1;
            return Some(m);
        }
    }
    None
}

#[inline]
fn move_left(input: &Maze) -> Option<Maze> {
    if input.position.1 > 0 {
        let val = input.data[input.position.0][input.position.1-1];
        if val == '.' || val.is_ascii_lowercase() {
            // legal move
            let mut m = input.clone();
            m.set_at_pos('@');
            m.position.1 -= 1;
            m.path_len += 1;
            return Some(m);
        }
    }
    None
}

#[inline]
fn move_right(input: &Maze) -> Option<Maze> {
    if input.position.1+1 < input.data[0].len() {
        let val = input.data[input.position.0][input.position.1+1];
        if val == '.' || val.is_ascii_lowercase() {
            // legal move
            let mut m = input.clone();
            m.set_at_pos('@');
            m.position.1 += 1;
            m.path_len += 1;
            return Some(m);
        }
    }
    None
}

type ShortestPathHash = ((usize,usize), String);
struct MemoizeShortestPath {
    data: HashMap<ShortestPathHash, u32>,
}

#[inline]
fn vec2str(v: &Vec<char>) -> String {    
    let mut chars = v.clone();
    chars.sort_by(|a, b| b.cmp(a));
    chars.iter().collect::<String>()
}

impl MemoizeShortestPath {
    fn new() -> MemoizeShortestPath {
        MemoizeShortestPath{data: HashMap::new()}
    }

    fn get(&self, position: (usize,usize), keys: &Vec<char>) -> Option<u32> {
        let hashkey = (position, vec2str(&keys));
        match self.data.get(&hashkey) {
            Some(x) => Some(*x),
            None => None,
        }
    }

    fn set(&mut self, position: (usize,usize), keys: &Vec<char>, path_len: u32) -> bool {
        let hashkey = (position, vec2str(&keys));
        match self.data.get_mut(&hashkey) {
            Some(x) => {
                if path_len < *x {
                    *x = path_len;
                } else {
                    return false;
                }
            },
            None => {
                self.data.insert(hashkey, path_len);
            },
        };
        true
    }
}

/// Find the shortest path through a maze.
fn shortest_path(input: Maze) -> Maze {
    let mut paths = vec![input];
    let mut memory = MemoizeShortestPath::new();

    while !paths.is_empty() {
        let p = paths.remove(0);

        if p.num_total_keys == p.path.len() {
            // all keys found, so halt
            return p;
        }

        // try walking in each direction
        let mut directions = vec![move_up(&p), move_down(&p), move_left(&p), move_right(&p)];
        for (i,opt) in directions.drain(..).enumerate() {
            match opt {
                Some(mut m) => {
                    //println!("moved {}", i);

                    if m.get_at_pos().is_ascii_lowercase() {
                        m.create_path_segment();
                    }

                    // check the memory
                    if memory.get(m.position, &m.keys()).is_some() {
                        continue; // already has shorter distance
                    }

                    // add to memoization
                    let mut keys = Vec::new();
                    let mut len = m.path_len;
                    for p in m.path.iter() {
                        len += p.2;
                        keys.push(p.0);
                    }
                    memory.set(m.position, &keys, len);
                    paths.push(m);
                },
                None => (),
            }
        }
    }

    panic!("no shortest path found!");
}


fn main() {
    let input = "
#################################################################################
#.............#...#...O.#.#...........#.#...#.........#.......#.....#.......#.Z.#
#####.#######.#H#.#.###.#.#.#####.###.#.#.###.#####.#.#.#####.#.###.#.###.###.#.#
#.....#.#...#.#.#...#.#.#.#.#.#...#.....#.#...#...#.#.#...#w..#...#...#.#.....#.#
#.#####.#.#.#.#.#####.#.#.#B#.#.#########.#.###.#.#.#####.#.#####.#####.#######.#
#.#...#...#.....#...#.#.#.....#.........#.#.#...#.#.....#.#...#..y..#...#.....#.#
#.#.#.#.#########.#.#.#.###########.###.#.#.#.#######.###.###.#####.#.#.#.###.#.#
#.#.#...#.........#.#.......#.....#...#.#.#.#.#.....#.....#.#.#...#...#.#.#...#.#
#.###.###.#########.#######.#.###.#####.#.#.#.#.###.#######.#.#.#.#####.#.#.###.#
#...#.#.....#x....#.#..f..#...#...#...#.#.#.#.....#.....#i..#...#.#...#.#.#.#...#
#.#.#.#.#####.###.#.#A###.#####.###.#.#.#.#.#########.#.#.#.#####.#.#.###.#.#.#.#
#.#.#.#.#...#.#.#.#...#.#.#...#.#.T.#...#.#...#.....#.#...#.#.#..e#.#.....#.#.#.#
###.###.#X#.#.#.#.#.###.#.###.#.###.###.#.###.#.###.#.#####.#.#.###.#######.#.#.#
#...#d..#.#...#.#.#.....#...#.#...#...#.#...#.#.#.....#.......#.#.....#.#...#.#.#
#.###.###.#####R#.#####.###.#.###.###.#.#.#.#.#.###########.###.#.###.#.#.#####.#
#...#.#.#...#.......#...#...#...#.....#.#.#...#.....#.....#.#...#...#.#...#...#.#
###E#.#.###.#######.#####.###.#.#######.#.#####.###N#.###.###.###.###.#.###.#.#.#
#.#...#...#..c..#...#r..F.#...#.....#.#.#...#.#.#.#.#...#.#...#...#...#.....#.#.#
#.#####.#######.###.#.#######.#####.#.#.###.#.#.#.#.###.#.#.###.###.#########.#.#
#z....#.......#...#...#.....#s#.......#.#.#.#.....#...#.#...#v..#.#...#.....#...#
#.#.###.#.#######.#####G###.#.#######.#.#.#.#####.#.###.#######.#.###.#.#######.#
#.#.#...#.......#...#...#.#.#...#...#.#.#.#.....#.#...#.D.....#.....#.#.......#.#
###.#.#########.###C#####.#.#.#.#.#.###.#.#####.#####.###############.#.#####.#.#
#...#...#.....#...#....g....#.#.#.#.....#.....#.....#...........#.....#.....#...#
#.#####.#.###.#.#############.#.#.#######.#########.#.#######.#.#.#########.#####
#.....#...#...#.#.........#...#.#.#.....#.........#.#.#.....#.#...#.......#.#..u#
#Q#.#######.###.#.#.#.#####.###.#.#####.#.#######.#.###.###.#.#####.###.#.#.#.###
#.#.......#.#...#.#.#.#...#.#.#...#...#.#.#.....#.#.#...#.#.#...#.....#.#.#.#...#
#.#####.###.#.###.#.###.#.#.#.#####.#.#.#.#.###.#.#.#.###.#.#.#.#######.#.#.#.#.#
#...#...#...#.....#.....#...#.......#...#.#.#.#.K.#...#.#...#.#.#.......#.#.#.#.#
###.#.###.#####################.###.#####.#.#.#.#######.#.#####.#.#####.###.#.#.#
#...#.....#...#.....#.........#...#.#...#.#.#...#.......#...#...#...#...#...#.#.#
#.#########.###.###.#.###.###.###.#.#.#.###.#####.#.###.###.#.###.#.#.###.#####.#
#.#.....#.........#...#...#.#.#.#.#.#.#.#...#.....#.#l..#.#.#...#.#.#...#...#...#
#.###.#.###.###########.###.#.#.#.#.#.#.#.###.#####.#.###.#.###.#.#.#.#####.#.#.#
#.#...#...#...#...#.....#...#.#...#...#.#...#.#.#...#.....#.....#.#.#.#...#.#.#.#
#.#.#####.#.###.#.#.#####.###.#########.#.#.#.#.#.#################.###.#.#P#.#.#
#.#.#.....#.#...#.#...#.#...#.#.....#...#.#.#.#.#.#.................#...#...#.#.#
#.#.#.#######.###.###.#.#.#.#.#.###.#.#.#.#.#.#.#.#.#########.#######.#######.#.#
#.U.#p........#....j..#...#.....#.....#...#.....#...........#...........J.....#.#
#######################################.@.#######################################
#...#.....#...#.......#.........#.........#.....#.....#.....#.....#.............#
#.#.#.###.###.#.#L###.#.#####.###.#.###.#.#.#.###.#.###.#.#.#.#.#.#.#########.#.#
#.#...#.....#...#...#.#.#...#.....#...#.#...#.....#.....#.#.#.#.#.#.#.......#q#.#
#.#.#######.#.#####.#.###.#.#########.#.#.###############.###.#.#.###.#####.###.#
#.#.#.....#.#...#...#.....#.#.........#.#.#.....#.....#...#...#.#.....#...#...#.#
#.#.#.###.#.#####.#########.#.#########.#.#.#.#.#.###.#.###.###.#.#####.#.###.#.#
#.#.#...#.#.....#.....#...#...#.......#.#.#.#.#...#...#.#.....#.#.#...#.#...#...#
#.#####.#.#####.#####.#.#######.###.###.#.#.#.#####.###.#.#####.#.#.#.#.###.###.#
#.....#.#...#...#...#.#...........#...#.#k#.#...#.....#.#.#.....#.#.#.#...#...#.#
#####.#.###.###.#.#.#.#.#############.#.#.#.###.#.#####.#.#.#######.#.###.#.###.#
#.....#...#...#...#...#.#...........#...#.#...#.#...#...#.#...#...#.#...#.#.....#
#.#######.###.#####.#####.#########.#.###.#####.###.#.###.###.#.#.#.###.#.#######
#.......#...#.....#.#.....#.......#.#...#.......#.#.#.#...#.#...#.....#.#.#.....#
#.###.#####.#####.###.#####.#.#.###.#############.#.#.#.###.###########.#.#.###.#
#...#.#.........#...#.#.....#.#.#...#...#...#.....#.#...#...#.......#...#.#.#m..#
#.###.#.###########.#.#######.###.###.#.#.#.#.#.###.#####.#.#######.#.###.#.#.#.#
#.#...#...#.#.......#...#...#.W...#...#.#.#.#.#.....#.....#...#.....#...#.#.#.#.#
###.#.###.#.#.#########.#.#.#####.###.#.#.###.#######.#######.#.###.###.#.###.#.#
#...#...#...#...#.....#...#.....#...#.#.#.#...#.........#.......#.#.#...#.....#.#
#.#########.###.#.#.###########.###.#.#.#.#.###.#######.#.#######.#.#.#########.#
#.........#.#...#.#.#....a......#.#...#.#.#...#.#.......#.#...#.....#.#...#.....#
#.#######.#.#.###.#.#############.#####.#.###.#.#.#######.#.###.#####.###.#.#####
#.#.....#...#.#...#.....#...#.......#h..#.....#.#.#.......#.#...#...#...#.#.#...#
#.###.#.#####.#.#####.#.#.#.#.#####.#.###.#######.#######.#.#####.#.###.#.#.#.#.#
#...#.#...#...#.#...#.#.#.#..o#...#...#.#.......#.......#.#.....#.#.#...#.....#.#
###.#####.#.#.#.###.#.###.#######.#####.#######.#.#####.#######.#.#.#.###########
#.#...#...#.#.#.....#.....#.#.......#...#...#...#.....#.......#.#.#.#...#.......#
#.###.#.###.#.#####.#######.#.#I###.###.#.#.#.###########.###.#.#.#.###.#.#####.#
#...#...#...#.....#.#.#.....#.#...#t..#.#.#...#.........#.#.#.#...#...#...#.....#
#.#####.#.#########.#.#.###.#.###.###.#.#.#####.#######.#.#.#.#######.#####.#####
#...M...#.......#...#...#.#...#.#...#...#.......#.....#.#.#.........#.....#.#...#
#.#############.#.###.###.#####.###.#############.#####.#.#.#######.#####.#.###.#
#.#.....Y...#.#.#.#.#.#..b..#...#.#.#...#...#.........#.#.#.#...#.#.#...#...#...#
#.#.#######.#.#.#.#.#.###.#.#.#.#.#.#.#.#.#.#.#######.#.#.###.#.#.#.#.#.#####.#.#
#...#n....#.#.#...#.......#...#...#...#.#.#.#.#.....#...#.....#.#...#.#.......#.#
#####.###.#.#.###################.#####.#.###.#.###.#####V#####.#.###.###.#####.#
#.....#...#...#.....#...#...#.....#.....#.....#...#...........#.#...#.#...#.....#
#.###########.###.#.#.#.#.#.#######.###.#.#####################.###.###.###.#####
#.................#...#...#.........#.S.#.......................#.......#.......#
#################################################################################";
    
    let m = Maze::from(input);
    let m = shortest_path(m);
    println!("maze_path: {:?}", m.path);
    println!("length: {}", m.total_path_len());
}

mod tests {
    use super::*;

    #[test]
    fn test_make_maze() {
        let input = "
#########
#b.A.@.a#
#########";
        let mut m = Maze::from(input);
        assert_eq!(m.position, (1,5));
        m.position = (1,1);
        assert_eq!(m.get_at_pos(), 'b');
        m.position = (1,3);
        assert_eq!(m.get_at_pos(), 'A');
        m.position = (1,7);
        assert_eq!(m.get_at_pos(), 'a');
    }

    #[test]
    fn test_find_shortest_path() {
        let input = "
#########
#b.A.@.a#
#########";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 8);
        assert_eq!(m.keys(), vec!['a','b']);
    }

    #[test]
    fn test_find_shortest_path2() {
        let input = "
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 86);
        assert_eq!(m.keys(), vec!['a','b','c','d','e','f']);
    }

    #[test]
    fn test_find_shortest_path3() {
        let input = "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 132);
        assert_eq!(m.keys(), vec!['b','a','c','d','f','e','g']);
    }

    #[test]
    fn test_find_shortest_path4() {
        let input = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 136);
        //assert_eq!(m.keys(), vec!['a','f','b','j','g','n','h','d','l','o','e','p','c','i','k','m']);
    }

    #[test]
    fn test_find_shortest_path5() {
        let input = "
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 81);
        assert!(m.keys() == vec!['a','c','d','g','f','i','b','e','h'] ||
                m.keys() == vec!['a','c','f','i','d','g','b','e','h']);
    }

}