// Here’s a small programming problem: 
// // write a function that takes a string of words separated by spaces and returns the first word it finds in that string. 
// // If the function doesn’t find a space in the string, 
// // the whole string must be one word, so the entire string should be returned.

use std::io::Bytes;

fn main(){
    let a = String::from("hoe tf");
    println!("{a}");
    let ok = decode(&a);
    println!("{ok}");

}

fn decode(stro:&String) -> &str{
    
    println!("{stro}");
    let by = stro.as_bytes();
    for (index,&refer) in by.iter().enumerate(){
        if refer == b' '{
          return &stro[0..index];  
          
        }
    }
    return &stro;

}