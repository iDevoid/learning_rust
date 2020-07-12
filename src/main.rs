mod ownership;
mod datatype;
mod loop_and_if;
mod functions;
mod guess_number;
mod hash_vs_vector;

use std::io;
use ownership::ownership; // after importing from other file, you can actually "use" it for less syntax

fn main() {
    println!("want to start with rust : https://www.rust-lang.org/learn/get-started");
    println!("Pick one menu!");
    println!("1. guessing game\n2. shadowing\n3. length of input\n4. data types\n5. functions\n6. loop and if\n7. ownership\n8. hash vs vector on find string performance");
    // this works too!
    // println!("1. guessing game
    //  2. something
    // ")

    println!("enter the number");
    let mut guess = String::new();
    // read input from terminal, i don't know why rust has only new line reader on terminal
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // .trim() remove \n when input the value
    match guess.trim() {
        "1" => guess_number::guessing_number(),
        "2" => shadowing_example(),
        "3" => get_the_length_input(),
        "4" => datatype::data_types(),
        "5" => functions::functions(),
        "6" => loop_and_if::loop_and_if_statement(),
        "7" => ownership(),
        "8" => hash_vs_vector::hash_vs_vector_on_strings(),

        // this is important to have in match without specifict operation / comparation function as the default return when there's no matches value
        _ => println!("something else!"),
    };
}


fn get_the_length_input() {
    println!("input something");
    let mut input = String::new();
    // read input from terminal
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    println!("input length without trim : {}", input.len());
    println!("input length with trim or remove new line input or \\n from input : {}", input.trim().len());
}

fn shadowing_example() {
    let x = 5;
    println!("the value of x at declaration is: {}", x);
    
    let x = x + 1; // this takes 5 and + 1 = 6
    println!("the value of x at addition is: {}", x);

    let x = x * 2; // this takes 6 and multiply it with 2 ==>> 6 * 2 = 12
    println!("the value of x at multiplication is: {}", x);

    let x = x - 2; // this takes 12 and substract it with 2 ==>> 12 - 2 = 10
    println!("the value of x at substraction is: {}", x);

    let x = x / 5; // this takes 10 and divide with 5 ==>> 10 / 5 = 2
    println!("the value of x at division is: {}", x);

    let x = x % 2; // this takes 2 and mod it with 2 => 2 % 2 = 0
    println!("the value of x at remainder is: {}", x);
}
