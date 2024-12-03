//use serde::Deserialize;
//use serde_scan::scan;
use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;

pub fn one_p1() -> io::Result<()> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();
    let mut answer = 0;

    // This gets a buffer for the text file, the ? is to continue if it does not fail.
    // If im not mistaken it returns the result if it fails
    let reader = io::BufReader::new(File::open("data/one.txt")?);

    for line_res in reader.lines() {
        // this gets if it can atualy read the line from the file that we got returns error if not
        let line = line_res?;
        
        // my program would crash at the end of the file if this did not happen
        // I turns out that there was some spaces at the end. I could have fixed it by deleting them. this looks better to me
        if line.trim().is_empty() {
            continue;
        }

        // this uses the serde scan from_str to take the line and turn it into 2 sepreate numbers
        let (left_num, right_num): (i32, i32) = serde_scan::from_str(&line).unwrap();

        //testing this prints the line
        //println!("{}", line);

        left_nums.push(left_num);
        right_nums.push(right_num);
    }

    // testing something
    // let (mut avr_l, mut avr_r): (u128, u128) = (0, 0);
    // for count in 0..1000 {
    //     avr_l += left_nums[count] as u128;
    //     avr_r += right_nums[count] as u128;
    // }
    // println!("avr answer: {} from {} - {}", avr_l - avr_r, avr_l, avr_r);

    left_nums.sort();
    right_nums.sort();

    for count in 0..1000 {
        let t = (left_nums[count] - right_nums[count]).abs();
        //println!("count: {}, {} - {} = {}", count, left_nums[count], right_nums[count], t);
        answer += t;
    }
    
    println!("answer: {}", answer);

    Ok(())
}

pub fn one_p2() -> io::Result<()> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();
    let mut left_rep = Vec::new();
    let mut answer = 0;

    let reader = io::BufReader::new(File::open("data/one.txt")?);

    for line_res in reader.lines() {
        let line = line_res?;
        
        if line.trim().is_empty() {
            continue;
        }

        let (left_num, right_num): (i32, i32) = serde_scan::from_str(&line).unwrap();

        left_nums.push(left_num);
        right_nums.push(right_num);
        left_rep.push(0);
    }
    
    for ii in 0..1000 {
        for jj in 0..1000 {
            if left_nums[ii] == right_nums[jj] {
                left_rep[ii] += 1;
            }
        }
    }

    for ii in 0..1000 {
        answer += left_rep[ii] * left_nums[ii];
    }
    println!("answer: {}", answer);
    Ok(())
}
