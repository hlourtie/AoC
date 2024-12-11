use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
//use std::collections::{HashMap, HashSet};

fn main() {
    let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vals:Vec<i32> = Vec::new();
    
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            vals = line.chars().map(|s| s as i32 - 0x30).collect(); 
        }
    }
    let mut tup_vals : Vec<(i32, usize, bool,i32)> = Vec::new();
    let mut index = 0;
    for n in (0..vals.len()).step_by(2){
        tup_vals.push((vals[n],n/2,true,vals[n+1]));
    }
    let mut compressed : Vec<usize> = Vec::new();
    let mut i = 0;
    
    loop{
        if tup_vals[i].2{ 
            for _ in 0..(tup_vals[i].0){
                compressed.push(tup_vals[i].1);
            }
        }else{
            for _ in 0..(tup_vals[i].0){
                compressed.push(0);
            }
        }
        let mut index = i;
        tup_vals[i].2=false;
        let mut tup_val_back= tup_vals[tup_vals.len()-1];
        let mut index_back= tup_vals.len()-1;
        let mut j = 1;
        let mut vals_index = tup_vals.len()-j;
        let mut remainder_back = tup_vals[vals_index].0;
        let mut size_open = tup_vals[i].3;
        let mut to_put = tup_val_back.2;
        
        while size_open > 0 && index <= index_back{
            if size_open >=remainder_back && to_put {
                for _ in 0..remainder_back{
                    compressed.push(index_back);
                    size_open-=1;
                }
                tup_vals[index_back].2 = false;
            }
                j+=1;
                vals_index = tup_vals.len()-j;
                remainder_back = tup_vals[vals_index].0;
                index_back= vals_index ;
                to_put = tup_vals[vals_index].2
        }

        while size_open > 0{
            compressed.push(0);
            size_open-=1;
        }
        i+=1;
        let mut is_finished = true;
        for l in (i..tup_vals.len()){
            if tup_vals[l].2{
                is_finished= false;
                break;
            }
        }
        if is_finished{
            break;
        }
    }
    let mut  tot: i64 =0;
    for k in 0..compressed.len(){
        tot+= compressed[k] as i64 * k as i64;
    }
    println!("time elapsed {:?}", time.elapsed());
    println!("tot {}", tot);
}