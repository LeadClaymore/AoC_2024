use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn nine() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;

    match read_data(String::from("data/nine_test.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    print_data(&data);
    Ok(())
}

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

fn print_data(data: &Vec<Vec<char>>) {
    data.iter().for_each(|line| {
        line.iter().for_each(|letter| {
            print!("{letter}");
        });
        println!("");
    });
}