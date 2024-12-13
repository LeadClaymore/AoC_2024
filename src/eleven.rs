use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn eleven() -> io::Result<()> {
    let mut data = Vec::new();

    match read_data(String::from("data/eleven/data.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    //print_data(&data);
    let mut processed_data = Vec::new();
    process_data(&data[0], &mut processed_data);
    //processed_data.iter().for_each(|num| print!("{num} "));
    for ii in 0..75 {
        apply_blink(&mut processed_data);
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
fn process_data(data: &Vec<char>, processed_data: &mut Vec<u64>) {
    processed_data.clear();
    let mut num = 0;
    for letter in data {
        if letter.is_numeric() {
            num = num * 10 + letter.to_digit(10).unwrap() as u64;
        } else {
            processed_data.push(num);
            num = 0;
        }
    }
    processed_data.push(num);
}

#[allow(dead_code, unused_assignments)]
fn apply_blink(line: &mut Vec<u64>) {
    //let mut new_line = Vec::new();
    let mut ii = 0;
    while ii < line.len() {
        if line[ii] == 0 {
            line[ii] = 1;
            //print!("on 1 ");
        } else if let Some((left, right)) = has_even_digits2(line[ii]) {
            //println!("\nii: {}, l: {}, r: {}", ii, left, right);
            line[ii] = right;
            line.insert(ii, left);
            ii += 1;
            // new_line.push(left);
            // new_line.push(right);
            //print!("lr {left} {right} ");
        } else {
            line[ii] = line[ii] * 2024;
            //print!("tf {} ", ii * 2024);
        }
        ii += 1;
    }
    //println!("");
}

#[allow(dead_code, unused_assignments)]
fn has_even_digits(num: u64) -> Option<(u64, u64)> {
    let chrs: Vec<char> = num.to_string().chars().collect();
    // println!("");
    // chrs.iter().for_each(|c| print!("{c} "));
    // println!("");
    if chrs.len() % 2 == 0 {
        let mut ret: (u64, u64) = (0, 0);
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
                ret.1 = ret.1 * 10 + chrs[ii].to_digit(10).unwrap() as u64;
            } else {
                ret.0 = ret.0 * 10 + chrs[ii].to_digit(10).unwrap() as u64;
            }
        }
        return Some(ret);
    }
    return None;
}

fn has_even_digits2(num: u64) -> Option<(u64, u64)> {
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
        num / u64::pow(10, count / 2), 
        num % u64::pow(10, count / 2)
    ));
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