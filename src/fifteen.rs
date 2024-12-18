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
    //p1
    //need to take the position its moving to and check if there is 
    //1 imoveable blocks,
    //2 moveable blocks with clear spaces to move to (contiue checking in the same way)
    //3 moveable blocks with blockage in the way (aka theres a wall or series of blocks leading to a wall)
    // I know how I could do this with recursion, however I dont want to send a new reference to each, so ima learn a new way
    // if the block cant be moved then stop the command
    // otherwise apply the movement

    for dd in 0..moves.len() {
        //println!("Dir #{} is: {}", dd + 1, moves[dd].p_dir());
        let mut moved = false;
        (moved, maze) = try_to_move(maze, pos.0, pos.1, moves[dd]);
        if moved {
            if let Some(n_pos) = find_pos(&maze) {
                pos = n_pos;
            }
        }
        //print_maze(&maze);
    }
    print_maze(&maze);
    let mut total = 0;
    for ii in 0..maze.len() {
        for jj in 0..maze.len() {
            if 
                maze[ii][jj] == 'O' ||
                maze[ii][jj] == '[' 
            {
                total += 100 * ii + jj;
            }
        }
    }
    println!("{total} = total");
    //println!("pos = ({}, {}), GPS = {}", pos.0, pos.1, 100 * pos.0 + pos.1);
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
                } else if c == 'v' {
                    ret_d.push(Dir::Down);
                }
            }
        } else {
            let mut line_c = Vec::new();
            let mut line_jj = 0;
            for c in line.chars() {
                if c == '@' {
                    s_pos = (line_inx, line_jj * 2);
                    //s_pos = (line_inx, line_jj);
                    line_c.push('@');
                    line_c.push('.');
                } else if c == '#' {
                    line_c.push('#');
                    line_c.push('#');
                } else if c == 'O' {
                    line_c.push('[');
                    line_c.push(']');
                } else if c == '.' {
                    line_c.push('.');
                   line_c.push('.');
                } else {
                    println!("Error on {line_inx}, {line_jj}");
                }
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

// this works for p2 when horizontal, so ima use it
#[allow(dead_code, unused_assignments)]
fn try_to_move(mut maze: Vec<Vec<char>>, ii: usize, jj: usize, dd: Dir) -> (bool, Vec<Vec<char>>) {
    if let Some((ni, nj)) = in_bound(ii, jj, maze.len(), maze[0].len(), dd.c_cords()) {
        if 
            maze[ni][nj] == 'O' || 
            maze[ni][nj] == '[' || 
            maze[ni][nj] == ']'
        {
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

//TODO plan
// I dont need the in bound check so ima just scrap it
// The only diffrence is with the moving block thing
// horizontaly I can reuse try_to_move() so ima foucus on vertical moves
// this will work completely different from the first iteration
// it will create a list of moves, by branching out through each possible effect of each move
// and if the entire move is valid, elsewhere it will apply the move 

// TODO the move needs to happen opposite of the dirrection of the push so it does not delete stuff
// aka if we push up then the effects have to happen top down

/// Important: this takes pos of next move not cur move
/// Will return maze and either 
/// None: push is bad. 
/// Some(Vec.empty()): edge '.' space but push is still good.
/// Some(Vec.not_empty()) contents to be move also move is good.
#[allow(dead_code, unused_assignments)]
fn try_to_move_p2(maze: Vec<Vec<char>>, ii: usize, jj: usize, dd: Dir) -> (Option<Vec<(usize, usize)>>, Vec<Vec<char>>) {
    let (nni, nnj);

    // unless block we dont need to calculate anything
    if maze[ii][jj] == '.' {
        return (Some(Vec::new()), maze);
    } else if maze[ii][jj] == '#' {
        return (None, maze);
    } else if maze[ii][jj] == '[' {
        //find other half
        (nni, nnj) = (ii, jj + 1);
    } else if maze[ii][jj] == ']' {
        //find other half
        (nni, nnj) = (ii, jj - 1);
    } else {
        return (None, maze);
    }
    // if it makes it here then its a block, so we try to recurse with both halfs
    let dif = dd.c_cords();
    let (tomv_1, maze) = try_to_move_p2(maze, ii + dif.0, jj + dif.1, dd);
    let (tomv_2, maze) = try_to_move_p2(maze, nni + dif.0, nnj + dif.1, dd);

    // wild if let to get both returns
    if let (Some(mut ret_1), Some(mut ret_2)) = (tomv_1, tomv_2) {
        // combine and return all the changes that must be made
        ret_1.append(&mut ret_2);
        return (Some(ret_1), maze);
    } else {
        // if one of the changes wont work, none of them will
        return (None, maze);
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