use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn seventeen() -> io::Result<()> {
    let (mut prog, mut reg_a, mut reg_b, mut reg_c) = match read_data(String::from("data/17/data.txt")) {
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

    let mut inx = 0;
    while inx < prog.len() {
        let l_op = prog[inx + 1];
        let c_op = match l_op {
            0 => Some(0),
            1 => Some(1),
            2 => Some(2),
            3 => Some(3),
            4 => Some(reg_a),
            5 => Some(reg_b),
            6 => Some(reg_c),
            _ => None,
        };

        match prog[inx] {
            0 => {
                reg_a = adv(reg_a, c_op.unwrap());
            },
            1 => {
                reg_b = bxl(reg_b, l_op);
            },
            2 => {
                reg_b = bst(c_op.unwrap());
            },
            3 => {
                if let Some(n_inx) = jnz(reg_a, l_op) {
                    inx = n_inx;
                    continue;
                }
            },
            4 => {
                reg_b = bxc(reg_b, reg_c);
            },
            5 => {
                out(c_op.unwrap());
            },
            6 => {
                reg_b = bdv(reg_a, c_op.unwrap());
            },
            7 => {
                reg_c = cdv(reg_a, c_op.unwrap());
            },
            _ => {
                println!("error here is the program: ");
                print_inst(&prog, reg_a, reg_b, reg_c, inx);
            },
        }
        inx += 2;
    }
    println!("");
    print_inst(&prog, reg_a, reg_b, reg_c, inx);
    Ok(())
}

/// prints all relevent data
#[allow(dead_code, unused_assignments)]
fn print_inst(prog: &Vec<u32>, reg_a: u32, reg_b: u32, reg_c: u32, inx: usize) {
    println!("reg A: {}", reg_a);
    println!("reg B: {}", reg_b);
    println!("reg C: {}", reg_c);
    println!("inx: {}", inx);
    print!("Prog: ");
    for nn in 0..prog.len() {
        print!("{},", prog[nn]);
    }
    println!("");
}

/// 0, put this in a
#[allow(dead_code, unused_assignments)]
fn adv(reg_a: u32, c_op: u32) -> u32 {
    let two: u32 = 2;
    return reg_a / (two.pow(c_op) as u32);
}

/// 1, put this in b
#[allow(dead_code, unused_assignments)]
fn bxl(reg_b: u32, l_op: u32) -> u32 {
    return reg_b ^ l_op;
}

/// 2, put this in b
#[allow(dead_code, unused_assignments)]
fn bst(c_op: u32) -> u32 {
    return c_op % 8;
}

/// 3, if none do nothing, if some jump inx to this and dont increment by 2
#[allow(dead_code, unused_assignments)]
fn jnz(reg_a: u32, l_op: u32) -> Option<usize> {
    if reg_a == 0 {
        return None;
    } else {
        return Some(l_op as usize);
    }
}

/// 4 put this in b
#[allow(dead_code, unused_assignments)]
fn bxc(reg_b: u32, reg_c: u32) -> u32 {
    return reg_b ^ reg_c;
}

/// 5 prints a value, idk how /n is meant to work
#[allow(dead_code, unused_assignments)]
fn out(c_op: u32) {
    print!("{},", c_op % 8);
}

/// 6 put this in b
#[allow(dead_code, unused_assignments)]
fn bdv(reg_a: u32, c_op: u32) -> u32 {
    let two: u32 = 2;
    return reg_a / (two.pow(c_op) as u32);
}

/// 7 put this in c
#[allow(dead_code, unused_assignments)]
fn cdv(reg_a: u32, c_op: u32) -> u32 {
    let two: u32 = 2;
    return reg_a / (two.pow(c_op) as u32);
}

// /// # put this in #
// #[allow(dead_code, unused_assignments)]
// fn ###(reg_b: u32, l_op: u32) -> u32 {
//     return reg_b ^ l_op;
// }



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