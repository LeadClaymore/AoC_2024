use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn nineteen() -> io::Result<()> {
    let (threads, patterns) = match read_data(String::from("data/19/test.txt"), true) {
        Ok(stuff) => {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };
    Ok(())
}

/// returns patterns and threads
#[allow(dead_code, unused_assignments)]
fn read_data(file: String, debug: bool) -> io::Result<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    let mut ret_t: Vec<Vec<char>> = Vec::new();
    let mut ret_p: Vec<Vec<char>> = Vec::new();
    let mut s_or_t = true;

    let reader = io::BufReader::new(File::open(file)?);
    for line_res in reader.lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            s_or_t = false;
            continue;
        } else {
            if s_or_t {
                let mut line_c = Vec::new();

                for cc in line.chars() {
                    match cc {
                        ' ' => continue,
                        ',' => {
                            ret_t.push(line_c);
                            line_c = Vec::new();
                        },
                        _ => line_c.push(cc),
                    }
                }
                ret_t.push(line_c);
            } else {
                ret_p.push(line.chars().into_iter().collect());
            }
        }
    }
    if debug {
        for ii in 0..ret_t.len() {
            ret_t[ii].iter().for_each(|cc| print!("{}", cc));
            print!(", ");
        }
        println!("\n");
        for ii in 0..ret_p.len() {
            ret_p[ii].iter().for_each(|cc| print!("{}", cc));
            println!("");
        }
    }
    return Ok((ret_t, ret_p));
}

///returns a vector for each pattern in each is a vector of indexes of what threads make the pattern
#[allow(dead_code, unused_assignments)]
fn solve_patterns(patterns: &Vec<Vec<char>>, threads: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for pp in 0..patterns.len() {
        let line = Vec::new();
        
        ret.push(line);
    }
    return ret;
}

//the way this is going to work is that for each word we 
#[allow(dead_code, unused_assignments)]
fn compose_thread(pattern: &Vec<char>, threads: &Vec<Vec<char>>) -> Vec<usize> {
    let ret = Vec::new();
    // for ii in 0..pattern.len() {
    //     for jj in 0..threads.len() {
    //         for kk in 0..threads[ii].len() {
    //             if pattern[ii] == 
    //         }
    //     }
    // }
    return ret;
}

///finds instances of cv2 in cv1 and returns a vec of where they start
#[allow(dead_code, unused_assignments)]
fn contains(cv1: &Vec<char>, cv2: &Vec<char>, debug: bool) -> Option<Vec<usize>> {
    if debug { println!("contains start"); }
    let mut ret = Vec::new();
    // we subtract cv2 to get where the instance of cv2 could start and the +1 is so we get every char possible
    for ii in 0..(cv1.len() - cv2.len() + 1) {
        if cv1[ii] == cv2[0] {
            if debug { print!("{}:{} ", ii, cv2[0]); }
            let mut contains = true;
            for jj in 1..cv2.len() {
                if cv1[ii + jj] != cv2[jj] {
                    contains = false;
                    break;
                }
                if debug { print!("{}:{} ", ii + jj, cv2[jj]); }
            }
            if debug { println!(""); }
            if contains {
                ret.push(ii);
            }
        }
    }
    if ret.is_empty() {
        return None;
    } else {
        return Some(ret);
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains1() {
        let word1 = "letters".chars().collect();
        //let word1  = vec!['l', 'e', 't', 't', 'e', 'r', 's'];
        let word2  = vec!['e'];
        let answer = contains(&word1, &word2, true).unwrap();
        let lhs: usize = vec![1, 4].iter().sum();
        let rhs: usize = answer.iter().sum();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_contains2() {
        let word1 = "jajajojaj".chars().collect();
        let word2 = "ja".chars().collect();
        let answer = contains(&word1, &word2, true).unwrap();
        let lhs: usize = vec![0, 2, 6].iter().sum();
        let rhs: usize = answer.iter().sum();
        assert_eq!(lhs, rhs);
    }
}
//end