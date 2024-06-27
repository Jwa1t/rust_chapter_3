fn main() {
    println!("Converting temperature examples:");
    convert_temperature(74.0, 'F');
    convert_temperature(52.0, 'f');
    convert_temperature(25.0, 'C');
    convert_temperature(31.0, 'c');
    convert_temperature(99.0, 'Z');

    println!("Printing Fibonacci sequence");
    let n = 20;
    let result = print_fibonacci(n);
    println!("The {n} number is: {result}");

    println!("Printing The Twelve Days of Christmas Lyrics\n");
    print_twelve_days_of_christmas();
}

// This function takes in a temp and its unit and converts to the opposite (F -> C or C -> F)
fn convert_temperature(temp: f32, unit: char) {
    if unit == 'C' || unit == 'c' {
        println!("{temp}{unit} converted to Fahrenheit is {}F", (temp * (9.0/5.0)) + 32.0);
    } else if unit == 'F' || unit == 'f' {
        println!("{temp}{unit} converted to Celcius is {}C", (temp - 32.0) * (5.0/9.0));
    } else {
        println!("Input a proper unit type (F or C)")
    }
}

// This function prints the Fibonacci sequence and returns the nth Fibonacci number
fn print_fibonacci(n: i32) -> i32 {
    let mut result = 0;

    println!("F0: 0");

    if n > 0 {
        println!("F1: 1");
        let mut first = 0;
        let mut second = 1;
        result = 1;
        for x in 2..n {
            result = first + second;
            first = second;
            second = result;
            println!("F{x}: {result}");
        }
    }

    return result;
}

fn print_twelve_days_of_christmas() {
    let true_love = "my true love gave to me";
    let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts: [&str; 12] = ["partridge in a pear tree.", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", 
        "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", 
        "Twelve drummers drumming,"];

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("{true_love}");
        
        for count_down in (1..day+1).rev() {
            println!("{}", gifts[count_down]);
        }

        if day == 0 {
            println!("A {}\n", gifts[0]);
        } else {
            println!("And a {}\n", gifts[0]);
        }
    }
}
