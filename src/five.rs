use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn five_p1() -> io::Result<()> {
    let mut rules = Vec::new();
    let mut data = Vec::new();
    let mut answer = 0;

    let reader = io::BufReader::new(File::open("data/five.txt")?);

    let mut mode = true;
    for line_res in reader.lines() {
        let line = line_res?;
        let mut c_line = Vec::new();
        
        let mut num = 0;
        let mut cur_rule = (0, 0);
        if line.trim().is_empty() {
            mode = false;
            continue;
        } else {
            for ii in line.chars() {
                //print!("{}", ii);
                if mode {
                    if ii == '|' {
                        cur_rule.0 = num;
                        num = 0;
                    } else {
                        num = num * 10 + ii.to_digit(10).unwrap();
                    }
                } else {
                    if ii == ',' {
                        //print!("{num},");
                        c_line.push(num);
                        num = 0;
                    } else {
                        num = num * 10 + ii.to_digit(10).unwrap();
                    }
                }
            }
            if mode {
                cur_rule.1 = num;
                //println!("{},{}", cur_rule.0, cur_rule.1);
                rules.push(cur_rule);
            } else {
                //println!("");
                c_line.push(num);
                data.push(c_line);
            }
            //println!("");
        }
    }

    // for &rule in &rules {
    //     println!("{}|{}", rule.0, rule.1);
    // }
    // println!("");
    // for line in data {
    //     for &num in &line {
    //         print!("{num},");
    //     }
    //     println!("");
    // }

    for line in data {
        let mut line_good = false;
        let mut line_bad = false;
        //let mut mid_inx = 0;
        //print!("line:");
        for &rule in &rules {
            let mut line_might_be_bad = false;
            let mut r0_found = false;
            for &num in &line {
                //print!("{num}");
                if num == rule.0 {
                    if line_might_be_bad {
                        line_bad = true;
                    }
                    r0_found = true;
                } else if num == rule.1 {
                    //print!("r1 found {num}");
                    if r0_found {
                        line_good = true;
                    } else {
                        line_might_be_bad = true;
                    }
                }
            }
            //println!("");
        }
        //println!("{mid_inx}, {mid_num}");
        if line_good && !line_bad {
            //println!("line that starts with {} is good", line[0]);
            answer += line[line.len() / 2];
        }
    }

    println!("answer = {}", answer);
    Ok(())
}