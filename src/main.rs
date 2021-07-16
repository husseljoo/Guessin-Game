use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number=rand::thread_rng().gen_range(1..101);
    // println!("Your secret number is {}",secret_number);
    loop{

        let mut guess = String::new();
        println!("Enter your input guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //println!("You guessed {}",guess);
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Not a valid number my lord!");
                continue;
            }
            };

        match guess.cmp(&secret_number){
            Ordering::Less=> println!("Too small!"),
            Ordering::Greater=> println!("Too big!"),
            Ordering::Equal=> {
                println!("You win!");
                break;
                }
            }
    }

}
