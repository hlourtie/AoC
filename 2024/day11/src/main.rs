use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use cached::proc_macro::cached;


fn main() {
    //let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vals:Vec<usize>= Vec::new();
    let mut result = 0;
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            result = line
            .split_ascii_whitespace()
            .filter_map(|sval| sval.parse::<usize>().ok())
            .map(|stone| compute_stone(stone, 75))
            .sum();
        }
    }
    // let mut results: Vec<usize> = Vec::new();
    // let num_it = 75;
    // for i in 0..vals.len(){
    //     results.push(compute_stone(vals[i],num_it));
        
    // }
        println!("{}", result);

}
   

#[cached]
fn compute_stone(stone: usize, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let num_digits = (stone as f64).log10().floor() as usize + 1;

    if stone == 0 {
        compute_stone(1, depth - 1)
    } else if num_digits % 2 == 0 {
        let half_len = num_digits / 2;
        let divisor = 10usize.pow(half_len as u32);

        let left_num = stone / divisor;
        let right_num = stone % divisor;

        compute_stone(left_num, depth - 1) + compute_stone(right_num, depth - 1)
    } else {
        compute_stone(stone * 2024, depth - 1)
    }
}


// fn get_stone(vals:i64, temp: &mut Vec<i64>, results:&mut HashMap<i64,(i64,i64)>){
//     //println!("{}", vals);
//     if vals.to_string().len() %2 == 0{
//         let value_len = vals.to_string().len();
//         let first_part =  vals/(10_i32.pow(value_len as u32/2) as i64);
//         let second_part = vals - (first_part *(10_i32.pow(value_len as u32/2) as i64));
//         temp.push(first_part);
//         temp.push(second_part);
//         results.insert(vals,(first_part,second_part));

//     }else{
//         temp.push(vals * 2024);
//         results.insert(vals,(0,2024));
//     }
// }
