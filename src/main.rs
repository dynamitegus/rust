use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Geuss the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
    println!("please input your geuss");

    let mut guess =String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    
    };


    println!("you geussed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {println!("you win"); break;}
    };
}
}
