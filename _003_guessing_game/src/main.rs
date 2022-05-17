// sometimes we need packages to run our programs, we can use "use" with "library::module" to get the package imported
use std::io;

fn main() {
    println!("Guess my Numba!");
    
    println!("Input your guess.");
    
    // let keyword inits new var; the ::new is a method, which creates the string
    // if you want a variable to be changable you need to add the mut keyword after let
    let mut guess: String = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
        
    println!("You guessed: {}", guess);
}
