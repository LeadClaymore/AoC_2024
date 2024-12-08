use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, PartialEq, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl Dir {
    fn turn(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    /// (x axis, y axis)
    fn move_dir(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    //fn print_char
}

#[allow(dead_code, unused_assignments)]
pub fn six_p1() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;

    match read_data(String::from("data/six.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }

    let mut graph = Vec::new();
    let mut move_graph = Vec::new();
    let (mut yy, mut xx) = (0, 0);
    let mut dir = Dir::Up;
    for ii in 0..data.len() {
        let mut n_line = Vec::new();
        let mut m_line = Vec::new();
        for jj in 0..data[ii].len() {
            if data[ii][jj] == '.' {
                n_line.push(false);
            } else if data[ii][jj] == '#' {
                n_line.push(true);
            } else if data[ii][jj] == '^' {
                n_line.push(false);
                (yy, xx) = (ii as i32, jj as i32);
            }
            m_line.push(None);
            //print!("{jj}");
        }
        graph.push(n_line);
        move_graph.push(m_line);
        //println!("");
    }

    let (x_max, y_max) = (graph[0].len() as i32, graph.len() as i32);
    move_graph[yy as usize][xx as usize] = Some(Dir::Up); //starting pos was traversed

    loop {
        let n_pos = (xx + dir.move_dir().0, yy + dir.move_dir().1) ;
        if within_range(n_pos.0, n_pos.1, x_max, y_max) {
            if graph[n_pos.1 as usize][n_pos.0 as usize] {
                dir = dir.turn();
            } else {
                (xx, yy) = n_pos;
                move_graph[yy as usize][xx as usize] = Some(dir.clone());
            }
        } else {
            break;
        }
    }

    for line in move_graph {
        for place in line {
            if place.is_some() {
                answer += 1;
            }
        }
    }

    println!("answer = {}", answer);
    Ok(())
}

#[allow(dead_code, unused_assignments)]
pub fn six_p2() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;
    let mut answer2 = 0;

    match read_data(String::from("data/six.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }

    let mut graph = Vec::new();
    let mut move_graph = Vec::new();
    let (mut yy, mut xx) = (0, 0);
    let mut _dir = Dir::Up;
    for ii in 0..data.len() {
        let mut n_line = Vec::new();
        let mut m_line = Vec::new();
        for jj in 0..data[ii].len() {
            if data[ii][jj] == '.' {
                n_line.push(false);
            } else if data[ii][jj] == '#' {
                n_line.push(true);
            } else if data[ii][jj] == '^' {
                n_line.push(false);
                (yy, xx) = (ii as i32, jj as i32);
            }
            m_line.push(None);
            //print!("{jj}");
        }
        graph.push(n_line);
        move_graph.push(m_line);
        //println!("");
    }

    let (_x_max, _y_max) = (graph[0].len() as i32, graph.len() as i32);
    gen_move_graph(&graph, &mut move_graph, (xx as usize, yy as usize), None);
    
    let mut new_move_graph: Vec<Vec<Option<Dir>>> = Vec::new();
    for ii in 0..graph.len() {
        let mut new_m_graph = Vec::new();
        for _jj in 0..graph[ii].len() {
            new_m_graph.push(None);
        }
        new_move_graph.push(new_m_graph);
    }

    // brute force answer to p2
    for ii in 0..move_graph.len() {
        for jj in 0..move_graph[ii].len() {
            // if its not the starting place of the guard
            if (jj, ii) != (xx as usize, yy as usize) {
                clear_move_graph(&mut new_move_graph);
                if !gen_move_graph(&graph, &mut new_move_graph, (xx as usize, yy as usize), Some((jj as i32, ii as i32))) {
                    answer2 += 1;
                }
            }
        }
    }

    for ii in 0..move_graph.len() {
        for jj in 0..move_graph[ii].len() {
            if move_graph[ii][jj].is_some() {
                answer += 1;

                // tried to do the smart way
                // // oo and uu is how much the graph cursor will move each index if turned at this point
                // let ((uu, oo), n_dir, c_dir) = match &move_graph[ii][jj] {
                //     Some(cur_dir) => (cur_dir.turn().move_dir(), cur_dir.turn(), *cur_dir),
                //     None => ((0, 0), Dir::Up, Dir::Up),
                // };
                
                // if 
                //     within_range(ii as i32 + c_dir.move_dir().1, jj as i32 + c_dir.move_dir().0, x_max, y_max) &&
                //     !graph[(ii as i32 + c_dir.move_dir().1) as usize][(jj as i32 + c_dir.move_dir().0) as usize] 
                // {
                //     let mut inx = 0;
                //     while within_range(
                //         jj as i32 + uu * inx, 
                //         ii as i32 + oo * inx, 
                //         x_max, 
                //         y_max
                //     ) {
                //         //println!("");
                //         let o_dir = move_graph[(ii as i32 + oo * inx) as usize][(jj as i32 + uu * inx) as usize];
                //         if o_dir.is_some() {
                //             if o_dir.unwrap() == n_dir {
                //                 answer2 += 1;
                //                 print_graph(&graph, ((jj as i32 + uu * inx) as usize, (ii as i32 + oo * inx) as usize));
                //                 //print!("\n{jj},{ii} to {},{}", );
                //                 break;
                //             } else if graph[(ii as i32 + oo * inx) as usize][(jj as i32 + uu * inx) as usize] {
                //                 //wall hit without loop (idk if 2 bounces are needed)
                //                 break;
                //             }
                //         }
                //         inx += 1;
                //     }
                // }
            }
        }
    }

    println!("");
    println!("answer = {}", answer);
    println!("answer2 = {}", answer2);
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
        for jj in line.chars() {
            c_line.push(jj);
        }
        data.push(c_line);
    }
    Ok(())
}

fn within_range(x: i32, y: i32, x_max: i32, y_max: i32) -> bool {
    y >= 0 &&
    y < y_max &&
    x >= 0 &&
    x < x_max
}

fn _print_graph(data: &Vec<Vec<bool>>, pos: (usize, usize)) {
    for ii in 0..data.len() {
        if ii == 0 {
            println!("");
            println!("");
            for _jj in 0..data[ii].len() {
                print!("-");
            }
            println!("");
        }
        for jj in 0..data[ii].len() {
            if (jj, ii) == pos {
                print!("O");
                continue;
            }
            if data[ii][jj] {
                print!("#");
            } else {
                print!(".");
            }
            
        }
        println!("");
    }
}

fn clear_move_graph(move_graph: &mut Vec<Vec<Option<Dir>>>) {
    for ii in 0..move_graph.len() {
        for jj in 0..move_graph[ii].len() {
            move_graph[ii][jj] = None;
        }
    }
}

///start is x, y
fn gen_move_graph(graph: &Vec<Vec<bool>>, move_graph: &mut Vec<Vec<Option<Dir>>>, start: (usize, usize), new_block: Option<(i32, i32)>) -> bool {
    let mut dir = Dir::Up;
    let (mut xx, mut yy) = (start.0 as i32, start.1 as i32);
    let (x_max, y_max) = (graph[0].len() as i32, graph.len() as i32);
    move_graph[start.1][start.0] = Some(Dir::Up); //starting pos was traversed

    loop {
        let n_pos = (xx + dir.move_dir().0, yy + dir.move_dir().1) ;
        if within_range(n_pos.0, n_pos.1, x_max, y_max) {
            if 
                graph[n_pos.1 as usize][n_pos.0 as usize] ||
                n_pos == new_block.unwrap_or((-1, -1))
            {
                dir = dir.turn();
            } else {
                (xx, yy) = n_pos;
                if 
                    move_graph[yy as usize][xx as usize].is_some() &&
                    move_graph[yy as usize][xx as usize].unwrap() == dir
                {
                    //inf loop
                    return false;
                }
                move_graph[yy as usize][xx as usize] = Some(dir.clone());
            }
        } else {
            return true;
        }
    }
}
//end