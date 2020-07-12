
pub fn ownership() {
    let s1 = String::from("hello");
    
    // take ownership
    // let s2 = s1;
    // println!("{}, world!", s1);
    // you can't access s1 variable because it's already transfered to s2 and s1 goes out of scope
    
    // take the value and register to new allocation memory
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // primitive values by default make a copy rather than taking the ownership of the memory
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let mut s = String::from("hello");
    append_string(&mut s);

    let got_referenced = no_dangle();
    println!("{}", got_referenced);
}

// this takes the param as reference and can be changed without making a return
// this can only be done if the param is mutable 
fn append_string(some_string: &mut String) {
    some_string.push_str(", world");
}

// this function creates a copy of s as the return
// but since the s is created inside the no_dungle function
// this will deallocated s inside the no_dungle function
// so you have to copy it instead of accessing invalid pointer or cleaned memory
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
