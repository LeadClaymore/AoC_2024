use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn nineteen() -> io::Result<()> {
    let (threads, patterns) = match read_data(String::from("C:/Users/Clayton Ross/Desktop/Rust/AoC_2024/data/19/data.txt"), true) {
        Ok(stuff) => {
            println!("Data read");
            stuff
        },
        Err(ret) => return Err(ret),
    };

    let mut answers = 0;
    for ii in 0..patterns.len() {
        if let Some(_recreation) = solve_patterns(&patterns[ii], &threads, false) {
            answers += 1;
        }
    }
    println!("Answer: {}", answers);
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

///returns a vector for each pattern in each optionaly a vector of indexes of (start_inx, size, threads_inx) if it exists
#[allow(dead_code, unused_assignments)]
fn solve_patterns(pattern: &Vec<char>, threads: &Vec<Vec<char>>, debug: bool) -> Option<Vec<(usize, usize, usize)>> {
    let mut ret = None;
    if debug { 
        print!("trying pattern: ");
        pattern.iter().for_each(|cc| print!("{cc}") );
        println!("");
    }
    let mut options = Vec::new();
    for tt in 0..threads.len() {
        options.push(contains(&pattern, &threads[tt], false));
    }
    if debug {
        for oo in 0..options.len() {
            print!("\t");
            threads[oo].iter().for_each(|cc| print!("{cc}") );
            print!(": ");
            if let Some(opt) = &options[oo] {
                opt.iter().for_each(|nn| print!("{nn}, ") );
                println!("");
            } else {
                println!("None");
            }
        }
        println!("finding threads");
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
                    paths.push(vec![(0, threads[ii].len(), ii)]);
                }
            }
        }
    }


    let mut found = false;
    // while there are valid paths to take
    // initial condition is the options we went through earlier
    while paths.len() > 0 {
        if debug { println!("currently {} paths", paths.len()) }
        let mut next_path = Vec::new();
        //pp is the index of the path
        for yy in 0..paths.len() {
            if debug { 
                print!("\ttrying path {yy}: ");
                paths[yy].iter().for_each(|(p1, p2, p3)| {
                    print!("({p1}, {p2}, ");
                    threads[*p3].iter().for_each(|cc| print!("{cc}"));
                    print!(") ");
                });
                println!("");
            }
            //this is the last
            if paths[yy].len() > 0 {
                let target_inx = paths[yy][paths[yy].len() - 1];
                if debug { println!("\t\ttarget_inx ({}, {})", target_inx.0, target_inx.1) }
                // now we are looking for another thread to fill the void
                for ii in 0..options.len() {
                    // list is an option that can be used (aka not none)
                    if let Some(list) = &options[ii] {
                        // jj is inx of list (aka where the useable thread would start in the inx of pattern)
                        for jj in 0..list.len() {
                            // if debug { 
                            //     print!("\t\t\ttrying {} aka ", list[jj]);
                            //     threads[list[jj]].iter().for_each(|cc| print!("{cc}"));
                            //     println!("");
                            // }
                            // we only want the threads at the next open space (target_inx.1 + target_inx.0)
                            if list[jj] == (target_inx.1 + target_inx.0) {
                                if debug { print!("\t\t\t\tTarget found!, p_len: {}, cur_len: {}\n", pattern.len(), list[jj] + threads[ii].len());}
                                let mut next = paths[yy].clone();
                                next.push((list[jj], threads[ii].len(), ii));
                                if (list[jj] + threads[ii].len()) == pattern.len() {
                                    if debug { print!("\t\t\t\tEnd found!, \n");}
                                    ret = Some(next);
                                    found = true;
                                    break;
                                } else {
                                    next_path.push(next);
                                }
                            }
                        }
                    }
                    if found { break; }
                }
            }
            if found { break; }
        }
        if found { break; } else {
            paths.clear();
            paths = next_path;
        }
    }
    
    print!("Pattern: ");
    pattern.iter().for_each(|cc| print!("{cc}"));
    if !found {
        println!(" Not Found");
        return None;
    } else {
        println!(" Was Found!");
        return ret;
    }
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