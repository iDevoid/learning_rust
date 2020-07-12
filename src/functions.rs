use std::io;

pub fn functions() {
    println!("see the code to get into the detail\n1. function with primitive params\n2. function with tuple");

    // you must be aware that the input is always string format.. not in array of characters
    let mut selected_type = String::new();   
    io::stdin()
    .read_line(&mut selected_type)
    .expect("please input the number of type correctly");
    match selected_type.trim() {
        "1" => {
            let (a, b, c, d) = function_with_primitive_params(-13, false, 'a', 3.14);
            println!("returned signed32 : {}\nreturned bool : {}\nreturned char : {}\nreturned float64 : {}", a, b, c, d);
        },
        "2" => {
            let ((a, b, c), d) = function_with_tuple_params((32, "something", "better".into()), 123);
            // or tuple into one variable 
            // let (a, b) = function_with_tuple_params((32, "something", "better".into()), 123);
            println!("returned tuple signed32 : {}\nreturned tuple str : {}\nreturned tuple String : {}\nreturned another payload : {}", a, b, c, d);
        },
        _ => println!("non selected"),
    };
}

// as you can see that this func has tuple as param and integer with it.. you can actually play the params to parse and return as you can see below
fn function_with_tuple_params(tuple_param: (u32, &str, String), id: u64) -> ((u32, &str, String), u64) {
    let (a, b, c) = tuple_param;
    ((a, b, c), id)
}

// a standard function that has param and return the primitive data type 
fn function_with_primitive_params(a: i32, b: bool, c: char, d: f64) -> (i32, bool, char, f64) {
    println!("parsed signed32 : {}\nparsed bool : {}\nparsed char : {}\nparsed float64 : {}", a, b, c, d);
    (a, b, c, d) // return values doesn't need semicolon
}