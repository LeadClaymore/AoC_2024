use std::collections::HashSet;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::u32;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Edge {
    cost: u128,
    d1: Dir,
    p1: (usize, usize),
    d2: Dir,
    p2: (usize, usize),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Node {
    pos: (usize, usize),
    edges: Vec<Edge>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum MP {
    Empty,
    Node,
    Edge,
    Wall,
    DeadEnd,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code, unused_assignments)]
impl Dir {
    /// returns (ii, jj) subtract (1, 1) from result
    fn c_cords(&self) -> (usize, usize) {
        return match self {
            Dir::Down => (2, 1),
            Dir::Up => (0, 1),
            Dir::Right => (1, 2),
            Dir::Left => (1, 0),
        };
    }

    /// p1 is start and p2 is end
    fn from_pos(p1: &(usize, usize), p2: &(usize, usize)) -> Option<Dir> {
        let ii_c = p1.0 as i8 - p2.0 as i8;
        let jj_c = p1.1 as i8 - p2.1 as i8;
        // if the ii is neg and jj is still its down
        if ii_c < 0 && jj_c == 0 {
            return Some(Dir::Down);
        // if the ii is pos adn the jj is still its up
        } else if ii_c > 0 && jj_c == 0 {
            return Some(Dir::Up);
        // if the ii is still and jj is neg its left
        } else if ii_c == 0 && jj_c > 0 {
            return Some(Dir::Left);
        // if the ii is still and jj is pos its right
        } else if ii_c == 0 && jj_c < 0 {
            return Some(Dir::Right);
        }
        return None;
    }

    fn p_dir(&self) -> String {
        return match self {
            Dir::Down => "Down".to_string(),
            Dir::Up => "Up".to_string(),
            Dir::Right => "Right".to_string(),
            Dir::Left => "Left".to_string(),
        };
    }

    fn p_dir_c(&self) -> char {
        return match self {
            Dir::Down => 'v',
            Dir::Up => '^',
            Dir::Right => '>',
            Dir::Left => '<',
        };
    }

    /// returns (ii, jj) subtract (1, 1) from result
    fn c_cords_2(&self, pos: &(usize, usize)) -> (usize, usize) {
        return match self {
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0, pos.1 + 1),
            Dir::Left => (pos.0, pos.1 - 1),
        };
    }

    /// returns if its vertical or horizontal
    fn vert(&self) -> bool {
        return match self {
            Dir::Down => true,
            Dir::Up => true,
            Dir::Right => false,
            Dir::Left => false,
        };
    }

    // not used was for !opose
    /// (d: 0, u: 1, r: 2, l: 3)
    /// for comparason reasons
    fn to_num(&self) -> i32 {
        return match self {
            Dir::Down => 0,
            Dir::Up => 1,
            Dir::Right => 2,
            Dir::Left => 3,
        }
    }

    /// returns false if its the oposite of the dir
    fn not_opose(&self, dd: &Dir) -> bool {
        return !(self.vert() == dd.vert() && self != dd);
    }

    /// gets an list of viable options in the order of to_num
    fn all_but_opose(&self) -> Vec<Dir> {
        let mut ret = Vec::new();
        ret.push(Dir::Down);
        ret.push(Dir::Up);
        ret.push(Dir::Left);
        ret.push(Dir::Right);
        for ii in 0..ret.len() {
            if !self.not_opose(&ret[ii]) {
                ret.remove(ii);
                break;
            }
        }
        return ret;
    }
    
    /// made it more efficent elsewhere so this is now uneeded
    fn all() -> Vec<Dir> {
        let mut ret = Vec::new();
        ret.push(Dir::Down);
        ret.push(Dir::Up);
        ret.push(Dir::Left);
        ret.push(Dir::Right);
        return ret;
    }
}

#[allow(dead_code, unused_assignments)]
pub fn sixteen() -> io::Result<()> {
    let (maze, s_pos, e_pos) = match read_data(String::from("C:/Users/Clayton Ross/Desktop/Rust/AoC_2024/data/16/test.txt")) {
        Ok(stuff) => {
            println!("Data read");
            println!("s: ({}, {}), e: ({}, {})", stuff.1.0, stuff.1.1, stuff.2.0, stuff.2.1);
            for ii in 0..stuff.0.len() {
                for jj in 0..stuff.0[ii].len() {
                    print!("{}", stuff.0[ii][jj]);
                }
                println!("");
            }
            stuff
        },
        Err(ret) => return Err(ret),
    };


    if let Some(answer) = traverse_maze_4(&maze, s_pos, &e_pos) {
        // -1 is from us using the left of the starting position so we start by moving right
        println!("answer = {}", path_cost(&answer) - 1);
    }
    Ok(())
}

