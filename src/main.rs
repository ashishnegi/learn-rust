extern crate rand;

mod fib;
mod guess_game;
mod area;
mod maybe;
mod collection;
mod errors;
mod traits;
mod grep;

fn main() {
    // guess number :
    guess_game::guess_game();

    // fibonacci
    println!("15th fib is : {}", fib::fib(15));
}
