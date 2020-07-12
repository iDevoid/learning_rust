pub fn loop_and_if_statement() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 3 {
            break counter * 2; // this will break the loop and return the counter * 2 as the value of result
        } else if counter == 1 {
            println!("FIRST!");
        } else {
            println!("{}", counter); 
        }
    };

    let condition = true;
    let mut number = if condition { 3 } else { 5 };
    println!("the result : {} and the number : {}", result, number);

    while number > 0 {
        print!("{}! ", number);
        number -= 1;
    }
    print!("\n");

    let a = [1, 2, 3, 4];
    // enumerate func helps you to access both value and index from array
    for (i, val) in a.iter().enumerate() {
        println!("index {} val : {}", i, val);
    }

    // create a loop with (1, 2, 3) values but with reverse access
    for num in (1..4).rev() {
        print!("{}! ", num);
    }
}