extern crate rand;

mod fib;
mod guess_game;
mod area;
mod maybe;
mod collection;
mod errors;
mod traits;
mod grep;
mod closures;
mod talk;
mod tree;
mod threads;
mod posts;

fn main() {
    // guess number :
    guess_game::guess_game();

    // fibonacci
    println!("15th fib is : {}", fib::fib(15));
}
