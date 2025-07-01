use std::io;
use rand::{self, Rng};
fn main(){
    println!("Enter a number 1-100");
    
    
    let mut randomizer = rand::thread_rng();
    let random = randomizer.gen_range(0..100);

    loop {
        let mut gs = String::new();
        io::stdin().read_line(&mut gs).expect("Cannot read");
        let guess: i32 = gs.trim().parse().expect("TF");
        if guess == random{
            println!("ay");
            break;
        }
        else if guess > random{
            println!("2 high")
        }
        else{
            println!("2 lo")
        }
        
    }

}