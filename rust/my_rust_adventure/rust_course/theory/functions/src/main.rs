fn main() { // function example
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}

fn main() { // function parameters example
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn main() { // statement example
    let y = 6;
}

fn main() { // statement error example
    let x = (let y = 6);
}

fn main () { // expression example
    let x = 5;
    let y = {
        let x = 3;
        x + 1 /*
        no semicolon at end due to it being an expression, 
        this returns a value. Adding a semicolon turns it into a statement 
        */
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 { // functions with return values example
    5 // note there is no semicolon
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}

fn main() { // functions with return values example
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x :i32) -> i32 {
    x + 1 // note there is no semicolon
}