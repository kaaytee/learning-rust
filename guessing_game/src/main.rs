use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret number is {secret_number}");
    loop {
        
        println!("Enter input a number");
        let mut g = String::new();
        
        io::stdin().read_line(&mut g).expect("Could not read string");
        
        let g: u32 = match g.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };
        
        println!("You guessed: {}", g);
        
        match g.cmp(&secret_number) {
            Ordering::Less => println!("{g} is smaller than the number"),
            Ordering::Greater => println!("{g} is greater than the number"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }



    

    
}
