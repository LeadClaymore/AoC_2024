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
                if let Some(mut plot) = calc_plot(&mut used_data, (ii, jj), temp_c) {
                    let (area, prim, price) = process_plot(&mut plot);
                    println!("plot {ii}, {jj} has area: {area}, {prim} as {temp_c}");
                    total_area += area;
                    total_prim += prim;
                    total_price += price;

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

//my new aproach is to return a position of each plot, to later calculate the area & parameter

/// returns (area, perimeter)
#[allow(dead_code, unused_assignments)]
fn calc_plot(used_data: &mut Vec<Vec<char>>, p: (usize, usize), c: char) -> Option<Vec<(usize, usize)>> {
    if used_data[p.0][p.1] == '-' {
        //println!("error at ({}, {}) within {}, found '-' ", p.0, p.1, c);
        return None;
    }

    // this means its a edge
    if used_data[p.0][p.1] != c {
        print!("({},{}) ", p.0, p.1);
        return None;
    }

    //set this to in use
    used_data[p.0][p.1] = '-';
    let mut ret = Vec::new();

    // up
    if p.0 > 0 {
        // this if let would have been part of the last if but its not available
        if let Some(mut t_vec) = calc_plot(used_data, (p.0 - 1, p.1), c) {
            ret.append(&mut t_vec);
        }
    }
    
    // down
    if p.0 < used_data.len() - 1 {
        if let Some(mut t_vec) = calc_plot(used_data, (p.0 + 1, p.1), c) {
            ret.append(&mut t_vec);
        }
    }
    
    // left
    if p.1 > 0 {
        if let Some(mut t_vec) = calc_plot(used_data, (p.0, p.1 - 1), c) {
            ret.append(&mut t_vec);
        }
    }

    // right
    if p.1 < used_data[0].len() - 1 {
        if let Some(mut t_vec) = calc_plot(used_data, (p.0, p.1 + 1), c) {
            ret.append(&mut t_vec);
        }
    }

    //after all the sides are done, set this to used
    //used_data[p.0][p.1] = '.';

    return Some(ret);
}

/// returns (area, prim, price) and the vec taken is (ii, jj)
fn process_plot(plot: &mut Vec<(usize, usize)>) -> (u32, u32, u32) {
    let (mut area, mut prim, mut price) = (0, 0, 0);
    //let mut unique_plot: Vec<(usize, usize)> = Vec::new();
    let (mut min_ii, mut max_ii) = (0, 0);
    let (mut min_jj, mut max_jj) = (0, 0);
    for ii in 0..plot.len() {
        if ii == 0 {
            min_ii = plot[0].0;
            min_jj = plot[0].1;
            max_ii = plot[0].0;
            max_jj = plot[0].1;
        }
        // let mut unique = true;
        // for jj in 0..unique_plot.len() {
        //     if  plot[ii].0 == unique_plot[jj].0 && 
        //         plot[ii].1 == unique_plot[jj].1 
        //     {
        //         unique = false;
        //     }
        // }
        // if unique {
            if plot[ii].0 < min_ii {
                min_ii = plot[ii].0;
            }
            if plot[ii].1 < min_jj {
                min_jj = plot[ii].1;
            }
            if plot[ii].0 > max_ii {
                max_ii = plot[ii].0;
            }
            if plot[ii].1 > max_jj {
                max_jj = plot[ii].1;
            }
            //unique_plot.push(plot[ii]);
        //}
    }

    let (ofst_ii, ofst_jj) = (max_ii - min_ii, max_jj - min_jj);

    let mut inv_plot = Vec::new();
    for _ii in 0..=ofst_ii {
        let mut inv_line = Vec::new();
        for _jj in 0..=ofst_jj {
            inv_line.push(false);
        }
        inv_plot.push(inv_line);
    }

    for pos in plot {
        inv_plot[pos.0 - ofst_ii][pos.1 - ofst_jj] = true;
    }

    // Ok so now I have an map of the plot out of booleans
    // im going to search the first row untill I hit a true
    // this will get me a spot on the plot with 2 key features
    // because its a tight fit on the graph aka no blank rows of columns,
    // the first part I hit will be 1 on the edge of the plot
    // and 2 a corner aka the start of a fence bulk purchase
    // because of this I can start a fence there only knowing the starting location to join another fence to it
    // because otherwise we would need to trace the other side of the fence for where the fence started
    // because we started in the upper left moving right, the first block we encounter will be either:
    // a corner moving from up to right, or a penincula moving from up to right then down
    // either way we start with moving right, and we need to check the next place in the fence
    // to start we keep a running total of the sides starting with the 1 we are on 
    // and we keep the curring cursors dirrection then
    // once we find the layout by checking some of the surounding tiles, we either:
    // a move to the next block if its a straight line, or 
    // b change the dirrection to the next block and add 1 to the total sides
    // once we are at the starting location we have to check if its a pinencula pointing left then
    // +------...       +------...
    // |######...  vs   |######...
    // +------...       |######...
    // if so we add 1 to the count (see above for diagram),
    // otherwise we just stop

    let mut pos = (0, 0);
    let mut found = false;
    for ii in 0..inv_plot[0].len() {
        if inv_plot[0][ii] {
            pos.1 = ii;
            found = true;
            break;
        }
    }

    if !found {
        println!("stuff not found");
        return (0, 0, 0);
    }

    // dirs is right, up, left, down
    let mut dirs = Dir::Right;
    let side_count = 1;
    //TODO
    // loop {
    //     if 
    //     break;
    // }
    let area  = inv_plot.iter().flatten().filter(|&& val| val).count() as u32;
    return (area, prim, price);
}

#[allow(dead_code, unused_assignments)]
enum Dir {
    Right,
    Up,
    Left,
    Down,
}

#[allow(dead_code, unused_assignments)]
struct Adj {
    u: Option<bool>,
    d: Option<bool>,
    l: Option<bool>,
    r: Option<bool>,
    ul: Option<bool>,
    ur: Option<bool>,
    dl: Option<bool>,
    dr: Option<bool>,
}

#[allow(dead_code, unused_assignments)]
impl Adj {
    fn new() -> Adj {
        return Adj {
            u: None,
            d: None,
            l: None,
            r: None,
            ul: None,
            ur: None,
            dl: None,
            dr: None,
        }
    }

    /// pos = (ii, jj)
    fn get_suroundings(&mut self, data: &Vec<Vec<bool>>, pos: (usize, usize)) {
        let (max_ii, max_jj) = (data.len() - 1, data[0].len() - 1);
        let (sl, sr, su, sd) = (pos.1 > 0, pos.1 < max_jj, pos.0 > 0, pos.1 < max_ii);

        if su {
            self.u = Some(data[pos.0 - 1][pos.1]);
            if sl {
                self.ul = Some(data[pos.0 - 1][pos.1 - 1]);
            }
            if sr {
                self.ur = Some(data[pos.0 - 1][pos.1 + 1]);
            }
        }
        if sd {
            self.d = Some(data[pos.0 + 1][pos.1]);
            if sl {
                self.dl = Some(data[pos.0 + 1][pos.1 - 1]);
            }
            if sr {
                self.dr = Some(data[pos.0 + 1][pos.1 + 1]);
            }
        }
        if sl {
            self.u = Some(data[pos.0][pos.1 - 1]);
        }
        if sr {
            self.u = Some(data[pos.0][pos.1 + 1]);
        }
    }

    fn valid_dir(&self, c_dir: Dir) {

    }
}
//end