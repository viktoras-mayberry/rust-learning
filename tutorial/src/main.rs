fn main() {
    let x = 10;
    println!("x is: {}", x);

    {
        let x = 2;
        println!("x is: {}", x);
    }

    // incrementing the value of x by 1.
    let x = x + 1;
    println!("x is: {}", x);

}
