use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    //let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vals:Vec<Vec<i32>> = Vec::new();
    
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            vals.push( line.chars().map(|s| s as i32 - 0x30).collect()); 
        }
    }
    let mut tot = 0;
    for i in 0..vals.len(){
        for j in 0..vals[i].len(){
            if vals[i][j] == 0{
                // let mut tot_heads : HashSet<(i32,i32)> = HashSet::new();
                //println!("--------start trail head------");

                find_trail((i as i32,j as i32), vals.clone(),&mut tot);
                // tot +=tot_heads.len();
                //println!("--------end trail head------");
            }
        }
    }
    println!("{}", tot);
}
fn find_trail(location:(i32,i32), map: Vec<Vec<i32>>, tot:&mut i32 ){
    let direction:Vec<(i32,i32)> = vec![(0,-1),(-1,0),(0,1),(1,0)];
        for dir in direction{
         let new_location = (location.0 + dir.0, location.1 + dir.1);
            if new_location.0 >=0 && (new_location.0 as usize) < map.len()  && new_location.1 >= 0 && new_location.1 <(map[0].len() as i32){
                
                
                if map[new_location.0 as usize][new_location.1 as usize]- 1 == map[location.0 as usize][location.1 as usize]{
                    if map[new_location.0 as usize][new_location.1 as usize] == 9{
                       // println!("reached the end of the trail");
                        *tot+=1;
                    }
                    // println!("new location {:?}",new_location);
                    // println!("new location value {:?}",map[new_location.0 as usize][new_location.1 as usize]);
                    find_trail(new_location, map.clone(), tot);
                }
                //println!("tot {:?}", tot);
            }
            
        }
} 