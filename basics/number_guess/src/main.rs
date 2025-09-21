use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    //we use let to create a variable, and mut to make it mutable
    //String::new() creates a new empty string
    // :: is used to call a method on a type basically it helps to associate the method new with the String type
    let mut guess = String::new();
    
   //read_line takes whatever the user types into standard input and appends that into a string
   //& is used to reference the variable guess
   //read_line returns a Result value, and Result is an enum that has the variants Ok and Err
   //expect is used to handle the Result value, if the value is Err, the program will crash and display the string "Failed to read line"
   //if the value is Ok, the program will continue and display the string "You guessed: {guess}"
   //if we dont use expect the program will compile but will throw a warning
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
