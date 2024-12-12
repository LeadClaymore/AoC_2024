use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn twelve() -> io::Result<()> {
    let mut data = Vec::new();

    match read_data(String::from("data/12/test.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }

    for line in data.iter() {
        for ii in line.iter() {
            print!("{ii}");
        }
        println!("");
    }

    calc_data(&data);
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

#[allow(dead_code, unused_assignments)]
fn calc_data(data: &Vec<Vec<char>>) {
    let mut used_data = data.clone();
    let mut total_area = 0;
    let mut total_prim = 0;
    let mut total_price = 0;
    for ii in 0..data.len() {
        for jj in 0..data[ii].len() {
            // this 
            if used_data[ii][jj] != '.' {
                let temp_c = used_data[ii][jj];
                if let Some((area, prim)) = calc_plot(&mut used_data, (ii, jj), temp_c) {
                    println!("plot {ii}, {jj} has area: {area}, {prim} as {temp_c}");
                    total_area += area;
                    total_prim += prim;
                    total_price += area * prim;

                    for t_ii in 0..data.len() {
                        for t_jj in 0..data[t_ii].len() {
                            if used_data[t_ii][t_jj] == '-' {
                                used_data[t_ii][t_jj] ='.';
                            }
                        }
                    }
                } else {
                    println!("error at ({}, {}) within {}, found '-' ", ii, jj, used_data[ii][jj]);
                }
            }
        }
    }
    println!("total prim {total_prim}, total area {total_area}, with the price of {total_price}");
}

// current iteration goes off of an refrence so changes done elsewhere do not happen,
// I had the same problem in the trail problem, ill have to return positions of each to not dupe
// or get better at multithreaded opperation so I could use an constant update rather then a ref

//nvm I found an hacky way of fixing it

/// returns (area, perimeter)
#[allow(dead_code, unused_assignments)]
fn calc_plot(used_data: &mut Vec<Vec<char>>, p: (usize, usize), c: char) -> Option<(u32, u32)> {
    if used_data[p.0][p.1] == '-' {
        //println!("error at ({}, {}) within {}, found '-' ", p.0, p.1, c);
        return None;
    }

    // this means its a edge
    if used_data[p.0][p.1] != c {
        print!("({},{}) ", p.0, p.1);
        return Some((0, 1));
    }

    //set this to in use
    used_data[p.0][p.1] = '-';
    let mut r_a = 0;
    let mut r_p = 0;

    // up
    if p.0 > 0 {
        // this if let would have been part of the last if but its not available
        if let Some((t_a, t_p)) = calc_plot(used_data, (p.0 - 1, p.1), c) {
            r_a += t_a;
            r_p += t_p;
        }
    } else {
        r_a += 0;
        r_p += 1;
    }
    
    // down
    if p.0 < used_data.len() - 1 {
        if let Some((t_a, t_p)) = calc_plot(used_data, (p.0 + 1, p.1), c) {
            r_a += t_a;
            r_p += t_p;
        }
    } else {
        r_a += 0;
        r_p += 1;
    }
    
    // left
    if p.1 > 0 {
        if let Some((t_a, t_p)) = calc_plot(used_data, (p.0, p.1 - 1), c) {
            r_a += t_a;
            r_p += t_p;
        }
    } else {
        r_a += 0;
        r_p += 1;
    }

    // right
    if p.1 < used_data[0].len() - 1 {
        if let Some((t_a, t_p)) = calc_plot(used_data, (p.0, p.1 + 1), c) {
            r_a += t_a;
            r_p += t_p;
        }
    } else {
        r_a += 0;
        r_p += 1;
    }

    //after all the sides are done, set this to used
    //used_data[p.0][p.1] = '.';

    // +1 because it is it self
    return Some((r_a + 1, r_p));
}
//end