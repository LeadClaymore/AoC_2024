use std::collections::HashMap;
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
    iteration_two(&processed_data);
    
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
fn iteration_two(data: &Vec<u128>) {
    let mut hmap = HashMap::new();
    for &num in data {
        *hmap.entry(num).or_insert(0 as usize) += 1;
    }

    for blk in 1..=75 {
        hmap = apply_blink_v2(&hmap);
        let c_nums = hmap.iter().map(|(i, j)| j).sum::<usize>();
        println!("answer line {} = int: {}, c_nums: {}", blk, hmap.len(), c_nums);
    }
}

#[allow(dead_code, unused_assignments)]
fn apply_blink_v2(hmap: &HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut new_hmap = HashMap::new();
    for (&num, &count) in hmap {
        if num == 0 {
            *new_hmap.entry(1).or_insert(0 as usize) += count;
        } else if let Some((left, right)) = has_even_digits2(num) {
            *new_hmap.entry(left).or_insert(0 as usize) += count;
            *new_hmap.entry(right).or_insert(0 as usize) += count;
        } else {
            *new_hmap.entry(num * 2024).or_insert(0 as usize) += count;
        }
    }
    return new_hmap;
}

#[allow(dead_code, unused_assignments)]
fn iteration_one(data: &mut Vec<u128>) {
    let mut handles;
    //processed_data.iter().for_each(|num| print!("{num} "));
    for ii in 0..75 {
        let mut new_data = Vec::new();

        // Split the vector into chunks
        let chunk_size = (data.len() + 29) / 30;
        let chunks: Vec<Vec<u128>> = data.chunks(chunk_size)
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
        data.clear();
        *data = new_data;
        //new_data.append(apply_blink(&processed_data));

        //println!("");
        //processed_data.iter().for_each(|num| print!("{num} "));
        println!("answer line {} = {}", ii + 1, data.len());
    }
}

#[allow(dead_code, unused_assignments)]
fn process_data(data: &Vec<char>) -> Vec<u128>{
    let mut processed_data = Vec::new();
    let mut num = 0;
    for letter in data {
        if letter.is_numeric() {
            num = num * 10 + letter.to_digit(10).unwrap() as u128;
        } else {
            processed_data.push(num);
            num = 0;
        }
    }
    processed_data.push(num);
    return processed_data;
}

#[allow(dead_code, unused_assignments)]
fn apply_blink(line: &Vec<u128>) -> Vec<u128> {
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

fn has_even_digits2(num: u128) -> Option<(u128, u128)> {
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
        num / u128::pow(10, count / 2), 
        num % u128::pow(10, count / 2)
    ));
}

#[allow(dead_code, unused_assignments)]
fn has_even_digits(num: u128) -> Option<(u128, u128)> {
    let chrs: Vec<char> = num.to_string().chars().collect();
    // println!("");
    // chrs.iter().for_each(|c| print!("{c} "));
    // println!("");
    if chrs.len() % 2 == 0 {
        let mut ret: (u128, u128) = (0, 0);
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
                ret.1 = ret.1 * 10 + chrs[ii].to_digit(10).unwrap() as u128;
            } else {
                ret.0 = ret.0 * 10 + chrs[ii].to_digit(10).unwrap() as u128;
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