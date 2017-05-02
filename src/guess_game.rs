use std::io;
use rand;
use rand::Rng;
use std::cmp::Ordering;

pub fn guess_game() {
    println!("Guess number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("You guessed {} ..", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("You guessed it..!!!");
                break;
            }
            Ordering::Greater => println!("Too high..!!!")
        }
    }
}
