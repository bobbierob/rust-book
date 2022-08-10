fn main() {
    println!("100 F in C is {}", f_to_c(100.0, false));
    println!("15th fibonacci number is {}", fibonacci(15));
    println!("");
    christmas();
}

fn f_to_c(input: f64, reverse: bool) -> f64 {
    if reverse != true {
        (input - 32.0) * 0.5556
    } else {
        (input * 1.8) + 32.0
    }
}

fn fibonacci(n: i32) -> i32 {
    let mut prev_num = 0;
    let mut curr_num = 1;
    if n == 1 {
        prev_num
    } else if n == 2 {
        curr_num
    } else {
        let mut count = 2;
        while count < n {
            let next_num = curr_num + prev_num;
            prev_num = curr_num;
            curr_num = next_num;
            count = count + 1;
        }
        curr_num
    }
}

fn christmas() {
    let mut lines = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let number_words = ["first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for verse in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", number_words[verse]);
        for line in (0..verse+1).rev() {
            println!("{}", lines[line]);
            if verse == 0 {
                lines[0] = "And a partridge in a pear tree"
            }
        }
        println!("");
    }
}
