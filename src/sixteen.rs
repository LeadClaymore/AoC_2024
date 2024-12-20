use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    fn c_cords_2(&self, pos: (usize, usize)) -> (usize, usize) {
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
        return !(self.vert() == dd.vert() && self == dd);
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
            // for ii in 0..stuff.0.len() {
            //     for jj in 0..stuff.0[ii].len() {
            //         print!("{}", stuff.0[ii][jj]);
            //     }
            //     println!("");
            // }
            stuff
        },
        Err(ret) => return Err(ret),
    };

    let mut answer = 0;
    (answer) = traverse_maze(&maze, s_pos, &e_pos, Dir::Right, &mut HashSet::new());
    println!("answer = {}", answer);
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

// traverse maze fn that takes the maze, a POS, e_pos, the last dir, and a count. It returns the maze (for overhead) and a count. 
// Within take the POS on the map and check 
//      if this is the end. If so return the u32::max.
//      If this is a wall then return 0. 
// Send a new recursion in each dir excluding l_dir to reduce overhead. 
// If the next location is not the same dir then +2 to count otherwise +1 cause +3 dne (l_dir). 
// Using the returns 
//      if 0 ignore. 
//      If max then return 0 +# dep on dir, and 
//      if a number then num +#. If all 0 then ret 0. 
// Should make fn to reverse dir. After take count and print
#[allow(dead_code, unused_assignments)]
fn traverse_maze(maze: &Vec<Vec<char>>, pp: (usize, usize), eep: &(usize, usize), dd: Dir, been: &mut HashSet<(usize, usize)>) -> u32 {
    // dont even try if its been here
    if been.contains(&pp) {
        return 0;
    }

    // track where its been
    been.insert(pp);
    print_maze(maze, pp, dd.p_dir_c());
    //print!("({}, {}, {}) ", pp.0, pp.1, dd.p_dir());

    if maze[pp.0][pp.1] == '#' {
        return 0;
    }

    for n_dir in Dir::all() {
        let nnp = n_dir.c_cords_2(pp);
        if nnp == *eep {
            if dd == n_dir {
                // this needs to only move to reach
                return 1;
            } else {
                // this needs to turn then move to reach
                return 1001;
            }
        }
        let r_cc = traverse_maze(maze, nnp, eep, n_dir, been);
        if r_cc != 0 {
            if dd == n_dir {
                // this needs to only move to reach
                return r_cc + 1;
            } else {
                // this needs to turn then move to reach
                return r_cc + 1001;
            }
        }
    }
    return 0;
}

///prints the maze and replaces 1 position with a char passed in
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
}
//end