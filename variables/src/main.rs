fn main() {
    // let a: [f64; 4] = [32., 45., -10., 69.];

    // for x in a.iter() {
    //     println!("{}", farenheit_to_celsius(*x));
    // }

    // for n in 0..10 {
    //     println!("{}", fibonacci(n));
    // }
    tweleve_days();
}

// fn farenheit_to_celsius(x: f64) -> f64 {
//     (x - 32.) * (5. / 9.)
// }

// fn fibonacci(n: u32) -> u32 {
//     let mut a = 0;
//     let mut b = 1;

//     for _ in 0..n {
//         let old_a = a;
//         let old_b = b;

//         a = old_b;
//         b = old_a + old_b;
//     }

//     a
// }

fn tweleve_days() {
    const LINES: [&str; 11] = [
        "12 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a leaping",
        "Nine ladies dancing",
        "Eight maids a milking",
        "Seven swans a swimming",
        "Six geese a laying",
        "Five gold rings, badam-pam-pam",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
    ];
    const ORDINALS: [&str; 12] = [
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
        "twelfth"
    ];
    println!("On the first day of Christmas");
    println!("My true love gave to me");
    println!("A partridge in a pear tree");
    println!();
    for verse_number in (1..12).rev() {
        println!("On the {} day of Christmas", ORDINALS[12-verse_number]);
        println!("My true love gave to me");
        for line_number in verse_number-1..11 {
            println!("{}", LINES[line_number]);
        }
        println!("And a partridge in a pear tree");
        println!();
    }
}
