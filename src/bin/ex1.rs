
use std::{collections::HashMap, fs::exists, hash::Hash,};
fn main(){
    let mut s = vec![3,4,1,2,10,1,66,0,0];
    let mut counter;
    let mut largest_value=0;
    let mut largest_key = 0;
    for i in 0..s.len(){
        for j in 0..s.len(){
            if s[j] > s[i]{
               counter = s[i];
               s[i] = s[j];
               s[j] = counter;
            }
        }
    }

    let median = s[(s.len()/2)];
    println!("Median is {median:?}",);
    println!("{:?}",s);

    let mut hashhy: HashMap<usize, u32>  = HashMap::new();
    for num in s{
        if hashhy.contains_key(&num)==false{
            hashhy.insert(num, 0);
        }
        else {
            let g =hashhy.get(&num);
            let new_value = *g.unwrap() + 1;
            hashhy.insert(num, new_value);
            if largest_value < new_value {
                largest_value = new_value;
                largest_key = num;
            }
        }
    }
    println!("{largest_value:?}okk {largest_key:?}");
    
    

}