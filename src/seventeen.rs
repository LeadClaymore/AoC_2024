use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn seventeen() -> io::Result<()> {
    let (prog, regA, regB, regC) = match read_data(String::from("data/17/test.txt")) {
        Ok(stuff) => {
            println!("Data read");
            println!("reg A: {}", stuff.1);
            println!("reg B: {}", stuff.2);
            println!("reg C: {}", stuff.3);
            print!("Prog: ");
            for nn in 0..stuff.0.len() {
                print!(" {},", stuff.0[nn]);
            }
            println!("");
            stuff
        },
        Err(ret) => return Err(ret),
    };
    Ok(())
}

/// returns the map, the start, and end locations
#[allow(dead_code, unused_assignments)]
fn read_data(file: String) -> io::Result<(Vec<u32>, u32, u32, u32)> {
    let mut ret_p = Vec::new();
    let (mut ra, mut rb, mut rc) = (0, 0, 0);

    let mut t_num = 0;
    let mut ii = 0;
    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        } else if ii < 3 {
            for cc in line.chars() {
                if let Some(nn) = cc.to_digit(10) {
                    t_num = t_num * 10 + nn;
                }
            }
            if ii == 0 {
                ra = t_num;
                t_num = 0;
            } else if ii == 1 {
                rb = t_num;
                t_num = 0;
            } else if ii == 2 {
                rc = t_num;
                t_num = 0;
            }
            ii += 1;
        } else {
            for cc in line.chars() {
                if let Some(nn) = cc.to_digit(10) {
                    ret_p.push(nn);
                }
            }
        }
    }
    return Ok((ret_p, ra, rb, rc));
}