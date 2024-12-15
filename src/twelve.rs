use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn twelve() -> io::Result<()> {
    let mut data = Vec::new();

    match read_data(String::from("data/12/data.txt"), &mut data) {
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
    let mut something_failed = (-1, -1);
    for ii in 0..data.len() {
        for jj in 0..data[ii].len() {
            // this 
            if used_data[ii][jj] != '.' {
                let temp_c = used_data[ii][jj].clone();
                if let Some(plot) = calc_plot(&mut used_data, (ii, jj), temp_c) {
                    //TODO calc plot is finding stuff but its not getting to process_plot
                    let (area, prim, price) = process_plot(&plot);
                    println!("plot {ii}, {jj} has area: {area}, prim: {prim} as {temp_c}");
                    total_area += area;
                    total_prim += prim;
                    total_price += price;
                    if price == 0 {
                        something_failed = (ii as i32, jj as i32);
                    }
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
    if something_failed.0 == -1 && something_failed.1 == -1 {
        println!("total prim {total_prim}, total area {total_area}, with the price of {total_price}");
    } else {
        println!("fail at {}, {}", something_failed.0, something_failed.1)
    }
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
        //print!("({},{}) ", p.0, p.1);
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

    ret.push(p);
    return Some(ret);
}

#[allow(dead_code, unused_assignments)]
/// returns (area, prim, price) and the vec taken is (ii, jj)
fn process_plot(plot: &Vec<(usize, usize)>) -> (u32, u32, u32) {
    //let mut unique_plot: Vec<(usize, usize)> = Vec::new();
    let (mut min_ii, mut max_ii) = (0, 0);
    let (mut min_jj, mut max_jj) = (0, 0);
    for ii in 0..plot.len() {
        //print!("A");
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
    //println!("\nmaii {}, miii {}, majj {}, mijj {}, oii {}, ojj {},",max_ii, min_ii, max_jj, min_jj, ofst_ii, ofst_jj);

    let mut inv_plot = Vec::new();
    for _ii in 0..=ofst_ii {
        let mut inv_line = Vec::new();
        for _jj in 0..=ofst_jj {
            inv_line.push(false);
        }
        inv_plot.push(inv_line);
    }

    for pos in plot {
        inv_plot[pos.0 - min_ii][pos.1 - min_jj] = true;
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
    for jj in 0..inv_plot[0].len() {
        if inv_plot[0][jj] {
            pos.1 = jj;
            found = true;
            break;
        }
    }

    println!("");
    for line in &inv_plot {
        for &e in line {
            if e {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    if !found {
        println!("stuff not found");
        return (0, 0, 0);
    }

    let mut checked = Vec::new();
    let mut prim = 0;
    let mut area = 0;
    loop {
        let s_pos = pos;
        let mut c_dir = Dir::Right;
        let mut adj = Adj::new();
        adj.get_suroundings(&inv_plot, pos);
        //adj.print_suroundings(Some(&c_dir));
        let (mut r_ii, mut r_jj);
        if let Some(ret) = adj.valid_dir(&c_dir) {
            checked.push((c_dir.clone(), pos.clone())); // this is for later checking if this has been done
            let t_dir = c_dir.clone();
            (c_dir, (r_ii, r_jj)) = ret;
            pos.0 = (pos.0 as i8 + r_ii) as usize;
            pos.1 = (pos.1 as i8 + r_jj) as usize;
            if t_dir != c_dir {
                prim += 1;
            }
        } else {
            println!("found but starting dir not found");
            return (0, 0, 0);
        }

        //print!("(ii,jj) ");
        let mut l_inx = 0;
        loop {
            //print!("({},{}) ", pos.0, pos.1);

            adj.get_suroundings(&inv_plot, pos);
            //adj.print_suroundings(Some(&c_dir));
            if let Some(ret) = adj.valid_dir(&c_dir) {
                let t_dir = c_dir;

                checked.push((c_dir.clone(), pos.clone())); // this is for later checking if this has been done
                (c_dir, (r_ii, r_jj)) = ret;
                pos.0 = (pos.0 as i8 + r_ii) as usize;
                pos.1 = (pos.1 as i8 + r_jj) as usize;
                if t_dir != c_dir {
                    prim += 1;
                }
            } else {
                println!("error at prim == {}", prim);
                return (0, 0, 0);
            }
            if pos == s_pos && c_dir == Dir::Right && l_inx != 0 {
                break;
            }
            l_inx += 1;
        }
        checked.push((c_dir.clone(), pos.clone())); // this is for later checking if this has been done

        let mut more_exist = false;
        // we dont need 0 because we started with 0 and we need the inside of loops not outside
        for ii in 1..inv_plot.len() {
            for jj in 0..inv_plot[ii].len() {
                if 
                    inv_plot[ii][jj] && 
                    !checked.contains(&(Dir::Right, (ii, jj))) && 
                    !inv_plot[ii - 1][jj]
                {
                    more_exist = true;
                    pos = (ii, jj);
                    
                    checked.push((Dir::Right, pos.clone())); // this is for later checking if this has been done
                    println!("another one {ii},{jj} prim {prim}");
                    break;
                }
            }
            if more_exist {
                break;
            }
        }
        if !more_exist {
            break;
        }
    }
    //println!("");

    area = inv_plot.iter().flatten().filter(|&& val| val).count() as u32;
    let price = area * prim;
    return (area, prim, price);
}

//TODO ok so I have a problem that objects with holes in them, see below, the algorithim does not get the fences within.
// xxx
// xox
// xxx
// in this example it will get 4 for the pr

#[allow(dead_code, unused_assignments)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    Right,
    Up,
    Left,
    Down,
}

#[allow(dead_code, unused_assignments)]
impl Dir {
    fn p_str(&self) -> &str {
        return match self {
            Dir::Right => "R",
            Dir::Down => "D",
            Dir::Left => "L",
            Dir::Up => "U",
        }
    }
}

#[allow(dead_code, unused_assignments)]
//#[derive(PartialEq, Eq)]
struct Adj {
    u: bool,
    d: bool,
    l: bool,
    r: bool,
    ul: bool,
    ur: bool,
    dl: bool,
    dr: bool,
    pos: (usize, usize),
}

#[allow(dead_code, unused_assignments)]
impl Adj {
    fn new() -> Adj {
        return Adj {
            u: false,
            d: false,
            l: false,
            r: false,
            ul: false,
            ur: false,
            dl: false,
            dr: false,
            pos: (0, 0),
        }
    }

    fn print_suroundings(&self, c_dir: Option<&Dir>) {
        if let Some(dir) = c_dir {
            print!("{} ", dir.p_str());
        }
        println!("({}, {}),\nul:{}, u:{}, ur:{}, \nl:{}, \tr:{}, \ndl:{}, d:{}, dr:{}", 
            self.pos.0, self.pos.1, self.ul, self.u, self.ur, self.l, self.r, self.dl, self.d, self.dr);
    }

    /// pos = (ii, jj)
    fn get_suroundings(&mut self, data: &Vec<Vec<bool>>, pos: (usize, usize)) {
        self.ul = false;
        self.u = false;
        self.ur = false;
        self.l = false;
        self.r = false;
        self.dl = false;
        self.d = false;
        self.dr = false;
        self.pos = pos;

        let (max_ii, max_jj) = (data.len() - 1, data[0].len() - 1);
        let sl = pos.1 > 0;
        let sr = pos.1 < max_jj;
        let su = pos.0 > 0;
        let sd = pos.0 < max_ii;

        if su {
            self.u = data[pos.0 - 1][pos.1];
            if sl {
                self.ul = data[pos.0 - 1][pos.1 - 1];
            }
            if sr {
                self.ur = data[pos.0 - 1][pos.1 + 1];
            }
        }
        if sd {
            self.d = data[pos.0 + 1][pos.1];
            if sl {
                self.dl = data[pos.0 + 1][pos.1 - 1];
            }
            if sr {
                self.dr = data[pos.0 + 1][pos.1 + 1];
            }
        }
        if sl {
            self.l = data[pos.0][pos.1 - 1];
        }
        if sr {
            self.r = data[pos.0][pos.1 + 1];
        }
    }

    /// returns Some(Dir, (ii, jj))
    fn valid_dir(&self, c_dir: &Dir) -> Option<(Dir, (i8, i8))> {
        return match c_dir {
            Dir::Right => {
                if 
                    // right is true
                    self.r &&
                    //up right is false
                    !self.ur
                {
                    Some((Dir::Right, (0, 1)))
                } else if 
                    // right is true
                    self.r &&
                    // up right is true
                    self.ur &&
                    // up is false
                    !self.u 
                {
                    Some((Dir::Up, (-1, 1)))
                } else if 
                    // right is false
                    !self.r
                {
                    Some((Dir::Down, (0, 0)))
                } else {
                    None
                }
            },
            Dir::Up => {
                if 
                    // up is true
                    self.u &&
                    // up left is false
                    !self.ul 
                {
                    Some((Dir::Up, (-1, 0)))
                } else if 
                    // up is true
                    self.u &&
                    // up left is true
                    self.ul &&
                    // left is false
                    !self.l 
                {
                    Some((Dir::Left, (-1, -1)))
                } else if 
                    // up is false
                    !self.u
                {
                    Some((Dir::Right, (0, 0)))
                } else {
                    None
                }
            },
            Dir::Left => {
                if 
                    // left is true
                    self.l &&
                    // down left is false
                    !self.dl
                {
                    Some((Dir::Left, (0, -1)))
                } else if 
                    // left is true
                    self.l &&
                    // down left is true
                    self.dl &&
                    // down is false
                    !self.d 
                {
                    Some((Dir::Down, (1, -1)))
                } else if 
                    // left is false
                    !self.l
                {
                    Some((Dir::Up, (0, 0)))
                } else {
                    None
                }
            },
            Dir::Down => {
                if 
                    // Down is true
                    self.d &&
                    // down right is false
                    !self.dr 
                {
                    Some((Dir::Down, (1, 0)))
                } else if 
                    // down is true
                    self.d &&
                    // down right is true
                    self.dr &&
                    // right is false
                    !self.r
                {
                    Some((Dir::Right, (1, 1)))
                } else if 
                    // down is false
                    !self.d
                {
                    Some((Dir::Left, (0, 0)))
                } else {
                    None
                }
            },
        };
    }
}
//end