//imported for getting input from user
use std::io;

fn main() {
    println!("============================================");
    println!("++++++++++ Temperature Converter +++++++++++");
    //this loops makes so that the program doesn't close automatically
    loop {
        println!("============================================");

        println!("Chose Options to perform.");
        println!("1 -> for Fahrenheit to Celsius. \n2 -> for Celsius to Fahrenheit. \n3 -> To Exit the program.");
        println!("=============== Your Option ================");

        loop {
            //initilaizing an empty String
            let mut option = String::new();
            //Getting input from the user
            io::stdin()
                .read_line(&mut option)
                .expect("Error while getting input");
            //trimming the next line character from the input
            let option = option.trim();
            //Changeing control flow based on the user option
            if option == "1" {
                fahrenheit_to_celsius();
                break;
            } else if option == "2" {
                celsius_to_fahrenheit();
                break;
            } else if option == "3" {
                println!("Exiting the program...");
                return;
            } else {
                println!("Invalid input");
                println!("Choose Either 1, 2 or 3");
            }
        }
    }
}
//Function to convert fahrenheit_to_celsius
fn fahrenheit_to_celsius() {
    println!("============================================");
    loop {
        println!("Enter the Temperature Value for Conversion:");
        //gets input from the user and validates it
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Error while getting input");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input, please enter a valid temp");
                continue;
            }
        };
        //formula to convert fahrenheit_to_celsius
        let result = (temp - 32.0) * 5.0 / 9.0;
        println!("the Celsius value of {temp}°F is {result:.2}°C");
        break;
    }
}

//Function to convert celsius_to_fahrenheit
fn celsius_to_fahrenheit() {
    println!("============================================");
    loop {
        println!("Enter the Temperature Value for Conversion:");
        //gets input from the use and validates it
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Error while getting input");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input, please enter a valid temp");
                continue;
            }
        };
        //formula to convert celsius_to_fahrenheit
        let result = (temp * 9.0 / 5.0) + 32.0;
        println!("The Fahrenheit value of {temp}°C is {result:.2}°F");
        break;
    }
}

//January 13/26
//Sakthivel Durai S
