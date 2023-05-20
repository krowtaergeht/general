use std::collections::HashMap;

// formulas: https://byjus.com/mean-median-mode-formula/

fn mean(numbers: &Vec<i32>) -> f32 {
    // mean = sum of numbers / count or len
    let sum_of_numbers: i32 = numbers.iter().sum();
    // can use "as" to change data type, .len() will count
    sum_of_numbers as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    // must sort the values in the vector in ascending order
    numbers.sort();
    /*
    mid serves to find the middle index value,
    that is why we are trying to find its average
    */
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        /*
        we must take the average of the two middle numbers
        the indexing must take into account that the first index starts
        at zero. this is why we subtract 1 from the first index below.
        we must then turn the resulting value into an i32.
        */
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        /*
        when the .len() is odd, the index will round up to the middle number
        since it is returning as an i32, x.5 will get rounded up
        */
        numbers[mid]
    }
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    /*
    must make a hashmap to store the number of occurences of the numbers
    within the given vector
    */
    let mut map = HashMap::new();
    /*
    take the numbers from the vector and calculate their occurences or "counts"
    the resulting counts are then inputed into the hashmap
    the key is the integer, and the count is the value of the key
    must make sure each key is paired with a value
    */
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }
    /*
    calculating the mode by looking through the values in the hashmap
    must use clone to create a copy, and then use max to find out which
    has the highest occurences
    unsure what unwrap_or is used for
    */
    let max_value = map.values().cloned().max().unwrap_or(0);
    /*
    must iterate over the values of the hashmap and then filter the mode from
    the hashmap using the max value expression defined earlier
    from this filter, we map out the keys and thus create a new map
    collect is then used to gather the data to be presented in fn main
    and is returned as a vector
    */
    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}

fn main() {
    let mut some_integers: Vec<i32> = vec![1, 2, 4, 4, 5, 7, 7, 8, 10, 11];
    let average = mean(&some_integers);
    let middle = median(&mut some_integers);
    let most_common = mode(&some_integers);
    println!("The mean of the given numbers is: {}", average);
    println!("The median of the given numbers is: {}", middle);
    println!(
        "The mode(s) of the given numbers is(are): {:?}",
        most_common
    );
}