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

#[allow(dead_code, unused_assignments)]
pub fn five_p2() -> io::Result<()> {
    let mut rules = Vec::new();
    let mut data = Vec::new();
    let mut answer = 0;

    let ret = read_data(String::from("data/five.txt"), &mut data, &mut rules);
    if ret.is_err() {
        return ret;
    }

    // for &rule in &rules {
    //     println!("{}|{}", rule.0, rule.1);
    // }
    // for line in data.clone() {
    //     for &num in &line {
    //         print!("{num}, ");
    //     }
    //     println!("");
    // }

    //let mut temp_line = Vec::new();
    let mut bad_lines = Vec::new();
    check_lines(&mut data, &mut rules, &mut bad_lines);
    for (mut line, mut errors) in bad_lines {
        print_line("Starting line: ", &line);
        let mut found = false;
        for ((rn_0, rn_1), (ri_0, ri_1)) in &errors {
            println!("{rn_0},{rn_1} at {ri_0},{ri_1}")
        }
        while !found {
            let ((rn_0, _rn_1), (ri_0, ri_1)) = errors[0];
            line.remove(ri_0 as usize);
            line.insert(ri_1 as usize, rn_0);
            print_line("tried line: ", &line);
            let temp_num = check_line(&mut line, &rules, &mut errors);
            if temp_num.is_some() {
                println!("solved to {}", temp_num.unwrap());
                answer += temp_num.unwrap();
                found = true;
            }
        }
    }
    println!("answer = {}", answer);
    Ok(())
}

fn read_data(file: String, data: &mut Vec<Vec<u32>>, rules: &mut Vec<(u32, u32)>) -> io::Result<()> {
    let reader = io::BufReader::new(File::open(file)?);

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
                        c_line.push(num);
                        num = 0;
                    } else {
                        num = num * 10 + ii.to_digit(10).unwrap();
                    }
                }
            }
            if mode {
                cur_rule.1 = num;
                rules.push(cur_rule);
            } else {
                c_line.push(num);
                data.push(c_line);
            }
        }
    }
    Ok(())
}

fn check_lines(data: &mut Vec<Vec<u32>>, rules: &mut Vec<(u32, u32)>, bad_lines: &mut Vec<(Vec<u32>, Vec<((u32, u32), (u32, u32))>)>) -> u32 {
    let mut answer = 0;
    for line in data {
        let mut rules_broken: Vec<((u32, u32), (u32, u32))> = Vec::new();
        let mut line_good = false;
        let mut line_bad = false;
        for &mut rule in &mut *rules {
            let mut line_might_be_bad = false;
            let mut r0_found = false;
            let mut r_inxs = (0, 0);
            let mut inx = 0;
            for &mut num in &mut *line {
                if num == rule.0 {
                    r_inxs.0 = inx;
                    if line_might_be_bad {
                        rules_broken.push((rule.clone(), r_inxs.clone()));
                        line_bad = true;
                    }
                    r0_found = true;
                } else if num == rule.1 {
                    r_inxs.1 = inx;
                    if r0_found {
                        line_good = true;
                    } else {
                        line_might_be_bad = true;
                    }
                }
                inx += 1;
            }
        }
        if line_good && !line_bad {
            answer += line[line.len() / 2];
        } else {
            bad_lines.push((line.clone(), rules_broken.clone()));
        }
    }
    println!("p1 answer: {answer}");
    answer
}

fn check_line(line: &mut Vec<u32>, rules: &Vec<(u32, u32)>, rules_broken: &mut Vec<((u32, u32), (u32, u32))>) -> Option<u32> {
    let mut temp_rules_broken:  Vec<((u32, u32), (u32, u32))> = Vec::new();
    let mut line_good = false;
    let mut line_bad = false;
    for &rule in & *rules {
        let mut line_might_be_bad = false;
        let mut r0_found = false;
        let mut r_inxs = (0, 0);
        let mut inx = 0;
        for &mut num in &mut *line {
            if num == rule.0 {
                r_inxs.0 = inx;
                if line_might_be_bad {
                    temp_rules_broken.push((rule.clone(), r_inxs.clone()));
                    line_bad = true;
                }
                r0_found = true;
            } else if num == rule.1 {
                r_inxs.1 = inx;
                if r0_found {
                    line_good = true;
                } else {
                    line_might_be_bad = true;
                }
            }
            inx += 1;
        }
    }
    if line_good && !line_bad {
        return Some(line[line.len() / 2]);
    } else {
        rules_broken.clear();
        for b_rules in temp_rules_broken {
            rules_broken.push(b_rules.clone());
        }
        return None;
    }
}

fn print_line(start: &str, line: &Vec<u32>) {
    print!("{}", start);
    for &num in line {
        print!("{num}, ");
    }
    println!("");
}

// fn change_bad_lines(bad_lines: &mut Vec<(Vec<u32>, Vec<((u32, u32), (u32, u32))>)>) {
//     for lines in bad_lines {
//         let ((rn_0, rn_1), (ri_0, ri_1)) = lines.1[0];
//         //println!("problem {rn_0},{rn_1}");
//         lines.0.remove(ri_0 as usize);
//         lines.0.insert(ri_1 as usize, rn_0);
//     }
// }

// fn print_bad_lines(bad_lines: &Vec<(Vec<u32>, Vec<((u32, u32), (u32, u32))>)>) {
//     for (line, _probs) in bad_lines {
//         for num in line {
//             print!("{num} ");
//         }
//         println!("");
//     }
// }