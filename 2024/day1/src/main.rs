use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    let mut occurences: HashMap<i32,usize> = HashMap::new();
    
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            let temp_vec: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            vec1.push(temp_vec[0]);
            vec2.push(temp_vec[1]);
            if let Some(value) = occurences.get_mut(&temp_vec[1]) {
                *value+=1;
            } else {
                occurences.insert(temp_vec[1], 1);
            }
                }
    }
    //part 1
    vec1.sort();
    vec2.sort();
    let mut tot = 0;
    for i in 0..vec1.len(){
        tot+= if vec1[i]> vec2[i]{vec1[i]-vec2[i]}else {vec2[i] - vec1[i]};
    }
    println!("total {}", tot);

    //part2
    println!("{:?}", occurences);
    tot= 0;
    for i in 0..vec1.len(){
         if let Some(value) = occurences.get(&vec1[i]) {
            tot+=(*value as i32 )* vec1[i];
        }else{
            continue;
        };
    }
    println!("total part2 {}", tot);
    
}
