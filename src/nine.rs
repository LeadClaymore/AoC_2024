use std::fs::File;
//use std::intrinsics::mir::BasicBlock;
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
    //compress_blocks(&mut blocks);
    compress_blocks_four(&mut blocks);
    
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
    print_blocks(blocks);
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
    for c in blocks {
        if c.is_some() {
            sum += c.unwrap() as u128 * inx as u128;
        }
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

// currenly you block moved based on the free block had allready tried fillin that,
// however I think i need to go back to finding the block and trying to squeeze it into a space
// specificaly in order, once its tried 8888 and failed it SHOULD NEVER TRY AGAIN
// DISPITE THAT BEING THE WORST WAY OF COMPRESSING STUFF IVE HEARD OF 
// WHY WOULD YOU NOT WANT TO TRY THE LARGEST ONES FIRST THEN THE SMALL ONES LIKE C_B_2
// OR IF YOU SO FANCY IGNORE HOLES LIKE THAT AND JUST TRY TO FILL EVERY HOLE
// IVE SPENT FAR TOO LONG ON THIS STUPID INTERPERTATION OF A COMPRESSION SYSTEM
// I SPENT 120 LINES ON P1 AND AT THIS POINT 230 ON THIS NEEDY CRAP

// TODO:
// so make a compress_blocks_four that uses the learned aspects of 3 and 2 that will 
// go back to finding the block and trying to squeeze it into a space 
// but this time simply try each block once rather then again
// so to line out tommorow before starting on ten p1 you need to 
// 1: find the last largest block filled aka '99' at the end
// 2: take that block and find the first empty block (left of it) that can fit it
// 2.1: if there is none, go back to 1 with the next number
// 2.2: if there is no number then ur done thats it
// 3 take the block and e_block and swap it, then go back to 1 with the next number
// use the current num's, val and index to denote
    // a: what the next val should be (aka val + 1)
    // b: where the free space should be (aka 0..ii)
// AND AFTERWARDS USE MS PAINT OR COMMENTS TO PLAN THIS CRAP OUT SO YOU CAN KNOW IF THE ALGORITHM IS SHODDY BEFORE YOU SPEND TIME WRITING IT
fn compress_blocks_four(blocks: &mut Vec<Option<u32>>) {
    let mut curr_block = None;
    let mut empty_block = None;
    let mut cur_num = blocks[blocks.len() - 1].unwrap_or(
        //this should get the last some element and it should be valid to unwrap
        blocks.iter().filter(|val| val.is_some()).last().unwrap().unwrap()
    );
    //println!("{}, {}, {}", idk.0, idk.1, blocks.len());
    loop {
        curr_block = find_curr_block(blocks, cur_num);
        if curr_block.is_some() {
            empty_block = get_empty_block(blocks, curr_block.unwrap().1);
            //println!("cbi {}, cbs {}, cn {}, eb {}", curr_block.unwrap().0, curr_block.unwrap().1, cur_num, empty_block.unwrap_or(666));
            if empty_block.is_some() && curr_block.unwrap().0 > empty_block.unwrap() {
                recursive_swap(
                    blocks, 
                    empty_block.unwrap(), 
                    empty_block.unwrap() + curr_block.unwrap().1, 
                    curr_block.unwrap().0
                );
                //print_blocks(blocks);
            }
            if cur_num > 0 {
                cur_num = cur_num - 1;
            } else {
                //good end
                break;
            }
        } else {
            break;
        }
    }
    //print_blocks(blocks);
    find_answer(blocks);
}

fn get_empty_block(blocks: &Vec<Option<u32>>, size: usize) -> Option<usize> {
    // this iterates over the empty spots in blocks
    for ii in blocks.iter().enumerate().filter(
        |(_inx, &val)| val.is_none()
        ).map(|(inx, _val)| inx) 
    {
        // if the size could fit
        if ii + size < blocks.len() {
            let mut works = true;
            //this loops over the needed size starting at ii
            for jj in ii..(ii + size) {
                //println!("{}, {}, {}", ii, jj, size);
                // if it one is not none then it fails
                // and must try the next if possible
                if blocks[jj].is_none() {
                    continue;
                }
                works = false;
                break;
            }
            // if the needed size exists then return the start of it
            if works {
                return Some(ii);
            }
        }
    }
    return None;
}

///returns a block of Option(index, size)
fn find_curr_block(blocks: &Vec<Option<u32>>, cur_num: u32) -> Option<(usize, usize)> {
    let mut ret = (0, 0);
    let itt = blocks.iter().enumerate().filter(
        |(_inx, &val)| val.is_some() && val.unwrap() == cur_num
    );
    ret.1 = itt.clone().count();
    if ret.1 > 0 {
        return Some((
            itt.map(|(inx, _val)| inx ).next().unwrap(),
            ret.1,
        ));
    }
    return None;
}

fn compress_blocks_three(blocks: &mut Vec<Option<u32>>) {
    let mut free_block = None;
    let mut block_to_fill = None;
    let mut failed = false;
    let mut counter = 0;
    let mut ii = 0;
    loop {
        counter += 1;
        if counter == 100 {
            break;
        }
        loop {
            free_block = find_free_block(&blocks, ii);
            if free_block.is_some() {
                block_to_fill = find_block_to_fill(&blocks, free_block.unwrap().1);
                if 
                    block_to_fill.is_some() && free_block.unwrap().0 < block_to_fill.unwrap().0
                {
                    // this is the good option
                    break;
                }
                //if it fails here it could not find a block to fill
                ii = free_block.unwrap().0;
            }
            failed = true;
            break;
        }
        if failed {
            break;
        } else {
            if !recursive_swap(
                blocks, 
                free_block.unwrap().0, 
                free_block.unwrap().0 + block_to_fill.unwrap().1, 
                block_to_fill.unwrap().0
            ) {
                println!("fb: {}, fbs: {}, bf: {}, bfs: {}, b[fb]: {}, b[bf]: {}, b.len: {}", 
                    free_block.unwrap().0,
                    free_block.unwrap().1,
                    block_to_fill.unwrap().0,
                    block_to_fill.unwrap().1,
                    blocks[free_block.unwrap().0].unwrap_or(666),
                    blocks[block_to_fill.unwrap().0].unwrap_or(666),
                    blocks.len(),
                );
                print_blocks(blocks);
                break;
            } else {
                ii = free_block.unwrap().0 + free_block.unwrap().1;
            }
        }

        println!("fb: {}, fbs: {}, bf: {}, bfs: {}, b[fb]: {}, b[bf]: {}, b.len: {}, ii: {}", 
            free_block.unwrap().0,
            free_block.unwrap().1,
            block_to_fill.unwrap().0,
            block_to_fill.unwrap().1,
            blocks[free_block.unwrap().0].unwrap_or(666),
            blocks[block_to_fill.unwrap().0].unwrap_or(666),
            blocks.len(),
            ii
        );
        print_blocks(blocks);
    }

    //println!("{}, {}", find_free_block(blocks, 0).unwrap().0, find_free_block(blocks, 0).unwrap().1);
    print_blocks(blocks);
    find_answer(blocks);
}

fn find_block_to_fill(blocks: &Vec<Option<u32>>, size_to_find: usize) -> Option<(usize, usize)> {
    let mut c_inv_start_inx = blocks.len();
    let mut c_size = 0;
    let mut to_find = None;

    for ii in blocks.iter().rev()//.skip(blocks.len() - 1)
        .enumerate().filter(|(_inx, &val)| val.is_some())
        .map(|(inx,  _val)| inx)
    {
        let rev_inx = blocks.len() - 1 - ii;

        if to_find.is_none() {
            to_find = blocks[rev_inx].clone();
            c_size = 1;
            c_inv_start_inx = rev_inx;
        } else if to_find.unwrap() == blocks[rev_inx].unwrap() {
            c_size += 1;
        } else if to_find.unwrap() != blocks[rev_inx].unwrap() {
            if c_size <= size_to_find {
                return Some((c_inv_start_inx - c_size + 1, c_size - 1));
            } else {
                to_find = blocks[rev_inx].clone();
                c_size = 1;
                c_inv_start_inx = rev_inx;
            }
        }
    }
    return None;
}

/// free_block is (index, size), index_skip is where in blocks to start
fn find_free_block(blocks: &Vec<Option<u32>>, index_skip: usize) -> Option<(usize, usize)> {
    let mut c_start_inx = index_skip;
    let mut c_count = 0;

    for ii in index_skip..blocks.len() {
        if blocks[ii].is_none() {
            if c_start_inx == index_skip {
                c_start_inx = ii;
                c_count += 1;
            } else if c_start_inx + c_count == ii {
                c_count += 1;
            } else {
                break;
            }
        } else {
            if c_start_inx != index_skip {
                break;
            }
        }
    }
    
    if c_start_inx != index_skip {
        return Some((c_start_inx, c_count));
    }
    return None;
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

//works as expected
fn recursive_swap(blocks: &mut Vec<Option<u32>>, s_inx: usize, e_inx: usize, to_inx: usize) -> bool {
    let dif = e_inx - s_inx - 1;
    if 
        e_inx < blocks.len() &&
        to_inx + dif < blocks.len()
    {
        for ii in 0..=dif {
            blocks.swap(s_inx + ii, to_inx + ii);
        }
    } else {
        println!("error trying to swap s: {s_inx}, e: {e_inx}, to: {to_inx}");
        return false;
    }
    return true;
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