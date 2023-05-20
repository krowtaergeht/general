use std::io; // input and output for user

fn convert(degrees: f64, scale: char) -> f64 {
    // degrees and scale: f or c

    if scale == 'f' {
        // if I enter degrees in fahrenheit
        let result: f64 = (degrees - 32.0) * (0.5556);
        return result;
    } else {
        //if scale == 'c'//
        let result: f64 = (degrees * (1.8)) + 32.0;
        return result;
    }
}

fn main() {
    println!("Enter the number of degrees here.");
    /*
    input and output here, read line, removing whitespace and
    converting to f64 from a string
    */
    let mut degrees = String::new();
    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line.");
    let degrees: f64 = degrees.trim().parse().expect("Please type a number!");
    println!("Enter the scale: 'f' for farenheit or 'c' for celsius here.");
    /*
    input and output here, read line, removing whitespace and
    converting to f64 from a string
    */
    let mut scale = String::new();
    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line.");
    let scale: char = scale
        .trim()
        .parse()
        .expect("Please type 'f' for fahrenheit or 'c' for celsius!");
    // linking user inputs to the function parameters
    let x = convert(degrees, scale);
    if scale == 'f' {
        println!("The converted temperature is {} degrees celsius.", x);
    } else {
        println!("The converted temperature is {} degrees fahrenheit.", x);
    }
}