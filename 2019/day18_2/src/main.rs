use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    name: char,
    edges: Vec<(char,u32)>
}

#[derive(Debug, Clone)]
struct Maze {
    data: Vec<Vec<char>>,
    // char, quadrant, position, path_len
    path: Vec<(char, usize, (usize,usize), u32)>,
    position: [(usize,usize); 4],
    path_len: [u32; 4],
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

        // find the origins and number of keys
        let mut origins = Vec::new();
        let mut num_keys = 0;
        for (r,row) in maze.iter().enumerate() {
            for (c,col) in row.iter().enumerate() {
                if *col == '@' {
                    origins.push((r,c));
                }
                if col.is_ascii_lowercase() {
                    num_keys += 1;
                }
            }
        }
        assert_eq!(origins.len(), 4);

        Maze{data: maze, path: Vec::new(),
             position: [origins[0], origins[1], origins[2], origins[3]],
             path_len: [0;4], num_total_keys: num_keys}
    }

    #[inline]
    fn get_at_pos(&self, quadrant: usize) -> char {
        self.data[self.position[quadrant].0][self.position[quadrant].1]
    }

    #[inline]
    fn set_at_pos(&mut self, quadrant: usize, c: char) -> () {
        self.data[self.position[quadrant].0][self.position[quadrant].1] = c;
    }

    #[inline]
    fn get_quadrant_sizes(&self, quadrant: usize) -> (usize,usize,usize,usize) {
        match quadrant {
            0 => (0, self.data.len()/2, 0, self.data[0].len()/2),
            1 => (0, self.data.len()/2, self.data[0].len()/2, self.data[0].len()),
            2 => (self.data.len()/2, self.data.len(), 0, self.data[0].len()/2),
            3 => (self.data.len()/2, self.data.len(), self.data[0].len()/2, self.data[0].len()),
            _ => panic!("not a valid quadrant"),
        }
    }

    /// Clear a path of @ from the maze, in the current quadrant
    fn clear_path(&mut self, quadrant: usize) -> () {
        let (minrow, maxrow, mincol, maxcol) = self.get_quadrant_sizes(quadrant);
        for i in minrow..maxrow {
            for j in mincol..maxcol {
                if self.data[i][j] == '@' {
                    self.data[i][j] = '.';
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
    fn create_path_segment(&mut self, quadrant: usize) -> () {
        let c = self.get_at_pos(quadrant);
        assert!(c.is_ascii_lowercase());
        self.path.push((c, quadrant, self.position[quadrant].clone(), self.path_len[quadrant]));
        self.remove_door(c.to_ascii_uppercase());
        self.set_at_pos(quadrant, '.');
        self.path_len[quadrant] = 0;
        self.clear_path(quadrant);
    }

    fn keys(&self) -> Vec<char> {
        self.path.iter().map(|x| x.0).collect()
    }

    fn total_path_len(&self) -> u32 {
        let mut ret = self.path_len.iter().sum();
        for segment in self.path.iter() {
            ret += segment.3;
        }
        ret
    }
}

#[inline]
fn move_up(input: &Maze, quadrant: usize) -> Option<Maze> {
    let val = input.data[input.position[quadrant].0-1][input.position[quadrant].1];
    if val == '.' || val.is_ascii_lowercase() {
        // legal move
        let mut m = input.clone();
        m.set_at_pos(quadrant, '@');
        m.position[quadrant].0 -= 1;
        m.path_len[quadrant] += 1;
        return Some(m);
    }
    None
}

#[inline]
fn move_down(input: &Maze, quadrant: usize) -> Option<Maze> {
    let val = input.data[input.position[quadrant].0+1][input.position[quadrant].1];
    if val == '.' || val.is_ascii_lowercase() {
        // legal move
        let mut m = input.clone();
        m.set_at_pos(quadrant, '@');
        m.position[quadrant].0 += 1;
        m.path_len[quadrant] += 1;
        return Some(m);
    }
    None
}

#[inline]
fn move_left(input: &Maze, quadrant: usize) -> Option<Maze> {
    let val = input.data[input.position[quadrant].0][input.position[quadrant].1-1];
    if val == '.' || val.is_ascii_lowercase() {
        // legal move
        let mut m = input.clone();
        m.set_at_pos(quadrant, '@');
        m.position[quadrant].1 -= 1;
        m.path_len[quadrant] += 1;
        return Some(m);
    }
    None
}

#[inline]
fn move_right(input: &Maze, quadrant: usize) -> Option<Maze> {
    let val = input.data[input.position[quadrant].0][input.position[quadrant].1+1];
    if val == '.' || val.is_ascii_lowercase() {
        // legal move
        let mut m = input.clone();
        m.set_at_pos(quadrant, '@');
        m.position[quadrant].1 += 1;
        m.path_len[quadrant] += 1;
        return Some(m);
    }
    None
}

type ShortestPathHash = ([(usize,usize); 4], String);
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

    fn get(&self, position: [(usize,usize); 4], keys: &Vec<char>) -> Option<u32> {
        let hashkey = (position, vec2str(&keys));
        match self.data.get(&hashkey) {
            Some(x) => Some(*x),
            None => None,
        }
    }

    fn set(&mut self, position: [(usize,usize); 4], keys: &Vec<char>, path_len: u32) -> bool {
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
    let mut paths = vec![(input.clone(),0),(input.clone(),1),(input.clone(),2),(input,3)];
    let mut memory = MemoizeShortestPath::new();

    while !paths.is_empty() {
        let (p,quad) = paths.remove(0);

        if p.num_total_keys == p.path.len() {
            // all keys found, so halt
            return p;
        }

        // try walking in each direction
        let mut directions = vec![move_up(&p, quad), move_down(&p, quad),
                                  move_left(&p, quad), move_right(&p, quad)];
        for (i,opt) in directions.drain(..).enumerate() {
            match opt {
                Some(mut m) => {
                    //println!("moved {} in quad {}", i, quad);

                    let mut key_found = false;
                    if m.get_at_pos(quad).is_ascii_lowercase() {
                        m.create_path_segment(quad);
                        key_found = true;
                    }

                    // check the memory
                    if memory.get(m.position, &m.keys()).is_some() {
                        continue; // already has shorter distance
                    }

                    // add to memoization
                    let mut keys = Vec::new();
                    let mut len = m.path_len.iter().sum();
                    for p in m.path.iter() {
                        len += p.3;
                        keys.push(p.0);
                    }
                    memory.set(m.position, &keys, len);

                    if key_found {
                        // push all quads
                        paths.push((m.clone(), 0));
                        paths.push((m.clone(), 1));
                        paths.push((m.clone(), 2));
                        paths.push((m, 3));
                    } else {
                        // continue in current quad
                        paths.push((m, quad));
                    }
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
#.U.#p........#....j..#...#.....#.....#@#@#.....#...........#...........J.....#.#
#################################################################################
#...#.....#...#.......#.........#......@#@#.....#.....#.....#.....#.............#
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
    fn test_find_shortest_path() {
        let input = "
#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#.b#
#######";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 8);
    }

    #[test]
    fn test_find_shortest_path2() {
        let input = "
###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 24);
    }

    #[test]
    fn test_find_shortest_path3() {
        let input = "
#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 32);
    }

    #[test]
    fn test_find_shortest_path4() {
        let input = "
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
        let m = Maze::from(input);
        let m = shortest_path(m);
        println!("maze_path: {:?}", m.path);
        assert_eq!(m.total_path_len(), 72);
    }


}