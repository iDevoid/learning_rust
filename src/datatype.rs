use std::io;

pub fn data_types() {
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