/// returns the map, the start, and end locations
#[allow(dead_code, unused_assignments)]
fn read_data(file: String) -> io::Result<(Vec<Vec<char>>, (usize, usize), (usize, usize))> {
    let mut ret_m: Vec<Vec<char>> = Vec::new();
    let mut s_pos = (0, 0);
    let mut e_pos = (0, 0);

    let mut lli = 0;
    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        } else {
            let mut line_c = Vec::new();
            let mut llj = 0;
            for c in line.chars() {
                if c == 'S' {
                    s_pos = (lli, llj);
                } else if c == 'E' {
                    e_pos = (lli, llj);
                }
                line_c.push(c);
                llj += 1;
            }

            ret_m.push(line_c);
        }
        lli += 1;
    }
    return Ok((ret_m, s_pos, e_pos));
}

//traverse maze 5 (or 4 depending on count) is going to first take the maze, and atempt to turn it into a graph of costs between two points
#[allow(dead_code, unused_assignments)]
fn traverse_maze_4(maze: &Vec<Vec<char>>, s_pos: (usize, usize), e_pos: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut enm_graph = Vec::new();
    // for each node in the maze determin what MP it would be based on if its a '#' or what its adj to
    for ii in 0..maze.len() {
        enm_graph.push(Vec::new());
        for jj in 0..maze[ii].len() {
            if maze[ii][jj] == '#' {
                enm_graph[ii].push(MP::Wall);
            } else {
                let mut adj = 0;
                for dd in Dir::all() {
                    let pos = dd.c_cords_2(&(ii, jj));
                    if maze[pos.0][pos.1] != '#' {
                        adj += 1;
                    }
                }
                let mp = match adj {
                    0 => {
                        println!("Error non '#' with 0 adj spaces");
                        MP::Wall
                    },
                    1 => MP::DeadEnd,
                    2 => MP::Edge,
                    3 => MP::Node,
                    4 => MP::Node,
                    _ => {
                        println!("Error non '#' with >4 adj spaces");
                        MP::Wall
                    },
                };
                enm_graph[ii].push(mp);
            }
        }
    }

    

    return None;
}

// Ok so this being try 4 I will not use recursion and instead itteration to find the answer
// what im going to do is have a hashmap of positions
#[allow(dead_code, unused_assignments)]
fn traverse_maze_3(maze: &Vec<Vec<char>>, s_pos: (usize, usize), e_pos: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut been = Vec::new();
    let mut been_set = HashSet::new();
    let mut been_moves = Vec::new();

    been.push(Dir::Left.c_cords_2(&s_pos));
    been.push(s_pos);
    been_set.insert(Dir::Left.c_cords_2(&s_pos));
    been_set.insert(s_pos);
    been_moves.push(4);
    been_moves.push(0);
    
    let mut best_path = Vec::new();
    let mut best_val = u128::MAX;

    loop {
        // once we have check all of the maze
        if been_set.is_empty() {
            if best_path.is_empty() {
                return None;
            } else {
                return Some(best_path);
            }
        }

        // current position
        let c_inx = been_moves.len() - 1;
        let c_pos = been[c_inx];

        // end condition
        if c_pos.0 == e_pos.0 && c_pos.1 == e_pos.1 {
            //println!("end found!, prev best: {}", best_val);
            let c_val = path_cost(&been);
            if c_val < best_val {
                best_val = c_val;
                best_path = been.clone();
                println!("new best path cost: {}", best_val);
            }
        }

        // gets the dirrection based on how many times weve been on this pos
        let c_dir = match been_moves[c_inx] {
            0 => Dir::Down,
            1 => Dir::Up,
            2 => Dir::Left,
            3 => Dir::Right,
            // fully used
            _       =>  {
                been_moves.pop();
                been_set.remove(&c_pos);
                been.pop();
                continue;
            },
        };

        // next position based on dir, 
        // if been does not contain and is not a wall then put it in the stack
        // either way this square in been is used and gets itterated
        let n_pos = c_dir.c_cords_2(&c_pos);
        if 
            !been_set.contains(&n_pos) &&
            maze[n_pos.0][n_pos.1] != '#'
        {
            been.push(n_pos);
            been_set.insert(n_pos);
            been_moves.push(0);
        }
        been_moves[c_inx] += 1;
    }
}

// ok IDK if try 2 would work however it takes too damn long
// so with try 3 I will be ignoring changing dirreciton outside of calculating cost.
// im going to assume there are no turns from the start into the first move, and then if im one off then im going to add or subtract it
// #[allow(dead_code, unused_assignments)]

