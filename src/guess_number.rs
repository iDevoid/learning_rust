use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_number() {
    println!("guess the number!");
    let rand_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input your guess.");
        let mut guess = String::new();

        // read input from terminal
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow variable change string type to unsigned integer 32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    };
}