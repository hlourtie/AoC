use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut reports: Vec<Vec<i32>> = Vec::new();
   
    
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            reports.push(line.split_whitespace().map(|s| s.parse().unwrap()).collect());
                }
            }

    let mut tot = 0;
    for report in reports {
        // if report[0] > report[1] && is_decreasing_and_valid(report.clone()) {
        //     tot+=1;
        // }else if report[0] < report[1] && is_increasing_and_valid(report.clone()){
        //     tot+=1;
        // }
        if is_decreasing_and_valid(report.clone(),0) || is_increasing_and_valid(report, 0){
            tot+=1;
        }
    }
    println!("number of reports: {}", tot);
}

fn is_decreasing_and_valid (report:Vec<i32>, mut prob_num:i32)-> bool{
    
    for i in 0..(report.len()-1){
        if ((report[i] - report[i+1]) < 1 || (report[i] - report[i+1]) > 3 )&& prob_num > 1{
            return false;
        }else if (report[i] - report[i+1]) < 1 || (report[i] - report[i+1]) > 3{
            prob_num+=1;
            let mut temp_vec1 = report.clone();
           temp_vec1.remove(i);
           let mut temp_vec2 = report.clone();
           temp_vec2.remove(i+1);
           if is_decreasing_and_valid(temp_vec1, prob_num) || is_decreasing_and_valid(temp_vec2, prob_num){
            return true;
           }else{
            return false
           }
        }
    }

    if prob_num >1{
        return false;
    }else{
        println! ("decreasing and {} problems: {:?}", prob_num, report);
            return true; 
        }
}

fn is_increasing_and_valid (report:Vec<i32>, mut prob_num: i32)-> bool{
    
    for i in 0..(report.len()-1){
        if ((report[i+1] - report[i]) < 1 || (report[i+1] - report[i]) > 3) && prob_num >1{
            return false;
        } else if  (report[i+1] - report[i]) < 1 || (report[i+1] - report[i]) > 3 {
            prob_num+=1;
           let mut temp_vec1 = report.clone();
           temp_vec1.remove(i);
           let mut temp_vec2 = report.clone();
           temp_vec2.remove(i+1);
           if is_increasing_and_valid(temp_vec1, prob_num) || is_increasing_and_valid(temp_vec2, prob_num){
            return true;
           }else{
            return false
           }
        }
    }
   
    if prob_num >1{
        return false;
    }else{
        println! ("increasing and {} problems: {:?}", prob_num, report);
            return true; 
        }
}