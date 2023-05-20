fn main() { // type annotation example
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess = "42".parse().expect("Not a number!");
}

fn main() { // floating-point types example
    let x = 2.0; // f64, default type no need to annotate
    let y: f32 = 3.0; // f32
}

fn main() { // numeric operations example
    addition
    let sum = 5 + 10;
    subtraction
    let difference = 95.5 - 4.3;
    multiplication
    let product = 4 * 30;
    division
    let quotient = 56.7 / 32.2;
    remainder
    let remainder = 43 % 5;
}

fn main () { // boolean type example
    let t = true;
    let f: bool = false; // with explicit type annotation
}

fn main() { /*
    character type example, note: characters use single quotes and
    strings use double quotes
    */
    let c = 'z';
    let z = 'e';
    let heart_eyed_cat = 'h';
}

fn main () { // tuple type example
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn main () { // using pattern matching to destructure a tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

fn main() { // tuple indexing example
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

fn main() { // array type example
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", 
    "June", "July", "August", "September", "October", "November", "December"];
    let a_2: [i32, 5] = [1, 2, 3, 4, 5]; // annotation
    let a_3 = [3; 5]; /*
    implied annotation, '3' implies integer type with an example number,
    '5' array length
    */
    let a_4 = [3, 3, 3, 3, 3]; // alternative way of a_3 = [3; 5]
}

fn main () { // accessing array elements example
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
}

use std::io;

fn main() { // invalid array element access example
    let a = [1 ,2 ,3 ,4 ,5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Not a number");
    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);
}