use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

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
    let n_map = form_cost_map(&maze, (6, 6), true);
    let answer = form_path(&n_map, (0, 0), true);
    println!("answer: {}", answer.unwrap().len() - 1);
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
fn form_cost_map(map: &Vec<Vec<char>>, e_pos: (usize, usize), debug: bool) -> Vec<Vec<u32>> {
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

    let mut l_count: u128 = 0;
    let mut l_count_goal = 1;
    // while theres someting in to_change
    while !&to_change.is_empty() {
        let mut n_to_change = HashSet::new();
        for &c_pos in &to_change {
            if debug {
                l_count += 1;
                if l_count == l_count_goal {
                    println!("tried {l_count_goal} nodes");
                    l_count_goal *= 10;
                }
            }
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

#[allow(dead_code, unused_assignments)]
fn form_path(n_map: &Vec<Vec<u32>>, s_pos: (usize, usize), debug: bool) -> Option<Vec<(usize, usize)>> {
    if n_map[s_pos.0][s_pos.1] == u32::MAX {
        return None;
    }
    if debug { println!("valid path entered"); }

    let mut ret = Vec::new();
    let bound = (n_map.len(), n_map[0].len());
    ret.push(s_pos);

    while n_map[ret.last().unwrap().0][ret.last().unwrap().1] != 0 {
        let &c_node = ret.last().unwrap();
        // this puts all possible positions to move to in p_next
        let mut p_next = Vec::new();
        if c_node.0 > 0 {
            p_next.push((c_node.0 - 1, c_node.1));
        }
        if c_node.0 + 1 < bound.0 {
            p_next.push((c_node.0 + 1, c_node.1));
        }
        if c_node.1 > 0 {
            p_next.push((c_node.0, c_node.1 - 1));
        }
        if c_node.1 + 1 < bound.1 {
            p_next.push((c_node.0, c_node.1 + 1));
        }

        let mut best = p_next[0];
        for ii in 1..p_next.len() {
            if n_map[p_next[ii].0][p_next[ii].1] < n_map[best.0][best.1] {
                best = p_next[ii];
            }
        }
        ret.push(best);
    }
    if debug {
        println!("Path found");
        print_path(&n_map, &ret);
    }
    return Some(ret);
}

// from 16 but modified
/// prints the maze and replaces 1 position with a char passed in
#[allow(dead_code, unused_assignments)]
fn print_path(maze: &Vec<Vec<u32>>, path: &Vec<(usize, usize)>) {
    for ii in 0..maze.len() {
        for jj in 0..maze[ii].len() {
            if path.contains(&(ii, jj)) {
                print!("@");
            } else {
                match maze[ii][jj] {
                    u32::MAX => print!("#"),
                    _ => print!("."),
                }
            }
        }
        println!("");
    }

}
//end