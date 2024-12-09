use std::fs::File;
use std::io::{self, BufRead};



#[allow(dead_code, unused_assignments)]
pub fn nine() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;

    match read_data(String::from("data/nine.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    //print_data(&data);
    // gets how many non numbers are in the data
    if data.iter().map(|line|
        line.iter().filter(|&letter| letter.is_numeric()).count()
    ).sum::<usize>() == 0 {
        println!("Non numbers parsed");
        return Ok(());
    }
    if data.len() != 1 {
        println!("Unexpeded line size (expected 1)")
    }

    let new_data: Vec<u32> = data[0].iter().filter_map(|c| c.to_digit(10)).collect();
    let mut blocks = Vec::new();

    build_blocks(&new_data, &mut blocks);
    compress_blocks(&mut blocks);
    
    Ok(())
}

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

fn print_data(data: &Vec<Vec<char>>) {
    data.iter().for_each(|line| {
        line.iter().for_each(|letter| {
            print!("{letter}");
        });
        println!("");
    });
}

fn build_blocks(data: &Vec<u32>, blocks: &mut Vec<Option<u32>>) {
    blocks.clear();
    let mut counter: u32 = 0;
    let mut free_space = false;
    for ii in 0..data.len() {
        for _ in 0..data[ii] {
            if free_space {
                blocks.push(None);
            } else {
                blocks.push(Some(counter));
            }
        }
        if free_space {
            free_space = !free_space;
        } else {
            free_space = !free_space;
            counter += 1;
            // I was wrong
            // // I think the answer needs this to stay within 1 dig of base 10
            // if counter == 10 {
            //     counter = 0;
            // }
        }
    }
    // println!("");
    // blocks.iter().for_each(|c| print!("{c}"));
    // println!("");
}

fn compress_blocks(blocks: &mut Vec<Option<u32>>) {
    let mut dd = blocks.iter().position(|&c| c.is_none()).unwrap();
    let mut nn = blocks.len() - 1 - blocks.iter().rev().position(|&c| c.is_some()).unwrap();

    while nn > dd {
        blocks.swap(dd, nn);
        dd = blocks.iter().position(|&c| c.is_none()).unwrap();
        nn = blocks.len() - 1 - blocks.iter().rev().position(|&c| c.is_some()).unwrap();
    }

    // println!("");
    // blocks.iter().for_each(|c| print!("{c}"));
    // println!("");

    find_answer(&blocks);
}

fn find_answer(blocks: &Vec<Option<u32>>) {
    // dnw forgot that I need to mult by a number, I could do it with a move operation but ill learn to do that later
    // println!("{}", blocks.iter().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).sum::<u32>());
    let (mut sum, mut inx) = (0, 0);
    for c in blocks.iter().filter(|&c| c.is_some()).map(|c| c.unwrap()) {
        sum += c as u128 * inx as u128;
        inx += 1;
        // if inx == 10 {
        //     inx = 0;
        // }
    }
    println!("answer = {sum}");
}


//end