// fn traverse_maze_2(maze: &Vec<Vec<char>>, pos: (usize, usize), end: &(usize, usize), been: &HashSet<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
//     if been.contains(&(pos.0, pos.1)) {
//         return None;
//     }
//     let mut next_been = been.clone();
//     next_been.insert((pos.0, pos.1));
//     if pos.0 == end.0 && pos.1 == end.1 {
//         let mut ret = Vec::new();
//         ret.push((pos.0, pos.1));
//         println!("found end!");
//         return Some(ret);
//     }
//     let mut c_best = Vec::new();
//     let mut c_val = u32::MAX;
//     for n_pos in get_adj(&pos) {
//         if
//             maze[n_pos.0][n_pos.1] != '#' &&
//             !next_been.contains(&n_pos)
//         {
//             if let Some(mut n_path) = traverse_maze_2(maze, n_pos, end, &next_been) {
//                 n_path.push(pos.clone());
//                 let n_val = path_cost(&n_path);
//                 //update best path if better
//                 if n_val < c_val {
//                     c_best = n_path;
//                     c_val = n_val;
//                 }
//             }
//         }
//     }
//     // if its been updated by something then its checked each possible path from here and found one
//     if !c_best.is_empty() {
//         return Some(c_best);
//     }
//     return None;
// }

///returns all possible positions from another position
#[allow(dead_code, unused_assignments)]
fn get_adj(pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    ret.push((pos.0 + 1, pos.1));
    ret.push((pos.0 - 1, pos.1));
    ret.push((pos.0, pos.1 + 1));
    ret.push((pos.0, pos.1 - 1));
    return ret;
}

#[allow(dead_code, unused_assignments)]
fn path_cost(path: &Vec<(usize, usize)>) -> u128 {
    if path.len() < 1 {
        return u128::MAX;
    }
    let mut ret = 0;
    let mut l_pos = path[0];
    let mut l_dir = Dir::from_pos(&l_pos, &path[1]).unwrap();
    for ii in 1..path.len() {
        let n_dir = Dir::from_pos(&l_pos, &path[ii]).unwrap();
        ret += 1;
        if l_dir != n_dir {
            ret += 1000;
            l_dir = n_dir;
        }
        l_pos = path[ii];
    }
    return ret;
}

//ok so try 2 at traversing a maze, 
//first we check if this pos and dir is allready in the maze, if so 
/// pos end, and all instances of x, y is in ii, jj
#[allow(dead_code, unused_assignments)]
fn traverse_maze(maze: &Vec<Vec<char>>, pos: (usize, usize), dir: Dir, end: &(usize, usize), been: &HashSet<(usize, usize, Dir)>) -> Option<Vec<(usize, usize, Dir)>> {
    if been.contains(&(pos.0, pos.1, dir)) {
        return None;
    }
    let mut c_been = been.clone();
    c_been.insert((pos.0, pos.1, dir).clone());
    
    // print!("Been: ");
    // for (ii, jj, dd) in &c_been {
    //     print!("({}, {}, {}) ", ii, jj, dd.p_dir_c());
    // }
    // println!("");
    // print_maze(maze, pos, dir.p_dir_c());

    if pos.0 == end.0 && pos.1 == end.1 {
        return Some(Vec::new());
    }

    let mut cur_best = Vec::new();
    let mut cur_val = 0;
    for n_dir in dir.all_but_opose() {
        // if we dont change the dir then we need to change the pos
        // also check for there being a wall, or been in the same spot before
        if 
            n_dir == dir && 
            maze[dir.c_cords_2(&pos).0][dir.c_cords_2(&pos).1] != '#' &&
            !c_been.contains(&(dir.c_cords_2(&pos).0, dir.c_cords_2(&pos).1, n_dir)) 
        {
            if let Some(mut possible_route) = traverse_maze(maze, n_dir.c_cords_2(&pos), n_dir, end, &c_been) {
                possible_route.push((pos.0, pos.1, dir));
                let prv = path_val(&possible_route);
                if 
                    // if the current val exists or
                    cur_val == 0 ||
                    prv < cur_val
                {
                    cur_best = possible_route;
                    cur_val = prv;
                }
            }
        } else if 
            !c_been.contains(&(pos.0, pos.1, n_dir))
        {
            if let Some(mut possible_route) = traverse_maze(maze, pos, n_dir, end, &c_been) {
                possible_route.push((pos.0, pos.1, dir));
                let prv = path_val(&possible_route);
                if 
                    cur_val == 0 ||
                    prv < cur_val
                {
                    cur_best = possible_route;
                    cur_val = prv;
                }
            }
        }
    }

    if cur_best.is_empty() {
        return None;
    } else {
        return Some(cur_best);
    }
}

/// prints the maze and replaces 1 position with a char passed in
#[allow(dead_code, unused_assignments)]
fn print_maze(maze: &Vec<Vec<char>>, pp: (usize, usize), cc: char) {
    for ii in 0..maze.len() {
        for jj in 0..maze[ii].len() {
            if (ii, jj) == pp {
                print!("{cc}");
            } else {
                print!("{}", maze[ii][jj]);
            }
        }
        println!("");
    }

}

