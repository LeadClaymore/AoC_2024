use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn four_p1() -> io::Result<()> {
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
                //print!("{}", ii);
                c_line.push(ii);
            }
            //println!("");
            data.push(c_line);
        }
    }

    // bounds of the cross word puzzle
    let (x_bound, y_bound) = (data[0].len() as i32, data.len() as i32);

    // I wanted the word to be able to change easily in case of the p2 
    // and also it makes the code look better to me
    let og_search_word = String::from("XMAS");
    let mut search_word = Vec::new();
    for letter in og_search_word.chars() {
        search_word.push(letter);
    }
    //length of the word
    let sw_len = search_word.len() as i32;

    // this will hold the dirrection on the grid / graph that the cursor will check, could have been cleaner
    let mut dirs: Vec<(i32, i32)> = Vec::new();
    dirs.push((-1, -1));    // down left
    dirs.push((-1, 0));     // left
    dirs.push((-1, 1));     // up left
    dirs.push((0, -1));     // down

    // only need half the directions if we check reverse (we need to anyways)
    //dirs.push((0, 1));      // up
    //dirs.push((1, -1));     // down right
    //dirs.push((1, 0));      // right
    //dirs.push((1, 1));      // up right
    
    //dirs.push((0, 0));    // dne

    // search
    for ii in 0..data.len() {
        for jj in 0..data[ii].len() {
            //print!("{}", data[ii][jj]);
            
            // for each cordinate dirrection
            for &dir in &dirs {
                // if the dirrections dont fall off the graph / grid
                if 
                    ii as i32 + dir.0 * (sw_len - 1) >= 0 &&
                    ii as i32 + dir.0 * (sw_len - 1) < y_bound &&
                    jj as i32 + dir.1 * (sw_len - 1) >= 0 &&
                    jj as i32 + dir.1 * (sw_len - 1) < x_bound
                {
                    // for the length of the word (forward)
                    for inx in 0..sw_len {
                        // if the letter = the search word letter
                        if 
                            search_word[inx as usize] == 
                            data[(ii as i32 + dir.0 * inx) as usize]
                                [(jj as i32 + dir.1 * inx) as usize] 
                        {
                            // got to the end of the word
                            if inx == sw_len - 1 {
                                //println!("({},{}). dir: ({},{})", ii, jj, dir.0, dir.1);
                                answer += 1;
                            }
                            continue;
                        } else {
                            break;
                        }
                    }

                    // for the length of the word (backwards)
                    for inx in 0..sw_len {
                        let rev_inx = sw_len - inx - 1;
                        // if the letter = the search word letter
                        if 
                            search_word[rev_inx as usize] == 
                            data[(ii as i32 + dir.0 * inx) as usize]
                                [(jj as i32 + dir.1 * inx) as usize] 
                        {
                            // got to the end of the word
                            if inx == sw_len - 1 {
                                //println!("({},{}). dir: ({},{})", ii, jj, dir.0, dir.1);
                                answer += 1;
                            }
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        //println!("");
    }

    println!("answer = {}", answer);
    Ok(())
}

#[allow(dead_code, unused_assignments)]
pub fn four_p2() -> io::Result<()> {
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
                //print!("{}", ii);
                c_line.push(ii);
            }
            //println!("");
            data.push(c_line);
        }
    }

    // bounds of the cross word puzzle
    let (x_bound, y_bound) = (data[0].len() as i32, data.len() as i32);

    // I wanted the word to be able to change easily in case of the p2 
    // and also it makes the code look better to me
    let og_search_word = String::from("MAS");
    let mut search_word = Vec::new();
    for letter in og_search_word.chars() {
        search_word.push(letter);
    }
    //length of the word
    //let sw_len = search_word.len() as i32;

    // this will hold the dirrection on the grid / graph that the cursor will check, could have been cleaner
    let mut dirs: Vec<(i32, i32)> = Vec::new();
    dirs.push((-1, 0));    // down
    dirs.push((1, 0));     // up
    dirs.push((0, 1));     // right
    dirs.push((0, -1));    // left

    // search
    for ii in 0..data.len() {
        for jj in 0..data[ii].len() {
            //print!("{}", data[ii][jj]);
            
            // if the dirrections dont fall off the graph / grid
            if 
                ii > 0 && ii + 1 < y_bound as usize &&
                jj > 0 && jj + 1 < x_bound as usize
            {
                if 
                    data[ii - 1][jj - 1] == search_word[0] &&
                    data[ii - 1][jj + 1] == search_word[0] &&
                    data[ii][jj] == search_word[1] &&
                    data[ii + 1][jj - 1] == search_word[2] &&
                    data[ii + 1][jj + 1] == search_word[2]
                {
                    answer += 1;
                } else if 
                    data[ii + 1][jj + 1] == search_word[0] &&
                    data[ii + 1][jj - 1] == search_word[0] &&
                    data[ii][jj] == search_word[1] &&
                    data[ii - 1][jj + 1] == search_word[2] &&
                    data[ii - 1][jj - 1] == search_word[2]
                {
                    answer += 1;
                } else if 
                    data[ii - 1][jj + 1] == search_word[0] &&
                    data[ii + 1][jj + 1] == search_word[0] &&
                    data[ii][jj] == search_word[1] &&
                    data[ii - 1][jj - 1] == search_word[2] &&
                    data[ii + 1][jj - 1] == search_word[2]
                {
                    answer += 1;
                } else if 
                    data[ii - 1][jj + 1] == search_word[2] &&
                    data[ii + 1][jj + 1] == search_word[2] &&
                    data[ii][jj] == search_word[1] &&
                    data[ii - 1][jj - 1] == search_word[0] &&
                    data[ii + 1][jj - 1] == search_word[0]
                {
                    answer += 1;
                }
            }
        }
        //println!("");
    }

    println!("answer = {}", answer);
    Ok(())
}