use std::fs::File;
use std::io::{self, BufRead};
use std::thread;

#[allow(dead_code, unused_assignments)]
pub fn eleven() -> io::Result<()> {
    let mut data = Vec::new();

    //match read_data(String::from("data/eleven/data.txt"), &mut data) {
    match read_data(String::from("C:/Users/Clayton Ross/Desktop/Rust/AoC_2024/data/eleven/data.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    //print_data(&data);
    let mut processed_data = process_data(&data[0]);
    let mut handles;
    
    //processed_data.iter().for_each(|num| print!("{num} "));
    for ii in 0..75 {
        let mut new_data = Vec::new();

        // Split the vector into chunks
        let chunk_size = (processed_data.len() + 29) / 30;
        let chunks: Vec<Vec<u32>> = processed_data.chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        handles = Vec::with_capacity(chunks.len());
        for chunk in chunks.into_iter() {
            //let chunk_data = chunk.to_vec();
            let handle = thread::spawn(move || {
                apply_blink(&chunk)
            });
            handles.push(handle);
        }
        for handle in handles {
            match handle.join() {
                Ok(chunk_ret) => new_data.extend(chunk_ret),
                Err(e) => println!("error with line {}: {:?}", ii + 1, e),
            }
        }
        processed_data.clear();
        processed_data = new_data;
        //new_data.append(apply_blink(&processed_data));

        //println!("");
        //processed_data.iter().for_each(|num| print!("{num} "));
        println!("answer line {} = {}", ii + 1, processed_data.len());
    }
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
fn print_data(data: &Vec<Vec<char>>) {
    data.iter().for_each(|line| {
        line.iter().for_each(|letter| {
            print!("{letter}");
        });
        println!("");
    });
}

#[allow(dead_code, unused_assignments)]
fn process_data(data: &Vec<char>) -> Vec<u32>{
    let mut processed_data = Vec::new();
    let mut num = 0;
    for letter in data {
        if letter.is_numeric() {
            num = num * 10 + letter.to_digit(10).unwrap() as u32;
        } else {
            processed_data.push(num);
            num = 0;
        }
    }
    processed_data.push(num);
    return processed_data;
}

#[allow(dead_code, unused_assignments)]
fn apply_blink(line: &Vec<u32>) -> Vec<u32> {
    let mut new_line = Vec::new();
    for &ii in line {
        if ii == 0 {
            new_line.push(1);
        } else if let Some((left, right)) = has_even_digits2(ii) {
            //println!("\nii: {}, l: {}, r: {}", ii, left, right);
            new_line.push(right);
            new_line.push(left);
        } else {
            new_line.push(ii * 2024);
        }
    }
    return new_line;
}

fn has_even_digits2(num: u32) -> Option<(u32, u32)> {
    let mut temp_num = num;
    let mut count = 0;
    while temp_num > 0 {
        temp_num /= 10;
        count += 1;
    }
    if count % 2 == 1 || count == 0 {
        return None;
    }
    return Some((
        num / u32::pow(10, count / 2), 
        num % u32::pow(10, count / 2)
    ));
}

#[allow(dead_code, unused_assignments)]
fn has_even_digits(num: u32) -> Option<(u32, u32)> {
    let chrs: Vec<char> = num.to_string().chars().collect();
    // println!("");
    // chrs.iter().for_each(|c| print!("{c} "));
    // println!("");
    if chrs.len() % 2 == 0 {
        let mut ret: (u32, u32) = (0, 0);
        for ii in 0..chrs.len() {
            // // I made something that splits it up l r l r l r ... for the num
            // let r_jj = chrs.len() - 1 - ii;
            // //println!("{} ", chrs[r_jj].to_digit(10).unwrap());
            // if ii % 2 == 0 {
            //     ret.1 = ret.1 * 10 + chrs[r_jj].to_digit(10).unwrap();
            // } else {
            //     ret.0 = ret.0 * 10 + chrs[r_jj].to_digit(10).unwrap();
            // }

            if ii > (chrs.len() - 1) / 2 {
                ret.1 = ret.1 * 10 + chrs[ii].to_digit(10).unwrap() as u32;
            } else {
                ret.0 = ret.0 * 10 + chrs[ii].to_digit(10).unwrap() as u32;
            }
        }
        return Some(ret);
    }
    return None;
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hed2() {
        let testnum = 121212;
        let res = has_even_digits2(testnum).unwrap_or((666, 666));
        println!("{} hed -> {}, {}", testnum, res.0, res.1);
        assert_eq!(has_even_digits(testnum).unwrap_or((666, 666)), res);
    }
}
//end