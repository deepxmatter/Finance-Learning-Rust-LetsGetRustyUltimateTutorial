// sometimes we need packages to run our programs, we can use "use" with "library::module" to get the package imported
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess my Numba between 1 and 100!");
    
    // this line is producing a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("The secret number is {}", secret_number);
    
    loop {
        println!("Input your guess.");
        
        // let keyword inits new var; the ::new is a method, which creates the string
        // if you want a variable to be changable you need to add the mut keyword after let
        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
            
        
        // reassigning guess to a number instead of a string
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "ERRRRRROR: make sure you put a number between 1 and 100".red());
                continue;
            }
        };
            
        // this is like an interpolated string
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "you won king!".bright_blue().bold());
                // break works just like any other language
                break;
            }
        }
    }
}