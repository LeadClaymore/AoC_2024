use std::fs::File;
use std::io::{self, BufRead};

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
    fn get_cord_changes(&self) -> (usize, usize) {
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
    for dd in 0..moves.len() {
        if let Some((ni, nj)) = in_bound(pos.0, pos.1, maze.len(), maze[0].len(), moves[dd].get_cord_changes()) {
            //TODO need to 
        }
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
    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;
        let mut maze_over = false;
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
//end