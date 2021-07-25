use std::io;
use rand::Rng;
use std::cmp::Ordering;

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value<1 || value>100 {
            panic!("{} is out of range",value);
        }
        Guess{value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

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

        let num= Guess::new(guess.trim().parse().unwrap());

        match Guess::value(&num).cmp(&secret_number){
            Ordering::Less=> println!("Too small!"),
            Ordering::Greater=> println!("Too big!"),
            Ordering::Equal=> {
                println!("You win!"); break; } }
    }

}
