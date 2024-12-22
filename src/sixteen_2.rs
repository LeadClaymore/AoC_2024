use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

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
    let (maze, s_pos, e_pos) = match read_data(String::from("data/16/test.txt")) {
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


    if let Some(answer) = traverse_maze(&maze, s_pos, Dir::Right, &e_pos, &mut HashSet::new()) {
        println!("answer = {}", path_val(&answer));
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
}
//end