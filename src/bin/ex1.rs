// #Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
//  and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main(){
    let mut s = vec![3,4,1,2,10];
    let mut counter;
    for i in 0..s.len(){
        for j in 0..s.len(){
            if s[j] > s[i]{
               counter = s[i];
               s[i] = s[j];
               s[j] = counter;
            }
        }
    }

    let median = s[2];
    println!("Median is {}",median);
    println!("{:?}",s)


}