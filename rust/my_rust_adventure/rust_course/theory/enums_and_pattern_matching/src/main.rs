fn main() { // defining an enum and its values example
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4; /*
    both are of the same 
    IpAddrKind type
    */
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) { }

fn main() { /*
    storing the data and IpAddrKind variant
    of an IP address using a struct
    */
    enum IpAddrKind { // define enum
        V4,
        V6,
    }

    struct IpAddr { // define struct
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr { // instantiate struct
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr { // instantiate struct
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn main() { // alternative method of above code block
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

fn main() { // enum with different data types
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

use std::net::IpAddr; // standard library example

fn main() {
    struct Ipv4Addr { // standard library can store data in structs
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

}

fn main() { /*
    enum whose variants each store different amounts
    and types of values
    */
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}

fn main() { // alternative method to above code block using structs
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x:i32,
        y:i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
} /*
enums are seen as one type with multiple variants
structs are only seen as one type
*/

impl Message { // producing methods on enums
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message:Write(String::from("hello"));
m.call()

fn main() { // the option enum and its advantages over null values
    enum Option<T> {
        Some(T),
        None,
    }
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

fn main() { // following code will not work
    let x: i8 = 5; // i8 is not the same data type as Option<i8>
    let y: Option<i8> = Some(5);
    let sum = x + y; /*
    cannot add two 
    different data types
    */
} /*
to get the T value out of a Some variant when you have a value of type
Option<T> is made possible with the match expression
*/

fn main() { // the match control flow operator
    enum Coin { /*
        an enum and a match expression that has the variants
        of the enum as its patterns
        */
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents(coin: Coin) -> u8 { /*
    this function includes a print macro
    while also returning the value in cents
    */
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

use std::fmt::Debug; // patterns that bind to values

fn main() { 
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

}

fn value_in_cents(coin, Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() { // matching with option<t>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() { // matches are exhaustive
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1), /*
            code will not run, it is exhaustive
            and needs all possible scenarios. none needs to be included
            */
        }
    }
}

fn main() { // the _ placeholder
    let some_u8_value = 0u8;
    match some_u8_value { /*
        this expression will match the values needed without
        having to list all possible values
        */
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), /*
        matches all possible cases that are not specified before it
        () instructs to return nothing
        */
    }
}

fn main() { // concise control flow with if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value { /*
        alternative to previous code, 
        but lose exhaustive checking
        */
        println!("three");
    } /*
    if let is like match that runs code when the value matches one pattern
    and then ignores all other values. can also include else with an if let and
    would act like the _ case in the match expression
    */
}

fn main() { // matching state quarters and counting non-quarter coins
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count+= 1;
    }

    let mut count2 = 0; // alternative to above code
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}