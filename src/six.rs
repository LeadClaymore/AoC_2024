use std::fs::File;
use std::io::{self, BufRead};

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
            m_line.push(false);
            //print!("{jj}");
        }
        graph.push(n_line);
        move_graph.push(m_line);
        //println!("");
    }

    let (x_max, y_max) = (graph[0].len() as i32, graph.len() as i32);
    move_graph[yy as usize][xx as usize] = true; //starting pos was traversed

    loop {
        let n_pos = (xx + dir.move_dir().0, yy + dir.move_dir().1) ;
        if within_range(n_pos.0, n_pos.1, x_max, y_max) {
            if graph[n_pos.1 as usize][n_pos.0 as usize] {
                dir = dir.turn();
            } else {
                (xx, yy) = n_pos;
                move_graph[yy as usize][xx as usize] = true;
            }
        } else {
            break;
        }
    }

    for line in move_graph {
        for place in line {
            if place {
                answer += 1;
            }
        }
    }

    println!("answer = {}", answer);
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

//end