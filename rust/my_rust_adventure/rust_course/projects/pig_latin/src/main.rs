use std::io;

fn main() {
    println!("Please enter the word to be translated into Pig Latin.");
    // make user input empty string
    let mut input = String::new();
    /*
    creating a loop that reads the user input, with expect error handling
    the input is then trimmed of whitespaced and turned into a string
    the pig latin function is called on the input and then printed
    the input is then erased for future use
    */
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        piglatinize(&mut input);
        println!("{}", input);
        input.clear();
    }
}

fn piglatinize(s: &mut String) {
    /*
    clone the string and then remove the initial letter of the word
    must clone in order to avoid missing the first letter if word starts
    with a vowel
    */
    let initial_letter = s.clone().remove(0);
    /*
    create an if condition on the initial letter if a consonant (! means not)
    */
    if !is_vowel(initial_letter) {
        // must remove first letter if starts with a consonant
        s.remove(0);
        s.push_str("-");
        s.push(initial_letter);
        s.push_str("ay");
    // else initial letter is a vowel push -hay
    } else {
        s.push_str("-hay");
    }
}

// defining if vowel function
fn is_vowel(c: char) -> bool {
    c == 'a'
        || c == 'e'
        || c == 'i'
        || c == 'o'
        || c == 'u'
        || c == 'A'
        || c == 'E'
        || c == 'I'
        || c == 'O'
        || c == 'U'
}