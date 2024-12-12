use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn twelve() -> io::Result<()> {
    let mut data = Vec::new();

    match read_data(String::from("data/12/test.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }

    for line in data {
        for ii in line {
            print!("{ii}");
        }
        println!("");
    }
    
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

//end