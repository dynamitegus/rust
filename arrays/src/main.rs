use std::io;
// This code panics lol
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("please enter and array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("the value of the element and the index {index} is: {element}");
    

}
