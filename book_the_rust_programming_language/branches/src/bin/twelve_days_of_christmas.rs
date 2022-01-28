fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let present = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let mut i = 0;
    while i < 12 {
        println!("On the {} day of Christmas", days[i]);
        println!("My true love sent to me");

        for j in (0..=i).rev() {
            println!("{}", present[j]);
        }

        println!();
        i += 1;
    }
}
