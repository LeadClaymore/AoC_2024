// #[macro_use]
// extern crate ndarray;
// extern crate ndarray_linalg;

use ndarray::prelude::*;
//use ndarray_linalg::Solve;
//use nalgebra::{Matrix2, Vector2};

use std::fs::File;
use std::io::{self, BufRead};
//use serde_scan;

#[allow(dead_code, unused_assignments)]
pub fn thirteen() -> io::Result<()> {
    let mut data = Vec::new();

    match read_data_2(String::from("data/13/data.txt")) {
        Ok(ret) => data = ret,
        Err(ret) => return Err(ret),
    }

    let mut tokens = 0;
    for (a, b, c) in data {
        //println!("a: {}\nb: {}\nt: {}", a, b, c);
        //println!("[{}, {}] + [{}, {}] = [{}, {}]: ", a[[0, 0]], a[[1, 0]], b[[0, 0]], b[[1, 0]], c[[0, 0]], c[[1, 0]]);
        let low = find_lowest(&a, &b, &c);
        println!("({}, {})", low.0, low.1);
        if low != (i128::MAX, i128::MAX) && low.0 >= 0 && low.1 >= 0 {
            tokens += 3 * low.0 + low.1;
        }
    }
    println!("total tokens = {tokens}");
    //println!("answer = {}", );
    Ok(())
}

#[allow(dead_code, unused_assignments, unused_mut)]
fn read_data(file: String, data: &mut Vec<Vec<char>>) -> io::Result<()> {
    let reader = io::BufReader::new(File::open(file)?);

    for line_res in reader.lines() {
        let line = line_res?;
        let mut c_line = Vec::new();

        if line.trim().is_empty() {
            continue;
        }
        
        data.push(c_line);
    }
    Ok(())
}

#[allow(dead_code, unused_assignments)]
fn read_data_2(file: String) -> io::Result<Vec<(Array2<i128>, Array2<i128>, Array2<i128>)>> {
    let mut ret = Vec::new();
    let reader = io::BufReader::new(File::open(file)?);
    let offset = 10000000000000;

    let mut inx = 0;
    let shape = (2, 1);
    let (mut a, mut b, mut c) = (Array2::zeros(shape), Array2::zeros(shape), Array2::zeros(shape));
    for line_res in reader.lines() {
        let line = line_res?;

        //println!("{}",line);
        if line.trim().is_empty() {
            ret.push((a.clone(), b.clone(), c.clone()));
            (a, b, c) = (Array2::zeros(shape), Array2::zeros(shape), Array2::zeros(shape));
            inx = 0;
        } else {
            let mut v: [i128; 2] = [0, 0];
            let mut f_num_pushed = false;
            let mut t_num: i128 = 0;
            for ii in line.chars() {
                // all this does is look for numbers, 
                // if its a number, add it to a total, 
                // if its not a number push the current total into v[0],
                // if we have already pushed a number and its not a number push it into v[1],
                // basicly it takes any string of chars, pulls 2 numbers out of it and puts them in v

                if let Some(num) = ii.to_digit(10) {
                    t_num = t_num * 10 + num as i128;
                    //println!("was dig {num}");
                } else {
                    if t_num != 0 && !f_num_pushed {
                        v[0] = t_num as i128;
                        f_num_pushed = true;
                        t_num = 0;
                    }
                }
            }
            v[1] = t_num as i128;

            //println!("[{}][{}]", v[0], v[1]);
            if inx == 0 {
                a = Array2::from_shape_vec(shape, v.to_vec()).unwrap();
            } else if inx == 1 {
                b = Array2::from_shape_vec(shape, v.to_vec()).unwrap();
            } else if inx == 2 {
                v[0] += offset;
                v[1] += offset;
                c = Array2::from_shape_vec(shape, v.to_vec()).unwrap();
            }
            inx += 1;
        }
    }
    return Ok(ret);
}

// I believe my problem is one that the last iteration would have fixed, 
// where it does not take into account that a presses are more expensive then b presses, 
// and my current linear algebrea does not take that into account
#[allow(dead_code, unused_assignments, unused_variables)]
fn find_lowest(a: &Array2<i128>, b: &Array2<i128>, c: &Array2<i128>) -> (i128, i128) {
    let ao = 3;
    let bo = 1;
    let co: i128 = 0; //10000000000000;
    let det = a[[0, 0]] * b[[1, 0]] - a[[1, 0]] * b[[0, 0]];
    if det != 0 {
        let ret = (
            ((co + c[[0, 0]]) * b[[1, 0]] - (co + c[[1, 0]]) * b[[0, 0]]) / det,
            (0 - (co + c[[0, 0]]) * a[[1, 0]] + (co + c[[1, 0]]) * a[[0, 0]]) / det
        );
        if (co + c[[0, 0]]) == b[[0, 0]] * ret.1 + a[[0, 0]] * ret.0 {
            return ret;
        }
    }
    return (i128::MAX, i128::MAX);

    // // find the minimum ittetations of the lowest number to reach the total
    // let mi = if a[[0, 0]] > b[[0, 0]] {
    //     (c_offset + c[[0, 0]]) / b[[0, 0]] + 1
    // } else {
    //     (c_offset + c[[0, 0]]) / a[[0, 0]] + 1
    // };
    // let mj = if a[[1, 0]] > b[[1, 0]] {
    //     (c_offset + c[[1, 0]]) / b[[1, 0]]
    // } else {
    //     (c_offset + c[[1, 0]]) / a[[1, 0]]
    // };

    // //print!("{mi} {mj}: ");
    // let mut ret = (i128::MAX, i128::MAX);
    // for ii in 0..=mi {
    //     for jj in 0..=mj {
    //         if a * ii + b * jj == c {
    //             //print!("[{ii}][{jj}], ");
    //             if ret.0 == i128::MAX || a_but * ii + b_but * jj < ret.0 + ret.1 {
    //                 ret.0 = ii;
    //                 ret.1 = jj;
    //             }
    //         }
    //     }
    // }

    // return ret;
}

// #[allow(dead_code, unused_assignments, unused_variables)]
// fn find_lowest_2(a: &Array2<i128>, b: &Array2<i128>, c: &Array2<i128>) -> (i128, i128) {
//     // let aa: nalgebra::Matrix<i128, nalgebra::Const<2>, nalgebra::Const<2>, nalgebra::ArrayStorage<i128, 2, 2>> = Matrix2::new(a[[0, 0]], b[[0, 0]], a[[1, 0]], b[[1, 0]]);
//     // let bb = Vector2::new(c[[0, 0]], c[[1, 0]]);

//     // match a.try_inverse() {
//     //     Some(inv_a) => {
//     //         let solution = inv_a * b;
//     //         println!("Unique solution: {:?}", solution);
//     //     },
//     //     None => {
//     //         return (0, 0);
//     //     }
//     // }


//     // let aa: Array2<i128> = array![[a[[0, 0]], b[[0, 0]]],[a[[1, 0]], b[[1, 0]]]];
//     // let vv: Array2<i128> = array![c[[0, 0]], c[[1, 0]]];
//     // solve is meant to work
//     // I even found the docs.rs on it
//     // https://docs.rs/ndarray-linalg/latest/ndarray_linalg/solve/
//     // let solution = aa.solve_into(vv).unwrap();
//     return (0,0);
// }
//end