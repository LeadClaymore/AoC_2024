use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code, unused_assignments)]
impl Dir {
    // ima try a new thing where I just subtract both by 1 so I can have -1's without conversions
    /// returns (ii, jj)
    fn c_cords(&self) -> (usize, usize) {
        return match self {
            Dir::Down => (0, 1),
            Dir::Up => (2, 1),
            Dir::Right => (1, 2),
            Dir::Left => (1, 0),
        };
    }
}

#[allow(dead_code, unused_assignments)]
pub fn fifteen() -> io::Result<()> {
    let (mut maze, moves, s_pos) = match read_data_2(String::from("data/15/test.txt")) {
        Ok(stuff) =>  {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };
    
    let mut pos = s_pos.clone();
    print_maze(&maze);
    for dd in 0..moves.len() {
        //TODO need to take the position its moving to and check if there is 
        //1 imoveable blocks,
        //2 moveable blocks with clear spaces to move to (contiue checking in the same way)
        //3 moveable blocks with blockage in the way (aka theres a wall or series of blocks leading to a wall)
        // I know how I could do this with recursion, however I dont want to send a new reference to each, so ima learn a new way
        // if the block cant be moved then stop the command
        // otherwise apply the movement

        let mut moved = false;
        (moved, maze) = try_to_move(maze, pos.0, pos.1, moves[dd]);
        if moved {
            if let Some(n_pos) = find_pos(&maze) {
                pos = n_pos;
            }
        }
        print_maze(&maze);
    }

    Ok(())
}

/// return [i32; 6] is [start_jj, start_ii, velocity_jj, velocity_ii, current_jj, current_ii]
#[allow(dead_code, unused_assignments)]
fn read_data_2(file: String) -> io::Result<(Vec<Vec<char>>, Vec<Dir>, (usize, usize))> {
    let mut ret_m: Vec<Vec<char>> = Vec::new();
    let mut ret_d = Vec::new();
    let mut s_pos = (0, 0);

    let mut line_inx = 0;
    let mut maze_over = false;
    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            maze_over = true;
            continue;
        } else if maze_over {
            for c in line.chars() {
                if c == '^' {
                    ret_d.push(Dir::Up);
                } else if c == '>' {
                    ret_d.push(Dir::Right);
                } else if c == '<' {
                    ret_d.push(Dir::Left);
                } else if c == 'V' {
                    ret_d.push(Dir::Down);
                }
            }
        } else {
            let mut line_c = Vec::new();
            let mut line_jj = 0;
            for c in line.chars() {
                if c == '@' {
                    s_pos = (line_inx, line_jj);
                }
                line_c.push(c);
                line_jj += 1;
            }

            ret_m.push(line_c);
        }
        line_inx += 1;
    }
    return Ok((ret_m, ret_d, s_pos));
}

#[allow(dead_code, unused_assignments)]
fn in_bound(ii: usize, jj: usize, mi: usize, mj: usize, dif: (usize, usize)) -> Option<(usize, usize)> {
    if 
        ii == 1 && dif.0 == 0 ||
        jj == 1 && dif.1 == 0 ||
        ii == mi - 1 && dif.0 == 2 ||
        jj == mj - 1 && dif.1 == 2
    {
        return None;
    } else {
        return Some((ii + dif.0 - 1, jj + dif.1 - 1));
    }
}

#[allow(dead_code, unused_assignments)]
fn try_to_move(mut maze: Vec<Vec<char>>, ii: usize, jj: usize, dd: Dir) -> (bool, Vec<Vec<char>>) {
    if let Some((ni, nj)) = in_bound(ii, jj, maze.len(), maze[0].len(), dd.c_cords()) {
        if maze[ni][nj] == 'O' {
            let (ret, mut maze) = try_to_move(maze, ni, nj, dd);
            if ret == true {
                maze[ni][nj] = maze[ii][jj];
                maze[ii][jj] = '.';
                return (true, maze);
            }
            return (false, maze);
        } else if maze[ni][nj] == '.' {
            maze[ni][nj] = maze[ii][jj];
            maze[ii][jj] = '.';
            return (true, maze);
        } else {
            return (false, maze);
        }
    } else {
        return (false, maze);
    }
}

#[allow(dead_code, unused_assignments)]
fn find_pos(maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for ii in 0..maze.len() {
        for jj in 0..maze.len() {
            if maze[ii][jj] == '@' {
                return Some((ii, jj));
            }
        }
    }
    return None;
}


#[allow(dead_code, unused_assignments)]
fn print_maze(maze: &Vec<Vec<char>>) {
    println!("---------------------------------------------------------");
    for ii in 0..maze.len() {
        for jj in 0..maze[ii].len() {
            print!("{}", maze[ii][jj]);
        }
        println!("");
    }
}
//end