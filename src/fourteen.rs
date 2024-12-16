use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn fourteen() -> io::Result<()> {
    let mut data = match read_data_2(String::from("data/14/test.txt")) {
        Ok(stuff) =>  {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };
    
    //TODO
    //println!("answer = {}", );
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
fn read_data_2(file: String) -> io::Result<Vec<[i32; 4]>> {
    let mut ret = Vec::new();
    let reader = io::BufReader::new(File::open(file)?);

    let mut r_line: [i32; 4] = [0; 4];
    for line_res in reader.lines() {
        let line = line_res?;
        //println!("{}",line);
        if line.trim().is_empty() {
            continue;
        } else {
            r_line = [0; 4];
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
            for num in r_line {
                print!("{num} ");
            }
            println!("");
            ret.push(r_line.clone());
        }
    }
    return Ok(ret);
}
//end