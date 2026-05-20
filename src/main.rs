use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("----------Guess the Number----------");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        // println!("You secret number is {secret_number}");
        println!("Enter you guess:");
        let mut guess = String::new();
        println!("Your guess is :{guess}");
        io::stdin().read_line(&mut guess).expect("Failed!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Numbers Only!!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
