use std::collections::HashSet;
//use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

// #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
// struct Node {
//     ii: usize,
//     jj: usize,
//     hh: u32,
// }

// impl Node {
//     #[allow(dead_code, unused_assignments)]
//     fn new(ii: usize, jj: usize, hh: u32) -> Node {
//         return Node {
//             ii: ii,
//             jj: jj,
//             hh: hh,
//         }
//     }
// }

#[allow(dead_code, unused_assignments)]
pub fn eighteen() -> io::Result<()> {
    let pos_vec = match read_data(String::from("data/18/test.txt")) {
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
    //print each line
    //pos_vec.iter().for_each(|(ii, jj)| println!("({ii}, {jj})"));
    let maze = form_map(pos_vec, (6, 6), (0, 12), true);
    let n_map = form_cost_map(&maze, (0, 0), (6, 6), true);

    Ok(())
}

/// returns the map, the start, and end locations
#[allow(dead_code, unused_assignments)]
fn read_data(file: String) -> io::Result<Vec<(usize, usize)>> {
    let mut ret_pos = Vec::new();

    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;

        let (mut ii, mut jj) = (usize::MAX, usize::MAX);
        if line.trim().is_empty() {
            continue;
        } else {
            // split and parse
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                (ii, jj) = (parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap());
                ret_pos.push((ii, jj));
            } else {
                eprintln!("Malformed line: {}", line);
            }
        }
    }
    return Ok(ret_pos);
}

#[allow(dead_code, unused_assignments)]
fn form_map(corr: Vec<(usize, usize)>, size: (usize, usize), c_range: (usize, usize), debug: bool) -> Vec<Vec<char>> {
    if debug {println!("Forming Maze");}

    let mut ret = Vec::new();
    for _ in 0..=size.0 {
        ret.push(vec!['.'; size.1 + 1]);
    }

    if debug {
        println!("Filling Maze, bounds: ({}, {})", ret.len(), ret[0].len());
        ret.iter().for_each(|line| {
            line.iter().for_each(|cc| print!("{cc}"));
            println!("");
        });
    }

    // they do jj, ii just remember
    for ii in c_range.0..c_range.1 {
        ret[corr[ii].1][corr[ii].0] = '#';
    }

    if debug {
        println!("Maze Created");
        ret.iter().for_each(|line| {
            line.iter().for_each(|cc| print!("{cc}"));
            println!("");
        });
    }
    return ret;
}

///if a blank (vec::new()) map gets passed this will break
#[allow(dead_code, unused_assignments)]
fn form_cost_map(map: &Vec<Vec<char>>, s_pos: (usize, usize), e_pos: (usize, usize), debug: bool) -> Vec<Vec<u32>> {
    let bound = (map.len(), map[0].len());
    if debug { println!("initilizing nodemap"); }
    let mut ret = Vec::new();
    for ii in 0..bound.0 {
        ret.push(vec![u32::MAX; bound.1]);
    }
    if debug { println!("initilizing done, starting building it"); }
    let mut to_change = HashSet::new();

    //end pos initilization
    ret[e_pos.0][e_pos.1] = 0;
    to_change.insert(e_pos.clone());

    // while theres someting in to_change
    while !&to_change.is_empty() {
        let mut n_to_change = HashSet::new();
        for &c_pos in &to_change {
            // this puts all possible positions to move to in p_next
            let mut p_next = Vec::new();
            if c_pos.0 > 0 {
                p_next.push((c_pos.0 - 1, c_pos.1));
            }
            if c_pos.0 + 1 < bound.0 {
                p_next.push((c_pos.0 + 1, c_pos.1));
            }
            if c_pos.1 > 0 {
                p_next.push((c_pos.0, c_pos.1 - 1));
            }
            if c_pos.1 + 1 < bound.1 {
                p_next.push((c_pos.0, c_pos.1 + 1));
            }

            for n_pos in p_next {
                if 
                    ret[n_pos.0][n_pos.1] > ret[c_pos.0][c_pos.1] + 1 &&
                    map[n_pos.0][n_pos.1] != '#'
                {
                    ret[n_pos.0][n_pos.1] = ret[c_pos.0][c_pos.1] + 1;
                    n_to_change.insert(n_pos);
                }
            }
        }
        to_change = n_to_change;
    }
    
    if debug {
        println!("nodemap done");
        ret.iter().for_each(|line| {
            line.iter().for_each(|&num| {
                if num != u32::MAX {
                    print!("{num:02} ");
                } else {
                    print!("## ");
                }

            });
            println!("");
        });
    }

    return ret;
}
//end