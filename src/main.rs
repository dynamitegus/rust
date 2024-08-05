fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number was divisible by 4");
    } else if number % 3 == 0 {
        println!("number was divisible by 3");
    } else if number % 2 == 0 {
        println!("you get the point its divisible by 2");
    } else {
        println!("number is not divisible by 4 3 or 2");
    }
}
