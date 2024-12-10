use std::fs::File;
//use std::intrinsics::mir::BasicBlock;
use std::io::{self, BufRead};



#[allow(dead_code, unused_assignments)]
pub fn nine() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;

    match read_data(String::from("data/nine_test.txt"), &mut data) {
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
    //compress_blocks(&mut blocks);
    compress_blocks_two(&mut blocks);
    
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

fn compress_blocks_two(blocks: &mut Vec<Option<u32>>) {
    let mut f_block = find_last_full_block(&blocks);
    let mut e_block = None;
    if f_block.is_some() {
        e_block = find_empty_space(&blocks, f_block.unwrap().1);
    }

    while 
        e_block.is_some() && f_block.is_some() &&
        e_block.unwrap() < f_block.unwrap().0
    {
        print_blocks(blocks);
        //todo r swap
        recursive_swap(blocks, e_block.unwrap(), e_block.unwrap() + f_block.unwrap().1 - 1, f_block.unwrap().0);
        f_block = find_last_full_block(&blocks);
        if f_block.is_some() {
            e_block = find_empty_space(&blocks, f_block.unwrap().1);
        }
    }

    find_answer(blocks);
}

fn find_empty_space(blocks: &Vec<Option<u32>>, size: usize) -> Option<usize> {
    // this gets an iterator of the indexs of blocks that are none
    // enumerate seperates it into an iterator of a collection of (index, value)
    // filter in this case filters for if the value is none
    // map takes those filtered (index, value) of only none values, and changes it only to the indexes

    // aka we went from an iterator of an collection of Option then
    // changed it to an iterator of a collection of (index, Option) where the index is where the Option was in the previous collection
    // then we filtered for Nones in the options (aka removing all the options wtih some in them)
    // then we reduced the (index, option) to just and index in the same order that the (index, option) collection had
    // now we have a iterator of usize that is the indexes of Nones that were in the original collection (also in the same order)
    for ii in blocks.iter()
        .enumerate()
        .filter(|(_, &v)| v.is_none())
        .map(|(i, _)| i) 
    {
        // ii here is indexs of none within blocks

        // iterator magic of skipping to the index we found before, 
        // taking a slice of the size we need to check for being empty
        // filtering by what has some in it (aka what we dont want)
        // then getting the length of that collection of some
        // and if its 0 (aka there is no some) then this is the index we want to return
        if blocks.iter().skip(ii).take(size).filter(|&jj| jj.is_some()).count() == 0 {
            return Some(ii)
        }
    }
    
    // if we cant find a spot return none, but tbh there should allways be something, if the data is entered properly
    return None;
}

/// returns (last index, how many index)
fn find_last_full_block(blocks: &Vec<Option<u32>>) -> Option<(usize, usize)> {
    let mut num_to_find = None;
    let mut counter = 0;
    let mut last_inx = None;
    for ii in blocks.iter().rev()
        .enumerate().filter(|(_inx, &val)| val.is_some())
        .map(|(inx,  _val)| inx)
    {
        let rev_inx = blocks.len() - 1 - ii;
        if num_to_find.is_none() {
            num_to_find = Some(blocks[ii]);
            last_inx = Some(rev_inx);
            counter += 1;
            continue;
        } else {
            if num_to_find.unwrap() == blocks[ii] && rev_inx.abs_diff(last_inx.unwrap()) <= 1 {
                counter += 1;
                last_inx = Some(rev_inx);
                continue;
            }
        }
        break;
    }

    if last_inx != None {
        return Some((last_inx.unwrap(), counter));
    } else {
        return None;
    }
}

fn recursive_swap(blocks: &mut Vec<Option<u32>>, s_inx: usize, e_inx: usize, to_inx: usize) {
    let dif = e_inx - s_inx;
    if 
        e_inx < blocks.len() &&
        to_inx + dif < blocks.len()
    {
        for ii in 0..=dif {
            blocks.swap(s_inx + ii, to_inx + ii);
        }
    } else {
        println!("error trying to swap s: {s_inx}, e: {e_inx}, to: {to_inx}")
    }
}


fn print_blocks(blocks: &Vec<Option<u32>>) {
    blocks.iter().for_each(|c| {
        if c.is_some() {
            print!("{}", c.unwrap());
        } else {
            print!(".");
        }
    });
    println!("");
}
//end