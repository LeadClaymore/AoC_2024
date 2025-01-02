use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path;

#[allow(dead_code, unused_assignments)]
pub fn nineteen() -> io::Result<()> {
    let (threads, patterns) = match read_data(String::from("data/19/test.txt"), true) {
        Ok(stuff) => {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };

    let answers = solve_patterns(&patterns, &threads);
    println!("Answer: {}", answers.iter().filter(|idk| idk.is_some()).count());
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
fn solve_patterns(patterns: &Vec<Vec<char>>, threads: &Vec<Vec<char>>) -> Vec<Option<Vec<(usize, usize)>>> {
    let mut ret = Vec::new();
    for pp in 0..patterns.len() {
        let mut options = Vec::new();
        for tt in 0..threads.len() {
            options.push(contains(&patterns[pp], &threads[tt], false));
        }
        //paths is a list of (where the thread goes, how long the thread is)
        let mut paths = Vec::new();
        // ii is the index of options (aka possible threads to use)
        for ii in 0..options.len() {
            // list is an option that can be used (aka not none)
            if let Some(list) = &options[ii] {
                // jj is inx of list (aka where the useable thread would start in the inx of pattern)
                for jj in 0..list.len() {
                    // we only want the threads that work at 0 to start it
                    if list[jj] == 0 {
                        paths.push(vec![(ii, threads[ii].len())]);
                    }
                }
            }
        }

        let mut found = false;
        // while there are valid paths to take
        // initial condition is the options we went through earlier
        while paths.len() > 0 {
            //pp is the index of the path
            for pp in 0..paths.len() {
                //this is the last
                if paths[pp].len() > 0 {
                    let target_inx = paths[pp][paths[pp].len() - 1];
                    // now we are looking for another thread to fill the void
                    for ii in 0..options.len() {
                        // list is an option that can be used (aka not none)
                        if let Some(list) = &options[ii] {
                            // jj is inx of list (aka where the useable thread would start in the inx of pattern)
                            for jj in 0..list.len() {
                                // we only want the threads at the next open space (target_inx.1 + target_inx.0)
                                if list[jj] == (target_inx.1 + target_inx.0) {
                                    paths[pp].push((ii, threads[ii].len()));
                                    if (ii + threads[ii].len()) == patterns[ii].len() {
                                        ret.push(Some(paths[pp].clone()));
                                        found = true;
                                    }
                                }
                                if found { break; }
                            }
                        }
                        if found { break; }
                    }
                }
                if found { break; }
            }
            if found { break; }
        }
        if !found {
            ret.push(None);
        }
    }
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