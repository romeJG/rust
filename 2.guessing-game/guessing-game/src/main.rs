use std::io;
use rand::Rng;

fn main(){
   println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is: {secret_number}");

    println!("Enter your number");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("You guessed: {}",guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small bro"),
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal => println!("Nice! you winnn!!!"),
    }
    
}
