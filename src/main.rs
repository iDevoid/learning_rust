use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("want to start with rust : https://www.rust-lang.org/learn/get-started");
    println!("Pick one menu!");
    println!("1. guessing game\n2. shadowing\n3. length of input\n4. data types");
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
        "1" => guessing_number(),
        "2" => shadowing_example(),
        "3" => get_the_length_input(),
        "4" => data_types(),

        // this is important to have in match without specifict operation / comparation function as the default return when there's no matches value
        _ => println!("something else!"),
    };
}

fn data_types() {
    println!("source : https://doc.rust-lang.org/book/ch03-02-data-types.html \n1. integer\n2. float\n3. boolean\n4. character");
    println!("input the type");

    // you must be aware that the input is always string format.. not in array of characters
    let mut selected_type = String::new();   
    io::stdin()
    .read_line(&mut selected_type)
    .expect("please input the number of type correctly");
    match selected_type.trim() {
        "1" => integer_data_types(),
        "2" => float_data_types(),
        "3" => bool_data_types(),
        "4" => char_data_types(),
        "5" => tuple_data_types(),
        "6" => array_data_types(),
        _ => println!("non selected"),
    };
}

fn array_data_types() {
    // array declaration without annotation, i think this will only confuse you since it's not declare with type..
    // meaning you have to see the declared value
    // actually you can hover the variable name and see the type on IDE, not on github, etc
    let a = [1, 2, 3, 4];
    let random_strings = ["asddcs", "ewr ddsf", "xcojao"];

    // [data type; length]
    let better_with_annotation : [i32; 5] = [1, 2, 3, 4, -5];

    // declare an array with specific length and default value
    // this array will have ["default", "default", "default"]
    let default_array = ["default"; 3];

    // accessing the array
    println!("without annotation\narray of numbers at index 0 : {}\narray of random strings at index 1 : {}", a[0], random_strings[1]);
    println!("with annotation at index 4 : {}", better_with_annotation[4]);
    println!("with default value at index 2 : {}", default_array[2]);

    println!("you cannot access declared array with out of the index pointing since the compailer will write down it as an error, see the code and un-comment");
    // try to un-comment this to access out of index array
    // println!("cannot : {}", random_strings[10]);
}

fn tuple_data_types() {
    // once declared, they cannot grow or shrink in size.. meaning it's not a mutable data type
    let tup_annotation : (i32, f32, u8) = (-124, 4.34, 8);
    let tup_without_annotation = (1231, "something", true);

    // how to access the value inside the tuple?
    let (x, y, z) = tup_annotation; // declare like this
    println!("tuple annotation,\nx : {}, y : {}, z : {}", x, y, z);
    // or you can access directly like this
    println!("access direct to the tuple :\ntuple.0 : {}, tuple.1: {}, tuple.2: {}", tup_without_annotation.0, tup_without_annotation.1, tup_without_annotation.2);

}

fn char_data_types() {
    // remember the difference between string with "" and char with ''
    // this is the same as usual
    let normal_character = 'c';
    println!("normal character : {}", normal_character);

    // you can actually store the symbolic character like below
    let symbolic_character = 'ℤ';
    println!("symbolic character: {}", symbolic_character);

    // and of course you can also store the emoticon like this
    let emoticon = '😻';
    println!("dude, it's emoticon : {}", emoticon);
}

fn bool_data_types() {
    // simple as usual
    let t = true; // declaration without annotation
    let f : bool = false; //declaration with annotation
    println!("without annotation {} and with annotation {} , see the code", t, f);
}

fn float_data_types() {
    // string to float 32
    let string_to_float32 : f32 = "2131.44534".parse().unwrap();
    println!("on code string to float 32 be like : {}", string_to_float32);


    println!("input a float number");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("please input the correct number");
    let input = input.trim();

    // string to float 64
    let string_to_float64 : f64 = input.parse().expect("please input the correct number");
    println!("input {} convert to float64 : {}", input, string_to_float64);
}

fn integer_data_types() {
    // use unwrap when the value is something no inputted from user
    let string_to_int8: u8 = "8".parse().unwrap();
    println!("on code string to int 8 will be like : {}", string_to_int8);

    println!("input a number");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read");

    // shadowing variable on string
    let input = input.trim();

    // use .expect whenever you want to parse the input from user. so it will handle the input whether it's correct or not
    // i think rust is the only programming language that has integer 128 even golang doesn't have AFAIK
    let input_to_int128 : u128 = input.parse().expect("please input the number correctly");
    println!("input {} to int128 : {}", input, input_to_int128);

    // signed integer is ONLY for positive number
    let signed_integer_32 : u32 = 124;
    println!("signed integer 32 : {}", signed_integer_32);

    // support both negative and positive number
    let unsigned_integer_32_negative : i32 = -12736;
    println!("unsigned integer 32 : {}", unsigned_integer_32_negative);
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

fn guessing_number() {
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