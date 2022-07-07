fn main() {
    let days: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let mut verses: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12  {
        println!("On the {} day of Christmas, my true love sent to me", days[i]);
        let slice = &mut verses[0..i+1];
        for j in slice.iter().rev() {
            println!("{}", j);
        }
        println!("\n");
    }
}
