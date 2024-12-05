use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn five_p1() -> io::Result<()> {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut answer = 0;

    let reader = io::BufReader::new(File::open("data/four.txt")?);

    for line_res in reader.lines() {
        let line = line_res?;
        let mut c_line = Vec::new();
        if line.trim().is_empty() {
            
            continue;
        } else {
            for ii in line.chars() {
                //print!("{}", ii);-
                c_line.push(ii);
            }
            //println!("");
            data.push(c_line);
        }
    }

    println!("answer = {}", answer);
    Ok(())
}