extern crate rand;

mod fib;
mod guess_game;

fn main() {
    // guess number :
    guess_game::guess_game();

    // fibonacci
    println!("15th fib is : {}", fib::fib(15));
}
