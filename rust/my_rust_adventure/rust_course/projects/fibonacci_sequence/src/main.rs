use std::io;

fn fibonacci_sequence(n: f64) -> f64 {
    // following code uses the universal formula for the fibonacci sequence
    let phi_big = (1.0 + (5.0_f64.sqrt())) / 2.0;
    let phi_small = (1.0 - (5.0_f64.sqrt())) / 2.0;
    let result = ((phi_big).powf(n) - (phi_small).powf(n)) / (5.0_f64.sqrt());
    return result;
}

fn main() {
    println!("Please enter the nth fibonacci number");
    /*
    input and output here, read line, removing whitespace and
    converting to f64 from a string
    */
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    let n: f64 = n.trim().parse().expect("Please type a number!");
    // linking user input to the function parameter
    let x: f64 = fibonacci_sequence(n);
    println!("When n = {}, the resulting fibonacci number is: {}", n, x);
}