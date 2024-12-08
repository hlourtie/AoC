use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn main() {
    //let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vals:Vec<Vec<char>> = Vec::new();
    
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            vals.push(line.chars().collect()); 
        }
    }
    let hight = vals.len() as i32;
    let girth = vals[0].len() as i32;

    let mut hash_coordinates :HashMap<char,Vec<(i32,i32)>> = HashMap::new();
    for i in 0..vals.len(){
        for j in 0..vals[i].len(){
            //&& vals[i][j] !='#'
            if vals[i][j] != '.' && vals[i][j] !='#' {
                if let Some(value) = hash_coordinates.get_mut(&vals[i][j]){
                    value.push((i as i32,j as i32));
                }else{
                    hash_coordinates.insert(vals[i][j], vec![(i as i32,j as i32)]);
                }
            }
        }
    }
    //let mut tot = 0;
    let mut hash_set_of_coordinates: HashSet<(i32,i32)> = HashSet::new();
    for (key, value) in hash_coordinates{
        println!("Key {}, value: {:?}", key, value);
        for i in 0..value.len(){
            let prime = value[i];
            hash_set_of_coordinates.insert(prime);
            for j in i+1..value.len(){
                let second = value[j];
                hash_set_of_coordinates.insert(second);
                 let temp = antinodes(prime, second, girth, hight);
                 for x in temp.iter(){
                    hash_set_of_coordinates.insert(*x);
                 }
                
            }
        }
    }
    println!("{}", hash_set_of_coordinates.len());
}

fn antinodes(prime:(i32,i32), second:(i32,i32), girth:i32,hight:i32) ->Vec<(i32,i32)> {
    let x_diff = prime.0 - second.0;
    let y_diff = prime.1 - second.1;
    let mut antinodes:Vec<(i32,i32)> =Vec::new();

    let mut new1 = prime;
    let mut new2 = second;
    while in_bound(new1,girth,hight) || in_bound(new2,girth,hight) {
        if x_diff < 0 {
            if y_diff < 0{
                new1 = (new1.0 - x_diff.abs(), new1.1 - y_diff.abs());
                new2 = (new2.0 + x_diff.abs(), new2.1 + y_diff.abs());
            }else{
                new1 = (new1.0 - x_diff.abs(), new1.1 + y_diff.abs());
                new2 = (new2.0 + x_diff.abs(), new2.1 - y_diff.abs());

            }
        }else{
            if y_diff < 0{
                new1 = (new1.0 + x_diff.abs(), new1.1 - y_diff.abs());
                new2 = (new2.0 - x_diff.abs(), new2.1 + y_diff.abs());

            }else{
                new1 = (new1.0 + x_diff.abs(), new1.1 +y_diff.abs());
                new2 = (new2.0  - x_diff.abs(), new2.1 - y_diff.abs());
            }
        }
        println!("new1 {:?}, new2 {:?}", new1, new2);
        
        if in_bound(new1,girth,hight){
        antinodes.push(new1);
        }
        
        if in_bound(new2,girth,hight){
            antinodes.push(new2);
        }
    }
    println!("antinodes: {:?}", antinodes);
    antinodes
}
fn in_bound(tup : (i32,i32), bound:i32, other:i32) -> bool{
    tup.0 >= 0 && tup.0< other && tup.1 >=0 && tup.1 < bound
}