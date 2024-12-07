use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let reg = Regex::new(r"(?:mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let mut text = String::new();
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            text =line;
                }
            }
    let regnum = Regex::new(r"\d{1,3}").unwrap();
    let mut tot = 0;
    let mut enable: bool = true;
    for mat in reg.find_iter(text.as_str()){
        //println!("{:?}", mat.as_str());
        if mat.as_str()=="do()"{
            enable = true;
            continue;
        }
        if mat.as_str()=="don't()"{
            enable = false;
            continue;
        }
        if enable{
        let mut vec_num:Vec<i32> =  Vec::new();
            for num in regnum.find_iter(mat.as_str()){
                //println!("{:?}", num);
                vec_num.push(num.as_str().parse().unwrap());
            }
       
        tot += vec_num[0] * vec_num[1];  
        }
    }
    println!("total: {}", tot);
}
