use std::io; // input and output for user
             // https://benjaminbrandt.com/converting-temperatures-in-rust/

enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}

fn convert_temperature(temp: &Temperature) -> f64 {
    match temp {
        &Temperature::Fahrenheit(degrees) => (degrees - 32.0) * (0.5556),
        &Temperature::Celsius(degrees) => (degrees * (1.8)) + 32.0,
    }
}

fn print_temp(temp: &Temperature) {
    match temp {
        &Temperature::Fahrenheit(degrees) => {
            println!("{}F = {}C", degrees, convert_temperature(temp))
        }
        &Temperature::Celsius(degrees) => println!("{}C = {}F", degrees, convert_temperature(temp)),
    }
}

fn get_user_temp() {
    println!("\nType \"quit\" to end the program");

    loop {
        let mut temp_input = String::new();
        println!("\nPlease input a temperature you want to convert (Format: 100F or -40C):");
        io::stdin()
            .read_line(&mut temp_input)
            .expect("Failed to read line");

        let trimmed = temp_input.trim();
        if trimmed == "quit" {
            break;
        }
        let (temp, scale) = trimmed.split_at(trimmed.len() - 1);
        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let temp: Temperature = match scale {
            "C" => Temperature::Celsius(temp),
            "F" => Temperature::Fahrenheit(temp),
            _ => continue,
        };
        print_temp(&temp);
    }
}

fn main() {
    get_user_temp();
}