//use std::collections::HashSet;
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
    let mut maze = form_map(pos_vec, (6, 6), (0, 12), true);
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