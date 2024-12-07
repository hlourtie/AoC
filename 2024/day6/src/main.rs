use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;
fn main() {
    let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut map:Vec<Vec<char>> = Vec::new();
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            map.push(line.chars().collect());
        }
    }
    let mut guard_location : (usize, usize) = (0,0);
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            if map[i][j] =='^'{
                guard_location = (i ,j);
                break;
            }
        }
        if guard_location.0 != 0{
            break;
        }
    }
    let og_location = guard_location;
    let mut dir = 1;
    let mut obstacle_location : HashSet<(usize, usize)> = HashSet::new();
    while is_within_bounds(&guard_location, &map) {
       
        let current_guard_location:(usize, usize) = guard_location;
        match dir%4{
            1=> guard_location.0 -=1,
            2=> guard_location.1 +=1,
            3=>guard_location.0 +=1,
            0=>guard_location.1 -=1,
            _=> guard_location = guard_location
        }   
        if map[guard_location.0][guard_location.1] =='#'{
            guard_location = current_guard_location;
            dir+=1;
        }else{
            obstacle_location.insert(guard_location);
        }
     
    }
    let mut for_sure_obstacles  : HashSet<(usize, usize)> = HashSet::new();
    for obstacle in obstacle_location{
        // let mut cloned = map.clone();
        map[obstacle.0][obstacle.1] ='#';
        guard_location= og_location;
        dir =1;
        if is_infinite(&map, guard_location, dir){
            for_sure_obstacles.insert(obstacle);
        }
        map[obstacle.0][obstacle.1] ='.';

    }

    println!("tot {}", for_sure_obstacles.len());
    println!("time elapsed : {:?}", time.elapsed());
}

fn is_within_bounds(location: &(usize, usize), map: &Vec<Vec<char>>) -> bool {
    location.0 > 0
        && location.0 < map.len() - 1
        && location.1 > 0
        && location.1 < map[0].len() - 1
}

fn is_infinite(map:&Vec<Vec<char>>, mut guard_location:(usize, usize), mut dir:usize) ->bool{
  
    let mut hash_of_set:HashSet<((usize, usize), usize)> = HashSet::new();
    hash_of_set.insert((guard_location, dir%4));
    while is_within_bounds(&guard_location, map){
        
        let current_guard_location:(usize, usize) = guard_location;
        match dir%4{
            1=> guard_location.0 -=1,
            2=> guard_location.1 +=1,
            3=>guard_location.0 +=1,
            0=>guard_location.1 -=1,
            _=> guard_location = guard_location
        }
        if map[guard_location.0][guard_location.1] == '#'{
            guard_location = current_guard_location;
            dir+=1;
        }
        if hash_of_set.contains(&(guard_location, dir%4)){
            return true;
        }else{
            hash_of_set.insert((guard_location, dir%4));
        }
    }
    false
}

