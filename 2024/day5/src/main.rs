use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let time = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut orders: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates:Vec<Vec<i32>> = Vec::new();
    let mut break_time =false;
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {

            if line == ""{
                break_time = true;
                continue;
            }
            if !break_time {
                let temp_vec :Vec<i32> = line.split("|").map(|s| s.parse().unwrap()).collect();
                if let Some(value) = orders.get_mut(&temp_vec[1]){
                    value.push(temp_vec[0])
                }else{
                    orders.insert(temp_vec[1], vec![temp_vec[0]]);
                }
            }else{
                updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
            }
        }
    }
    // println!("orders {:?}", orders);
    // println!("updates {:?}", updates);
    let mut good_update :Vec<Vec<i32>> = Vec::new();
    let mut wrong_updates : Vec<Vec<i32>> = Vec::new();
    for order in updates{
        for i in 0..order.len()-1{
            let mut contained =false;
            if let Some( number) = orders.get(&order[i]){ 
                let temp_vec = &order[i+1..];
                for j in temp_vec{
                    contained = number.contains(j);
                    if contained {
                        break;
                    }
                }
            } 
            if contained {
                wrong_updates.push(order.clone());
                break;
            }else if i == order.len() -2{
                good_update.push(order.clone());
            }
        }
    }
    //println!("good updates {:?}", good_update);
    let mut tot :i32 = good_update.into_iter().map(|s| s[s.len() /2]).sum();
    println!("tot good updates{}", tot);
    //println!("wrong updates {:?}", wrong_updates.clone());

    let mut modified_updates : Vec<Vec<i32>> = Vec::new();
    for mut update in wrong_updates{
        let mut is_not_ordered = true;
        while is_not_ordered{
            ordering(&mut update, orders.clone());
            for i in 0..update.len()-1{
                let mut contained =false;
                if let Some( number) = orders.get(&update[i]){ 
                    let temp_vec = &update[i+1..];
                    for j in temp_vec{
                        contained = number.contains(j);
                        if contained {
                            break;
                        }
                    }
                } 
                if contained {
                    break;
                }else if i == update.len() -2{
                    is_not_ordered=false;
                    modified_updates.push(update.clone());
                }
            }
        }
    }
   
    //println!("modified updates{:?}", modified_updates);
    tot = modified_updates.into_iter().map(|s| s[s.len() /2]).sum();
    println!{"tot wrong : {}", tot};
    println!("time elapsed : {:?}", time.elapsed());
}

fn ordering( update:&mut Vec<i32>, orders: HashMap<i32,Vec<i32>>){
    let mut index = 0;
    let mut insert_index = 0;
    for i in 0..update.len()-1{
        let mut contained =false;
        if let Some( number) = orders.get(&update[i]){ 
            let temp_vec = &update[i+1..];
            for j in temp_vec{
                contained = number.contains(j);
                if contained {
                    for x in 0..number.len(){
                        if number[x] == *j{
                            index = x;
                            break;
                        }
                    }
                    for x in 0..update.len(){
                        if update[x] == number[index]{
                            index = x;
                            break;
                        }
                    }
                    break;
                }
            }
        } 
        if contained {
            insert_index = if i == 0 {0}else{i-1};
            break;
        }
    }
    let elem = update.remove(index);
    update.insert(insert_index,elem);
    

}
