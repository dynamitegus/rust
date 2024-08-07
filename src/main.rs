fn main() {
    let c = 0;
    
    println!("{}", ctf(c));
}


fn ctf(x: i32) -> i32{
    return x * (9/5) + 32;
}