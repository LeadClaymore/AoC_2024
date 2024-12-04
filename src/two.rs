use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn two_p1() -> io::Result<()> {
    //let mut data:Vec<Vec<i32>> = Vec::new();
    let mut answer = 0;

    let reader = io::BufReader::new(File::open("data/two.txt")?);

    for line_res in reader.lines() {
        let line = line_res?;
        
        if line.trim().is_empty() {
            continue;
        }

        let mut line_data = Vec::new();
        
        //print!("\nline =");
        let parts = line.split(" ");
        for num in parts {
            //print!(" {}", num.parse::<i32>().unwrap_or(-1));
            line_data.push(num.parse::<i32>().unwrap());
        }
        
        let mut checker = -1;
        let mut decender = 0;
        let mut found_unsafe = false;
        for num in line_data {
            // if -1 its the first num so just make it checker
            if checker != -1 {
                let dif = checker - num;
                // if the dif is more then 3 then unsafe
                if dif.abs() > 3 {
                    found_unsafe = true;
                    break;
                // if the decender is 0 this is the second num
                // and we dont know if its decending or assending
                } else if decender == 0 {
                    // if same then unsafe
                    if dif == 0 {
                        found_unsafe = true;
                        break;
                    // sets decending
                    } else if dif > 0 {
                        decender = 1;
                    // sets asending
                    } else if dif < 0 {
                        decender = -1;
                    }
                // third num on
                } else {
                    // if same num then unsafe
                    if dif == 0 {
                        found_unsafe = true;
                        break;
                    // if decending and has been decending
                    } else if dif > 0 && decender == 1 {
                        decender = 1;
                    // if asending and has been decending
                    } else if dif < 0 && decender == -1 {
                        decender = -1;
                    } else {
                        found_unsafe = true;
                        break;
                    }
                }
            }
            checker = num;
        }

        if !found_unsafe {
            answer += 1;
        }
    }
    println!("answer = {}", answer);
    Ok(())
}

#[allow(dead_code, unused_assignments)]
pub fn two_p2() -> io::Result<()> {
    //let mut data:Vec<Vec<i32>> = Vec::new();
    let mut answer = 0;

    let reader = io::BufReader::new(File::open("data/two.txt")?);

    for line_res in reader.lines() {
        let line = line_res?;
        
        if line.trim().is_empty() {
            continue;
        }

        let mut line_data = Vec::new();
        
        //print!("\nline =");
        let parts = line.split(" ");
        for num in parts {
            //print!(" {}", num.parse::<i32>().unwrap_or(-1));
            line_data.push(num.parse::<i32>().unwrap());
        }
        
        if check_line(&line_data) {
            answer += 1;
        } else {
            let mut damper_worked = false;
            for ii in 0..line_data.len() {
                let mut new_data = Vec::new();
                for jj in 0..line_data.len() {
                    if ii != jj {
                        new_data.push(line_data[jj]);
                    }
                }

                if check_line(&new_data) {
                    damper_worked = true;
                    break;
                }
            }
            if damper_worked {
                answer += 1;
            }
        }
        
    }
    println!("answer = {}", answer);
    Ok(())
}

#[allow(dead_code, unused_assignments)]
fn check_line(line_data: &Vec<i32>) -> bool {
    let mut checker = -1;
    let mut decender = 0;

    for &num in line_data {
        // if -1 its the first num so just make it checker
        if checker != -1 {
            let dif = checker - num;
            // if the dif is more then 3 then unsafe
            if dif.abs() > 3 {
                return false;
            // if the decender is 0 this is the second num
            // and we dont know if its decending or assending
            } else if decender == 0 {
                // if same then unsafe
                if dif == 0 {
                    return false;
                // sets decending
                } else if dif > 0 {
                    decender = 1;
                // sets asending
                } else if dif < 0 {
                    decender = -1;
                }
            // third num on
            } else {
                // if same num then unsafe
                if dif == 0 {
                    return false;
                // if decending and has been decending
                } else if dif > 0 && decender == 1 {
                    decender = 1;
                // if asending and has been decending
                } else if dif < 0 && decender == -1 {
                    decender = -1;
                } else {
                    return false;
                }
            }
        }
        checker = num;
    }

    return true;
}