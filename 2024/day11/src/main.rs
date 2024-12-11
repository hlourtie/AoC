use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn main() {
    let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
   
    let mut result:Vec<usize> = Vec::new();
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            result = line
            .split_ascii_whitespace()
            .filter_map(|sval| sval.parse::<usize>().ok()).collect();
        }
    }
    println!("{:?}", time.elapsed());
        // println!("{}", result);
        let mut has_cache : HashMap<(usize,usize),usize> = HashMap::new();
        let mut vals:Vec<usize>= Vec::new();
    for i in result{
        vals.push(get_stone(i,75,&mut has_cache));
    }
    println!("{:?}", time.elapsed());
    println!("{}",vals.into_iter().sum::<usize>());
}
   


fn get_stone(vals:usize, blinks:usize , results:&mut HashMap<(usize,usize),usize>) -> usize{
    if blinks == 0{
        return 1;
    }
    let mut temp = 0;
    if let Some(value) = results.get(&(vals,blinks)){
        *value    
    }else{
        let digi_len = (vals as f64).log10().floor() as usize + 1;
        if vals == 0{
            temp = get_stone(1, blinks-1, results);
        }else if digi_len %2 == 0{
            let half_len = digi_len / 2;
             let divisor = 10usize.pow(half_len as u32);
             let left = vals / divisor;
            let right = vals % divisor;

            temp = get_stone(left, blinks-1, results) +get_stone(right, blinks-1, results);
        }else{
            temp = get_stone(vals*2024, blinks-1, results)
        }
        results.insert((vals,blinks), temp);
        temp
    }
}
