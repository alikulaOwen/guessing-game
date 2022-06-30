use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number.");

    let random_number = rand::thread_rng().gen_range(1..=100);


    loop {
            println!("Please enter a number between 1-100: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess : {guess}");
    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    match guess.cmp(&random_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
                println!("You Lucky!");
                break;
            }
    }
    }
}
