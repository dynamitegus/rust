use std::io;


fn fiban(x: String) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = 0;

    let n = x.trim().parse::<u128>().unwrap();
if n == a {
    return a;
} else if n == b {
    return b;
}else {
    for  _i in 2..n+1 {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}
}



fn main() {
//define F0 and F1

//let iterationcmdedition = std::env::args().nth(1).expect();

//if iterationcmdedition != "" {
  //  fiban(iterationcmdedition);
//} 



println!("Please input the iteration of the fibonacci sequence.");

let mut iteration: String = String::new();

io::stdin()
    .read_line(&mut iteration)
    .expect("Failed to read line");


    println!("{}", fiban(iteration));






}
