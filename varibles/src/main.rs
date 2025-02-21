fn main() {
    println!("Hello, world!");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x in the small scope is {x}");
    }
    println!("x ounstide the scope is {x}");
}
      