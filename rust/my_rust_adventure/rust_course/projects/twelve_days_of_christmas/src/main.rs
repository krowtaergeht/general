fn twelve_days_of_christmas() {
    let mut day_index = 0;
    let mut index: usize = 10; // index for gift array, going backwards
    let day = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let first_gift = ["A partridge in a pear tree."];
    let gift = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And a partridge in a pear tree.",
    ];
    while day_index == 0 {
        // first verse different from others, seperate
        println!(
            "On the {} day of Christmas
        My true love gave to me
        {}",
            day[day_index], first_gift[0]
        );
        day_index = day_index + 1; // adding 1 to index to move to other loop
    } // the day index will always be adding by one for each loop
    while day_index < 12 {
        println!(
            "On the {} day of Christmas
        My true love gave to me
        {:?}",
            day[day_index],
            &gift[index..]
        );
        day_index = day_index + 1;
        if index > 0 {
            index = index - 1; /*
                               gift index must decline by 1 to include
                               other items
                               */
        }
    }
}

fn main() {
    twelve_days_of_christmas();
}