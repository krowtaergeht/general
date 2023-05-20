fn main() { // storing lists of values with vectors
    let v: Vec<i32> = Vec::new(); // creating a new vector
    let v = vec![1, 2, 3]; // vector macro
}

fn main() { // updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn main() { // dropping a vector drops its elements
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // <- v goes out of scope and is freed here

fn main() { // reading elements of vectors
    let v = vec![1 ,2 ,3 ,4 , 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn main() { // cannot index outside vector range
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[100]; // this will panic
    let does_not_exist = v.get(100); /*
    this will return
    none without panic
    */
}

fn main() { /*
    attemping to add an element to a vector while holding a reference
    to an item
    */
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
    v.push(6); // mutable borrow occurs here
    println!("The first element is: {}", first); /*
    immutable borrow
    later used here
    */
} // code will result in an error

fn main() { // iterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v { /*
        printing each element in a vector by iterating
        over the elements using a for loop
        */
        println!("{}", i);
    }
}

fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v { /*
        iterating over mutable references to
        elements in a vector
        */
        *i += 50; /*
        to change the value that the mutable reference refers to,
        we have to use the dereference operator (*) to get to the value in
        i before we can use the += operator
        */
    }
}

fn main() { // using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]; // defining an enum to store values of different types in one vector
}

fn main() { // storing UTF-8 encoded text with strings
    let mut s = String::new(); // creating a new string
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal string directly:
    let s = "initial contents".to_string();
} // using the to_string method to create a string from a string literal

fn main() { // alternative method to above code
    let s = String::from("initial contents");
} // using the string::from function to create a string from a string literal

fn main() { // updating a string
    let mut s = String::from("foo"); /*
    appending a string slice to
    a string using the push_str method
    */
    s.push_str("bar"); 
    let mut s1 = String::from("foo"); /*
    using a string slice after
    appending its contents to a string
    */
    let s2 = "bar";
    s1.push_str(s2); /*
    push_str does not take ownership of s2,
    s2 can be used again
    */
    println!("s2 is {}", s2);
    let mut s = String::from("lo"); /*
    adding one character to a
    string value using push
    */
    s.push('l');
}

fn main() { // concatenation with the + operator or the format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; /*
    note s1 has been moved here and
    can no longer be used
    */
    // must use reference to s2 due to the signature of the method:
    // fn add(self, s: &str) -> String {}
    // although s2 is &String, compiler coerces &String to &str
    // cannot add two string types together
    // can only add a &str to a string
}

fn main() { // concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3); // alternative way to above

}

fn main() { // indexing into strings
    let s1 = String::from("hello"); // this will result in error
    let h = s1[0];
    // rust strings do not support indexing
    let len = String::from("Hola").len(); /*
    string a wrapper over
    a Vec<u8>
    */
} /*
rust does not index strings because some characters take more than 1 byte
of storage and this would conflict with the index a user would be attempting
to get
*/

fn main() { // slicing strings
    let hello = "examplestringliteral";
    let s = &hello[0..4];
    for c in hello.chars() { // methods for iterating over strings
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap; /*
storing keys with associated values
in hash maps
*/

fn main() {
    let mut scores = HashMap::new(); // create hashmap
    scores.insert(String::from("Blue"), 10); // add keys and values
    scores.insert(String::from("Yellow"), 50); // must be same type
    /*
    combining two vectors into a vector of tuples, 
    then turning that into a hash map
    */
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];
    let scores: HashMap<_, _> = teams.iter()
    .zip(initial_scores
        .iter())
        .collect();
}

use std::collections::HashMap; // hash maps and ownership

fn main() { 
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    /*
    field_name and field_value are invalid at this point,
    try using them and see what compiler error you get!
    */
}

use std::collections::HashMap; // accessing values in a hash map

fn main() { 
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // iterating over each key/value pair
    for (key, value) in &scores { 
        println!("{}: {}", key, value); 
    }
}

use std::collections::HashMap; // updating a hash map

fn main() { // overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // only inserting a value if the key has no value
    let mut scores = HashMap::new(); 
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_entry(50);
    println!("{:?}", scores);
    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word)or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}