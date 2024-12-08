use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code, unused_assignments)]
pub fn eight() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;

    match read_data(String::from("data/eight.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    print_data(&data);
    let mut nodes: Vec<Vec<Option<char>>> = vec![vec![None; data[0].len()]; data.len()];
    // this makes each vector within the 2d vec an unique instance
    // into iter itterates over the vector
    // map takes each instance and replaces it with a clone of itself
    // and collect is needed to apply this function to return them into the vector they came from 
    // (otherwise it would be an itterator)
    nodes = nodes.into_iter().map(|v| v.clone()).collect();

    let mut anti_nodes: Vec<Vec<Option<char>>> = vec![vec![None; data[0].len()]; data.len()];
    anti_nodes = anti_nodes.into_iter().map(|v| v.clone()).collect();

    process_data(&data, &mut nodes, &mut anti_nodes);
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
    println!("--------------------------");
    for line in data {
        for ii in line {
            print!("{ii}");
        }
        println!("");
    }
}

fn process_data(data: &Vec<Vec<char>>, nodes: &mut Vec<Vec<Option<char>>>, anti_nodes: &mut Vec<Vec<Option<char>>>) {
    for ii in 0..data.len() {
        for jj in 0..data[ii].len() {
            if data[ii][jj] != '.' {
                nodes[ii][jj] = Some(data[ii][jj]);
                find_anti_node(data, anti_nodes, ii, jj);
            }
        }
    }
    print_nodes(&anti_nodes);
    println!("there are {} antinodes within the map", count_anti_nodes(anti_nodes));
}

fn print_nodes(nodes: &Vec<Vec<Option<char>>>) {
    println!("--------------------------------------------------------");
    // learned from count_anti_nodes and some documentation
    nodes.iter().for_each(
        |row| {
            row.iter().for_each(
                |&node| {
                    print!("{}", node.unwrap_or('.'));
                }
            );
            println!("");
        }
    );
}

fn find_anti_node(data: &Vec<Vec<char>>, anti_nodes: &mut Vec<Vec<Option<char>>>, n_ii: usize, n_jj: usize) {
    let node = data[n_ii][n_jj];
    for ii in 0..data.len() {
        for jj in 0..data[ii].len() {
            if data[ii][jj] == node && (ii, jj) != (n_ii, n_jj) {
                // I want them to be i32 because I need to check if they are less then 0 and not have an integer underflow
                // ii then jj
                let n_anti_n = (ii as i32 + (ii as i32 - n_ii as i32), jj as i32 + (jj as i32 - n_jj as i32));
                if within(
                    n_anti_n.0, 
                    n_anti_n.1, 
                    data.len(), data[0].len()
                ) {
                    if anti_nodes[n_anti_n.0 as usize][n_anti_n.1 as usize] != None {
                        //overlap
                    }
                    anti_nodes[n_anti_n.0 as usize][n_anti_n.1 as usize] = Some(node);
                }
            }
        }
    }
}

fn within(y: i32, x: i32, y_max: usize, x_max: usize) -> bool {
    return x >= 0 && y >= 0 && x < x_max as i32 && y < y_max as i32;
}

fn count_anti_nodes(anti_nodes: &Vec<Vec<Option<char>>>) -> usize {
    // this takes the antinodes into_iters them into a iterator of the 2d vec
    // then takes each 1d vec of the 2d vec and itterates over each option within the 1d vec
    // and for each option it filters them by wether they have a char in them (aka if they are some)
    // then it takes that collection of filtered values for each row / 1d array and gets the size of elements (aka count)
    // and sums up the count for each vector in the 2d vector and returns it

    // tbh I chatgpted this but now I know I will be using this method for counting from now on
    return anti_nodes.iter()
        .map(|row| row.iter().filter(|&a_node| a_node.is_some()).count())
        .sum::<usize>();
}
//end