fn path_val(moves: &Vec<(usize, usize, Dir)>) -> u32 {
    let mut ret = 0;
    let mut c_stat = (0, 0, Dir::Up);
    for ii in 0..moves.len() {
        if ii == 0 {
            c_stat = moves[0];
            ret += 1;
            continue;
        } else if
            //dir is 1 off (rotationaly) (2 or 0 off should not happen aka send error)
            c_stat.0 == moves[ii].0 &&
            c_stat.1 == moves[ii].1 &&
            c_stat.2.vert() != moves[ii].2.vert()
        {
            ret += 1000;
            c_stat = moves[ii];
        } else if
            // the diffrence between the 2 stats is 1 off in stat.0
            c_stat.0.abs_diff(moves[ii].0) == 1 &&
            c_stat.1 == moves[ii].1 &&
            c_stat.2 == moves[ii].2 
        {
            ret += 1;
            c_stat = moves[ii];
        } else if
            // the diffrence between the 2 stats is 1 off in stat.1
            c_stat.0 == moves[ii].0 &&
            c_stat.1.abs_diff(moves[ii].1) == 1 &&
            c_stat.2 == moves[ii].2 
        {
            ret += 1;
            c_stat = moves[ii];
        } else {
            println!("error with c_stat finding! ({}, {}, {}) to ({}, {}, {})", 
                c_stat.0, c_stat.1, c_stat.2.p_dir_c(), 
                moves[ii].0, moves[ii].1, moves[ii].2.p_dir_c()
            );
            return 0;
        }

    }
    return ret;
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opose() {
        let dd = Dir::Down;
        assert_eq!(true, dd.not_opose(&Dir::Down));
        assert_eq!(false, dd.not_opose(&Dir::Up));
        assert_eq!(true, dd.not_opose(&Dir::Right));
        assert_eq!(true, dd.not_opose(&Dir::Left));
    }

    #[test]
    fn test_comp_path() {
        let mut p1 = Vec::new();
        p1.push((10, 0, Dir::Up));
        p1.push((9, 0, Dir::Up));
        p1.push((8, 0, Dir::Up));
        p1.push((8, 0, Dir::Right));
        p1.push((8, 1, Dir::Right));
        let mut p2 = Vec::new();
        p2.push((10, 0, Dir::Up));
        p2.push((9, 0, Dir::Up));
        p2.push((8, 0, Dir::Up));
        p2.push((8, 0, Dir::Right));
        p2.push((8, 1, Dir::Right));
        p2.push((8, 1, Dir::Down));
        let p1_res = path_val(&p1);
        let p2_res = path_val(&p2);
        println!("p1: {p1_res}, p2: {p2_res}");
        assert_eq!(true, p1_res < p2_res);
        assert_eq!(false, p1_res > p2_res);
        //assert_ne!(false, comp_path(&p1, &p2));
    }

    #[test]
    fn test_pos_to_dir() {
        let s_pos = (1, 1);
        let u_pos = (0, 1);
        let d_pos = (2, 1);
        let l_pos = (1, 0);
        let r_pos = (1, 2);
        assert_eq!(Dir::Up, Dir::from_pos(&s_pos, &u_pos).unwrap());
        assert_eq!(Dir::Down, Dir::from_pos(&s_pos, &d_pos).unwrap());
        assert_eq!(Dir::Left, Dir::from_pos(&s_pos, &l_pos).unwrap());
        assert_eq!(Dir::Right, Dir::from_pos(&s_pos, &r_pos).unwrap());
    }

    #[test]
    fn test_path_cost() {
        let mut p1 = Vec::new();
        p1.push((10, 0));
        p1.push((9, 0));
        p1.push((8, 0));
        p1.push((8, 1));
        let mut p2 = Vec::new();
        p2.push((10, 0));
        p2.push((9, 0));
        p2.push((8, 0));
        p2.push((8, 1));
        p2.push((7, 1));
        let mut p3 = Vec::new();
        p3.push((10, 0));
        p3.push((9, 0));
        p3.push((8, 0));
        p3.push((8, 1));
        p3.push((8, 2));
        let p1_res = path_cost(&p1);
        let p2_res = path_cost(&p2);
        let p3_res = path_cost(&p3);
        println!("p1: {p1_res}, p2: {p2_res}, p3: {p3_res}");
        assert_eq!(true, p1_res < p2_res);
        assert_eq!(true, p2_res > p1_res);
        assert_eq!(true, p1_res < p3_res);
        assert_eq!(true, p3_res < p2_res);
    }
}
//end