use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn ten() -> io::Result<()> {
    let mut data = Vec::new();

    match read_data(String::from("data/ten/ten_test.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    //print_data(&data);
    let mut processed_data = Vec::new();
    process_data(&data, &mut processed_data);
    print_processed_data(&processed_data);
    find_trails(&processed_data);
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
fn process_data(data: &Vec<Vec<char>>, processed_data: &mut Vec<Vec<u32>>) {
    data.iter().for_each(|line| {
        let mut processed_line = Vec::new();
        line.iter().for_each(|letter| {
            processed_line.push(letter.to_digit(10).unwrap() as u32);
        });
        processed_data.push(processed_line);
    });
}

#[allow(dead_code, unused_assignments)]
fn print_processed_data(data: &Vec<Vec<u32>>) {
    data.iter().for_each(|line| {
        line.iter().for_each(|digit| {
            print!("{digit}");
        });
        println!("");
    });
}

// ok so what I need to do is create an algorithm that takes each 0 position

// and from there tries to move in the 4 dirrections 
// and if the next position is 1 higher then the last then move to that spot.
// once it reaches 9 it returns 1;
// I plan on making this a recursive function that starts with the 0 position
// then calls itself at the next position and sums the returns then returns that
// this should return a number for the sum of each trail head
// sum that and its the answer

#[allow(dead_code, unused_assignments)]
fn find_trails(processed_data: &Vec<Vec<u32>>) {
    let mut sum = 0;
    let mut sum2 = 0;
    for ii in 0..processed_data.len() {
        for jj in 0..processed_data[ii].len() {
            if processed_data[ii][jj] == 0 {
                if let Some(path) = rec_score_head(&processed_data, (ii, jj), 0) {
                    sum2 += path.len();
                    sum += count_unique_paths(&path);
                }
            }
        }
    }
    println!("answer = {}, answer2 = {}", sum, sum2);
}

//TODO currently you find each way you can reach 9 but you need to find each 9 you can reach
// so instead of returning how many 9's you reach via u32 instead return a vec of (usize, usize)
// this vector will be each 9 reached and apon returning it will append the vec with all returns and return that
// at the start of the recursion you take the vec and count the unique position
// also its going to be an option of vec of (usize, usize) bc most of them return nothing
#[allow(dead_code, unused_assignments)]
///pos is ii, jj
fn rec_score_head(processed_data: &Vec<Vec<u32>>, pos: (usize, usize), next_num: u32) -> Option<Vec<(usize, usize)>> {
    if processed_data[pos.0][pos.1] == 9 && next_num == 9 {
        return Some(vec![(pos)]);
    } else if processed_data[pos.0][pos.1] != next_num {
        return None;
    }

    let mut sum = Vec::new();
    let mut temp = None;
    // if up is valid
    if pos.0 > 0 {
        temp = rec_score_head(processed_data, (pos.0 - 1, pos.1), next_num + 1);
        if temp.is_some() {
            for pos in temp.unwrap() {
                sum.push(pos);
            }
        }
    }

    // if down is valid
    if pos.0 < processed_data.len() - 1 {
        temp = rec_score_head(processed_data, (pos.0 + 1, pos.1), next_num + 1);
        if temp.is_some() {
            for pos in temp.unwrap() {
                sum.push(pos);
            }
        }
    }

    // if left is valid
    if pos.1 > 0  {
        temp = rec_score_head(processed_data, (pos.0, pos.1 - 1), next_num + 1);
        if temp.is_some() {
            for pos in temp.unwrap() {
                sum.push(pos);
            }
        }
    }

    // if right is valid
    if pos.1 < processed_data[0].len() - 1 {
        temp = rec_score_head(processed_data, (pos.0, pos.1 + 1), next_num + 1);
        if temp.is_some() {
            for pos in temp.unwrap() {
                sum.push(pos);
            }
        }
    }

    if sum.is_empty() {
        return None;
    } else {
        return Some(sum);
    }
}

fn count_unique_paths(paths: &Vec<(usize, usize)>) -> u32 {
    let mut sum = 0;
    let mut used_paths: Vec<(usize, usize)> = Vec::new();

    for path in paths {
        let mut used = false;
        for u_path in &used_paths {
            if path.0 == u_path.0 && path.1 == u_path.1 {
                used = true;
                break;
            }
        }
        if !used {
            used_paths.push(path.clone());
            sum += 1;
        }
    }

    return sum;
}