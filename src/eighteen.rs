use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq)]
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

    /// returns if its vertical or horizontal
    fn vert(&self) -> bool {
        return match self {
            Dir::Down => true,
            Dir::Up => true,
            Dir::Right => false,
            Dir::Left => false,
        };
    }
}

#[allow(dead_code, unused_assignments)]
pub fn eighteen() -> io::Result<()> {
    let (pos_vec) = match read_data(String::from("data/18/test.txt")) {
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