use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    //let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vals:Vec<Vec<i64>> = Vec::new();
    let mut results : Vec<i64> = Vec::new();
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            let temp_vec: Vec<&str> = line.split(':').collect();
            results.push(temp_vec[0].parse().unwrap());
            vals.push(temp_vec[1].split_whitespace().map(|s| s.parse().unwrap()).collect())
        }
    }
    let mut tot = 0;
    for i in 0..vals.len(){
        if possible(results[i], vals[i].clone()){
            tot+=results[i];
        }

    }
    println!("total {}",tot)
}

fn possible(result : i64, vals:Vec<i64>)-> bool{

    let num_ops = vals.len() - 1;
    let mut op_combinations :Vec<Vec<char>> =Vec::new();
    let symbols = ["*", "+", "|"];

    let total_combinations = usize::pow(symbols.len() as usize, num_ops as u32);

    for num in 0..total_combinations {
        let mut combination = String::new();
        let mut temp_num = num;

        for _ in 0..num_ops {
            let index = temp_num % symbols.len();
            combination.push_str(symbols[index]);
            temp_num /= symbols.len();
        }

        op_combinations.push(combination.chars().collect());
    }
  
  
    //println!("{:?}", results);
    for ops in op_combinations {
        let mut total = vals[0];

        for i in 0..num_ops {

            total =  match ops[i]{
                '*' => total *vals[i+1],
                '+' => total + vals[i+1],
                '|' => {
                    let mut temp = total.to_string();
                    temp.push_str(&vals[i+1].to_string());
                    temp.parse().unwrap()
                },
                _=> total
            };

            if total > result{
                break;
            }
        }
        if total == result{
            return true;
        }
    }
    false
}