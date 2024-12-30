use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn nineteen() -> io::Result<()> {
    let (threads, patterns) = match read_data(String::from("data/19/test.txt"), true) {
        Ok(stuff) => {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };
    Ok(())
}

/// returns the map, the start, and end locations
#[allow(dead_code, unused_assignments)]
fn read_data(file: String, debug: bool) -> io::Result<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    let mut ret_t: Vec<Vec<char>> = Vec::new();
    let mut ret_p: Vec<Vec<char>> = Vec::new();
    let mut s_or_t = true;

    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            s_or_t = false;
            continue;
        } else {
            if s_or_t {
                let mut line_c = Vec::new();

                for cc in line.chars() {
                    match cc {
                        ' ' => continue,
                        ',' => {
                            ret_t.push(line_c);
                            line_c = Vec::new();
                        },
                        _ => line_c.push(cc),
                    }
                }
                ret_t.push(line_c);
            } else {
                ret_p.push(line.chars().into_iter().collect());
            }
        }
    }
    if debug {
        for ii in 0..ret_t.len() {
            ret_t[ii].iter().for_each(|cc| print!("{}", cc));
            print!(", ");
        }
        println!("\n");
        for ii in 0..ret_p.len() {
            ret_p[ii].iter().for_each(|cc| print!("{}", cc));
            println!("");
        }
    }
    return Ok((ret_t, ret_p));
}