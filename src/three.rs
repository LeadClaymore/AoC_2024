use std::fs::File;
use std::io::{self, BufRead};

pub fn _three_p1() -> io::Result<()> {
    let mut data: Vec<char> = Vec::new();
    let mut answer = 0;

    let reader = io::BufReader::new(File::open("data/three.txt")?);

    for line_res in reader.lines() {
        let line = line_res?;
        
        if line.trim().is_empty() {
            continue;
        } else {
            for ii in line.chars() {
                data.push(ii);
            }
        }
    }
    answer = find_mul(&data);

    println!("answer = {}", answer);
    Ok(())
}

fn find_mul(data: &Vec<char>) -> i32 {
    let mut ret: i32 = 0;
    let len = data.len();
    let mut index = 0;
    let mut num_size_1 = 0;
    let mut num_size_2 = 0;
    loop {
        (num_size_1, num_size_2) = (0, 0);

        //print!("{}", data[index]);

        // 8 is the min size of multipliable numbers
        if len - index > 8 
        {
            if
                data[index] == 'm' &&
                data[index + 1] == 'u' &&
                data[index + 2] == 'l' &&
                data[index + 3] == '(' 
            {
                while data[index + 4 + num_size_1].is_numeric() {
                    num_size_1 += 1;
                    if 
                        // 7 cause num size should inculde at least 1 that was the min of the command size
                        len - index > 7 + num_size_1 &&
                        data[index + 4 + num_size_1].is_numeric() 
                    {
                        // if the current one is numeric then continue
                        continue;
                    } else {
                        // if its not then its the last digit
                        break;
                    }
                }
                if 
                    num_size_1 != 0 &&
                    data[index + 4 + num_size_1] == ',' 
                {
                    while data[index + 5 + num_size_1 + num_size_2].is_numeric() {
                        num_size_2 += 1;
                        if 
                            len - index > 6 + num_size_1 + num_size_2 &&
                            data[index + 5 + num_size_1 + num_size_2].is_numeric() 
                        {
                            // if the current one is numeric then continue
                            continue;
                        } else {
                            // if its not then its the last digit
                            break;
                        }
                    }
                    if
                        num_size_2 != 0 &&
                        data[index + 5 + num_size_1 + num_size_2] == ')' 
                    {
                        // turns num 1 and num 2 into real numbers
                        let (mut num1, mut num2) = (0, 0);
                        for ii in (index + 4)..(index + 4 + num_size_1){
                            num1 = num1 * 10 + data[ii].to_digit(10).unwrap();
                        }
                        for jj in (index + 5 + num_size_1)..(index + 5 + num_size_1 + num_size_2){
                            num2 = num2 * 10 + data[jj].to_digit(10).unwrap();
                        }
                        ret += (num1 * num2) as i32;
                    }
                }  
            }
        }
        else {
            break;
        }
        index += 1;
    }
    return ret;
}

