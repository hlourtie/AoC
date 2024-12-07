use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    //let reg = Regex::new(r"(?:mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let mut text:Vec<Vec<char>> = Vec::new();
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            text.push(line.chars().collect());
                }
    }

    //XMAS
    let mut tot = 0;
    for i in 1..text.len()-1{
        for j in 1..text[i].len()-1{
            if text[i][j] =='A'{
                tot+= find_x(text.clone(), i, j);
            }
            //part1
            // if text[i][j] =='X' || text[i][j] =='S'{
            //     let word = if text[i][j] == 'X' {vec!['X','M','A','S']} else{ vec!['S','A','M','X']};
            //   if j < (text[i].len()-3){
            //     tot+= find_right(word.clone(), text[i].clone(), j);
            //   }
            //   if  i < (text.len() - 3){ 
            //     tot+= find_down(word.clone(),text.clone(), i, j);

            //     if j >= 3 { 
            //         tot+= find_diag_left(word.clone(),text.clone(), i,j);
            //     }

            //     if j < (text[i].len()- 3){
            //         tot+= find_diag_right(word.clone(),text.clone(), i,j);
            //     }
            //   }
            // }
        }
    }
    println!("tot: {}", tot);
}

fn find_x( text:Vec<Vec<char>>, i:usize,j:usize)->usize{

    if ((text[i-1][j-1] == 'S' && text[i+1][j+1] =='M' )||(text[i-1][j-1] == 'M' && text[i+1][j+1] =='S')) && ((text[i-1][j+1] == 'S' && text[i+1][j-1] =='M')||(text[i-1][j+1] == 'M' && text[i+1][j-1] =='S')){
        return 1;
    }else{
        return 0;
    }
}
// fn find_right(word:Vec<char> ,file:Vec<char>, j:usize )-> usize{
//     let mut word_new :Vec<char> = Vec::new();
//     for i in j..(j+4){
//         if file[i] == word[i-j]{
//             word_new.push(file[i]);
//         }
//     }
//     if word_new.len() == 4{
//         println!("Find right j: {}",j);
//         return 1;
//     }else{
//         return 0;
//     }
// }

// fn find_down(word:Vec<char> ,file:Vec<Vec<char>>, i:usize, j:usize)->usize {
//     let mut word_new :Vec<char> = Vec::new();
//     for k in 0..4{
//         if file[i+k][j] == word[k]{
//             word_new.push(file[i+k][j]);
//         }
//     }
//     if word_new.len() == 4{
//         println!("Find down i: {}, j: {}",i, j);
//         return 1;
//     }else{
//         return 0;
//     }
// }

// fn find_diag_right(word:Vec<char> ,file:Vec<Vec<char>>, i:usize, j:usize)->usize {
//     let mut word_new :Vec<char> = Vec::new();
//     for k in 0..4{
//         if file[i+k][j+k] == word[k]{
//             word_new.push(file[i+k][j+k]);
//         }
//     }
//     if word_new.len() == 4{
//         println!("Find diag right i: {}, j: {}",i, j);
//         return 1;
//     }else{
//         return 0;
//     }
// }

// fn find_diag_left(word:Vec<char> ,file:Vec<Vec<char>>, i:usize, j:usize)->usize {
//     let mut word_new :Vec<char> = Vec::new();
//     for k in 0..4{
//         if file[i+k][j-k] == word[k]{
//             word_new.push(file[i+k][j-k]);
//         }
//     }
//     if word_new.len() == 4{
//         println!("Find diag left i: {}, j: {}",i, j);
//         return 1;
//     }else{
//         return 0;
//     }
// }
