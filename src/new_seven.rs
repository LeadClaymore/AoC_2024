use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operator {
    Addition,
    Multiplication,
    Concatanation,
    // Subtraction,
    // Division,
}
impl Operator {
    fn apply_operator(&self, lhs: i64, rhs: i64) -> i64 {
        return match self {
            Operator::Addition => lhs + rhs,
            Operator::Multiplication => lhs * rhs,
            Operator::Concatanation => format!("{}{}", lhs, rhs).parse().unwrap(),
        }
    }

    fn get_all_ops(&self, op_list: &mut Vec<Operator>) {
        op_list.push(Operator::Addition);
        op_list.push(Operator::Multiplication);
        op_list.push(Operator::Concatanation);
        // op_list.push(Operator::Subtraction);
        // op_list.push(Operator::Division);
    }

    fn sym(&self) -> &str {
        return match &self {
            Operator::Addition => "+",
            Operator::Multiplication => "*",
            Operator::Concatanation => "||",
        }
    }
}

#[allow(dead_code, unused_assignments)]
pub fn seven_p1() -> io::Result<()> {
    let mut data = Vec::new();
    let mut answer = 0;

    match read_data(String::from("data/seven.txt"), &mut data) {
        Ok(_) =>  println!("Data read"),
        Err(ret) => return Err(ret),
    }
    let mut operands = Vec::new();
    let mut results = Vec::new();
    print_data(&data);
    process_data(&data, &mut operands, &mut results);
    print_p_data(&operands, &results);
    find_answer(&mut operands, &mut results);

    //println!("answer = {}", answer);
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

fn print_p_data(operands: &Vec<Vec<i64>>, results: &Vec<i64>) {
    println!("--------------------------");
    for ii in 0..operands.len() {
        print!("{}: ", results[ii]);
        for jj in 0..operands[ii].len() {
            print!("{} ", operands[ii][jj]);
        }
        println!("");
    }
}

fn process_data(data: &Vec<Vec<char>>, operands: &mut Vec<Vec<i64>>, results: &mut Vec<i64>) {
    for line in data {
        let mut res = true;
        let mut num = 0;
        let mut ops = Vec::new();
        for &c in line {
            if res {
                if c == ':' {
                    results.push(num);
                    res = false;
                    num = 0;
                } else if c.is_numeric() {
                    num = num * 10 + c.to_digit(10).unwrap() as i64;
                }
            } else {
                if c == ' ' && num != 0 {
                    ops.push(num);
                    num = 0;
                } else if c.is_numeric() {
                    num = num * 10 + c.to_digit(10).unwrap() as i64;
                }
            }
        }
        if num != 0 {
            ops.push(num);
        }
        operands.push(ops);
    }
}

fn find_answer(operands: &mut Vec<Vec<i64>>, results: &mut Vec<i64>) {
    let mut answer = 0;
    for ii in 0..operands.len() {
        let mut op_list: Vec<Operator> = Vec::new();
        if add_opperands(0, &mut operands[ii].clone(), results[ii], &mut op_list, None) {
            answer += results[ii];
        }
    }
    println!("answer = {}", answer);
}

fn add_opperands(c_inx: usize, line: &mut Vec<i64>, result: i64, operands: &mut Vec<Operator>, new_op: Option<Operator>) -> bool {
    if new_op.is_some() {
        operands.push(new_op.unwrap());
    }

    if c_inx == line.len() - 1 {
        return calc_line(line, result, operands);
    }

    let mut op_list: Vec<Operator> = Vec::new();
    Operator::Addition.get_all_ops(&mut op_list);
    for ii in 0..op_list.len() {
        if add_opperands(c_inx + 1, line, result, &mut operands.clone(), Some(op_list[ii].clone())) {
            return true;
        }
    }
    return false;
}

fn calc_line(line: &mut Vec<i64>, result: i64, operands: &mut Vec<Operator>) -> bool {
    let mut n_line = line.clone();
    let mut ret_str = String::from(format!("{} = ", result));
    for ii in 0..operands.len() {
        ret_str += &line[ii].to_string();
        ret_str += &" ";
        ret_str += operands[ii].sym();
        ret_str += &" ";
    }
    ret_str += &line[operands.len()].to_string();

    loop {
        if n_line.len() <= 1 {
            if n_line.len() == 0 {
                //TODO error
                return false;
            }
            if n_line[0] == result {
                println!("{}", ret_str);
                return true;
            }
            return false;
        }

        // // I SPENT TOO LONG LOOKING AT THIS CODE BECAUSE THE PROGRAM EXPECTS NO PEMDAS AKA ADDITION BEFORE MULT
        // // I SPENT TIME CODING PEMDAS INTO THIS BLOODY THING FOR IT NOT TO MEAN ANYTHING
        // // IT WORDKED FOR 2 OF THE THREE EXAMPLES AHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
        // let mut f_mut = false;
        // for ii in 0..operands.len() {
        //     if operands[ii] == Operator::Multiplication {
        //         f_mut = true;
        //         let t_num = n_line[ii] * n_line[ii + 1];
        //         n_line.remove(ii);
        //         n_line.remove(ii);
        //         n_line.insert(ii, t_num);
        //         operands.remove(ii);
        //         break;
        //     }
        // }
        // if !f_mut {
        //     for ii in 0..operands.len() {
        //         if operands[ii] == Operator::Addition {
        //             let t_num = n_line[ii] + n_line[ii + 1];
        //             n_line.remove(ii);
        //             n_line.remove(ii);
        //             n_line.insert(ii, t_num);
        //             operands.remove(ii);
        //             break;
        //         }
        //     }
        // }

        for ii in 0..operands.len() {
            if operands[ii] == Operator::Multiplication {
                let t_num = Operator::Multiplication.apply_operator(n_line[ii], n_line[ii + 1]);
                n_line.remove(ii);
                n_line.remove(ii);
                n_line.insert(ii, t_num);
                operands.remove(ii);
                break;
            } else if operands[ii] == Operator::Addition {
                let t_num = Operator::Addition.apply_operator(n_line[ii], n_line[ii + 1]);
                n_line.remove(ii);
                n_line.remove(ii);
                n_line.insert(ii, t_num);
                operands.remove(ii);
                break;
            } else if operands[ii] == Operator::Concatanation {
                let t_num = Operator::Concatanation.apply_operator(n_line[ii], n_line[ii + 1]);
                n_line.remove(ii);
                n_line.remove(ii);
                n_line.insert(ii, t_num);
                operands.remove(ii);
                break;
            }
        }
    }
}

//end