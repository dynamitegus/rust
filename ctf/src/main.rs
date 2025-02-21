use std::io;

fn main() {

    println!("what temp");

    let mut c =String::new();

    io::stdin()
        .read_line(&mut c)
        .expect("failed to read line");

    let c: f32 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    
    };

    
    println!("{}", ctf(c));
}


fn ctf(x: f32) -> f32{
    //return (x * (9 / 5)) + 32;

    let a: f32 = 9.0 / 5.0;
    println!("{}", a);
    let b: f32 = x * a;
    println!("{}", b);
    return b + 32.0;
}