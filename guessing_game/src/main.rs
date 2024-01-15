extern crate rand; // lets rust know we are using external(extern) dependency
// also call use rand


use std::io; // we have to bring in the input/output library
// std=standard library and io = input/output library
use std::cmp::Ordering;
use rand::Rng; // trait that defines method that rngs impelment
// Don't know what a trait is just yet

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    // This was just you for testting
    // println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();  //mut is for mutable so let mut guess is a mutable varibale assignment
        // String::new() is an empty new string type
        // new() method exist for most types

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); //expect come std::Result which has two types OK and Err
            // If Err is returnd after as a result from running stdin then expect will crash the program
            // and return the argument string
        
        // previously used for handing Err
        //let guess: u32 = matchguess.trim().parse()
        //    .expect("Please type a number!"); // we must convert string into  u32 so we clean up the value and then convert
            // expect deals with error if string cant be converted to number value

        //This repeats loop if a non number value is entered    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // The variats for Ordering and these 3 arms deal with those 3 different outcomes
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
} 