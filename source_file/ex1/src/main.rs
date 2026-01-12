//imported for random number generation
use rand::Rng;
//imported for checking if the guessed value is correct using the match function
use std::cmp::Ordering;
//standard library for getting input from the user
use std::io;

//The main function of the program, acts as an entry point
fn main() {
    println!("Welcome to Guess the Number Game!!");
    println!("==================================");

    //creates a random number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //an infinite loop
    loop {
        //creates a guess variable with an empty string
        let mut guess = String::new();

        println!("Enter Your Guess (1-100)");

        //gets input from the user using the imported io library and append it to the guess variable!
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't get the guess value");

        //converts the input into unsigned 32 bit integer to compare with secret_number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid number!");
                continue;
            }
        };

        println!("You Guessed {guess}");

        /*compares the guess witht the secret_number to check if the guess is right or wrong, the
         * match works like checking each of its arm for the result for a match */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The guess is low!"),
            Ordering::Greater => println!("The guess is high"),
            Ordering::Equal => {
                println!("You Guessed the right number!!!");
                println!("==================================");
                break;
            }
        }
    }
}

//created with the help of the rust doc
//12/january/26
//Sakthivel Durai S
