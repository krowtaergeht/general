fn main() {
    // if expression example
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn main() {
    // if expression error example, must be bool
    let number = 3;
    if number {
        println!("number was three");
    }
}

fn main() {
    // if expression example
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
}

fn main() {
    // handling multiple conditions of else if example
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn main() {
    // using if in a let statement example
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn main() {
    // using if in a let statement error example
    let condition = true;
    let number = if condition {
        5 // need to be same type, notice no semicolon
    } else {
        "six" // need to be same type, notice no semicolon
    };
    println!("The value of number is: {}", number);
}

fn main() {
    // repeating code loop example
    loop {
        println!("again");
    }
}

fn main() {
    // break and returning values from loops example
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }; // semicolon since end of a statement
    println!("The result is {}", result);
}

fn main() {
    // while loop example
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn main() {
    // while loop example
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
} // more efficient and safer to do a for loop

fn main() {
    // for loop example
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
} // for loops are much safer and are common amongst users

fn main() {
    // for loop example
    for number in (1..4).rev() {
        // rev method means reverse iterate
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}