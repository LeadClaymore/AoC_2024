use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn fourteen() -> io::Result<()> {
    let mut data = match read_data_2(String::from("data/14/data.txt")) {
        Ok(stuff) =>  {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };
    //grid size in (ii, jj)
    //let gs = (7, 11);
    let gs = (103, 101);
    let mut safty_area = Vec::new();
    for _ii in 0..gs.0 {
        let mut sa_line = Vec::new();
        for _jj in 0..gs.1 {
            sa_line.push(0);
        }
        safty_area.push(sa_line);
    }
    for line in &data {
        safty_area[line[1] as usize][line[0] as usize] += 1;
    }
    //print_sa(&safty_area);

    //holds quad at each sec
    let mut ot_quads = Vec::new();
    
    let mut t_quad = [0; 4];
    for sec in 1..=100 {
        safty_area.clear();
        for _ii in 0..gs.0 {
            let mut sa_line = Vec::new();
            for _jj in 0..gs.1 {
                sa_line.push(0);
            }
            safty_area.push(sa_line);
        }

        let mut c_quad = [0; 4];
        for line in &mut data {
            let n_pos = apply_movement(line, gs.0, gs.1);
            safty_area[n_pos.1 as usize][n_pos.0 as usize] += 1;
            // apply position to total quadrent
            //if on the midpoints
            if n_pos.0 == gs.1 / 2 || n_pos.1 == gs.0 / 2 {
                // it is part of none
            // if pos < midpoint jj
            } else if n_pos.0 < gs.1 / 2 {
                // if pos < midpoint ii
                if n_pos.1 < gs.0 / 2 {
                    t_quad[0] += 1;
                    c_quad[0] += 1;
                // if pos > midpoint ii
                } else {
                    t_quad[2] += 1;
                    c_quad[2] += 1;
                }
            // if pos > midpoint jj
            } else {
                if n_pos.1 < gs.0 / 2 {
                    t_quad[1] += 1;
                    c_quad[1] += 1;
                } else {
                    t_quad[3] += 1;
                    c_quad[3] += 1;
                }
            }
        }
        //print_sa(&safty_area);

        let t_saftey = c_quad[0] * c_quad[1] * c_quad[2] * c_quad[3];
        println!("sec {sec} quad {} {} {} {} = {}", c_quad[0], c_quad[1], c_quad[2], c_quad[3], t_saftey);
        ot_quads.push(c_quad);
    }
    
    Ok(())
}

#[allow(dead_code, unused_assignments)]
fn read_data(file: String, data: &mut Vec<Vec<char>>) -> io::Result<()> {
    let reader = io::BufReader::new(File::open(file)?);

    for line_res in reader.lines() {
        let line = line_res?;
        let mut c_line = Vec::new();

        if line.trim().is_empty() {
            continue;
        }
        for ii in line.chars() {
            c_line.push(ii);
        }
        data.push(c_line);
    }
    Ok(())
}

/// return [i32; 6] is [start_jj, start_ii, velocity_jj, velocity_ii, current_jj, current_ii]
#[allow(dead_code, unused_assignments)]
fn read_data_2(file: String) -> io::Result<Vec<[i32; 6]>> {
    let mut ret = Vec::new();
    let reader = io::BufReader::new(File::open(file)?);

    let mut r_line: [i32; 6] = [0; 6];
    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        } else {
            r_line = [0; 6];
            let mut t_num: i32 = 0;
            let mut neg = false;
            let mut num_inx = 0;
            let mut was_num = false;
            for ii in line.chars() {
                if let Some(num) = ii.to_digit(10) {
                    t_num = t_num * 10 + num as i32;
                    was_num = true;
                } else if ii == '-' {
                    neg = true;
                } else {
                    if was_num {
                        if neg {
                            r_line[num_inx] = 0 - t_num as i32;
                            neg = false;
                        } else {
                            r_line[num_inx] = t_num as i32;
                        }
                        was_num = false;
                        num_inx += 1;
                        t_num = 0;
                    }
                }
            }
            if neg {
                r_line[num_inx] = 0 - t_num as i32;
                neg = false;
            } else {
                r_line[num_inx] = t_num as i32;
            }

            //this puts the starting location into the ongoing location slot
            r_line[4] = r_line[0];
            r_line[5] = r_line[1];

            // for num in r_line {
            //     print!("{num} ");
            // }
            // println!("");
            ret.push(r_line.clone());
        }
    }
    return Ok(ret);
}

/// bound and ret are both going to be (jj, ii)
fn apply_movement(line: &mut [i32; 6], jj_m: i32, ii_m: i32) -> (i32, i32) {
    let mut ret = (line[4] + line[2], line[5] + line[3]);
    //print!("V{},{}::P{},{}::({},{})::[{},{}] ", line[2], line[3], line[4], line[5], ret.0, ret.1, ii_m, jj_m);
    // handling teleportation
    // jj
    if ret.0 < 0 {
        ret.0 = ret.0 + ii_m;
    } else if ret.0 >= ii_m {
        ret.0 = ret.0 - (ii_m - 0);
    }

    // ii
    if ret.1 < 0 {
        ret.1 = ret.1 + jj_m;
    } else if ret.1 >= jj_m {
        ret.1 = ret.1 - (jj_m - 0);
    }
    //println!("= ({},{})->({},{}) ", line[4], line[5], ret.0, ret.1);
    line[4] = ret.0;
    line[5] = ret.1;

    // returns the to be position
    return ret;
}

/// prints the locations
fn print_sa(safty_area: &Vec<Vec<i32>>) {
    //print saftey stuff
    for ii in 0..safty_area.len() {
        if ii == 0 {
            for _ in 0..safty_area[0].len() {
                print!("#");
            }
            println!("");
        }
        for jj in 0..safty_area[ii].len() {
            if safty_area[ii][jj] == 0 {
                print!(".");
            } else {
                print!("{}", safty_area[ii][jj]);
            }
        }
        println!("");
    }
}
//end