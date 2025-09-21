use rand::Rng;
use std::cmp::Ordering;
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

    //gen_range is a method that generates a random number between 1 and 100
    //..= is used to create a range
    //1..=100 is a range from 1 to 100 inclusive of 1 and 100
    // Rng is a trait that must be in scope to use the gen_range method
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // rust lets us create a variable from the value of another variable
    // like we can create a variable with same name as another
    // this is called shadowing

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    }
}
