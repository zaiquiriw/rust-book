// In this chapter we learn about variables, types, functions, comments, if EXPRESSIONS, and loops.
// This binary will allow the user to run the summary's suggested problems:
// - Convert temperatures between Fahrenheit and Celsius
// - Generate the nth Fibonacci number
// - Print the lyrics to the Christmas carol "The Tweleve Days of Christmas" taking advantage
//   of the repitition in the song.

// This bin also acts as another test of taking in user input, since the user can select the
// exercise to print.

// References: Chapters 1-3 of the book and some quick googles
// - (F-32)5/9 = C and  C9/5+32 = F
// - The Fibonacci sequence, starting with 0 and 1, is recurrently defined, F_n = F_{n-1} + F_{n-2}
// - Taking the lyrics straight of genius lyrics when written.

use std::io;

fn fib_calc(before: u32, fib: u32, index: u32, count: u32) -> u32 {
    if index == count {
        println!("{fib}");
        fib
    } else {
        fib_calc(fib, fib + before, index, count + 1)
    }
}

fn main() {
    loop {
        println!("Chapter 3 Exercises: Pick which one to run:");
        println!("[1]. Temperature Conversion");
        println!("[2]. Fibonacci Numbers");
        println!("[3]. The Tweleve Days of Christmas");
        println!("[4]. Quit");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        println!("You selected: {selection}");

        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut option = String::new();

        // As a challenge to only use what has been taught before the 3rd chapter, no using fancy functions or control flow haha...
        match selection {
            1 => {
                println!(
                    "You chose Temperature Conversion, Are you starting from F->C or C->F? [f|c|q]"
                );

                io::stdin()
                    .read_line(&mut option)
                    .expect("Failed to read line");

                let option: char = match option.trim().parse() {
                    Ok(option) => option,
                    Err(_) => continue,
                };
                match option {
                    'f' => {
                        println!("Please enter a temperature in F to convert to C [float|q]");

                        let mut num = String::new();

                        io::stdin()
                            .read_line(&mut num)
                            .expect("Failed to read line");

                        let num: f32 = match num.trim().parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        };

                        let result: f32 = (num - 32.0) * (5.0 / 9.0);
                        println!("{num} Farenheit is {result} in Celsius.");
                    }
                    'c' => {
                        println!("Please enter a temperature in C to convert to F [float|q]");

                        let mut num = String::new();

                        io::stdin()
                            .read_line(&mut num)
                            .expect("Failed to read line");

                        let num: f32 = match num.trim().parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        };

                        let result: f32 = (num * 9.0 / 5.0) + 32.0;
                        println!("{num} Celsius is {result} in Farenheit.");
                    }
                    'q' => break,
                    _ => continue,
                }
            }
            2 => {
                // Now I have to use the concepts described in Chapter 3 since this section
                println!("You chose Fibonacci Numbers, go ahead and enter a number! [+int]");

                io::stdin()
                    .read_line(&mut option)
                    .expect("Failed to read line");

                let option: u32 = match option.trim().parse() {
                    Ok(option) => option,
                    Err(_) => continue,
                };

                let fibefore = 0;
                let count = 1;
                let fibby_number = 1;

                println!("Calculating the fibonacci number at this index: {option}");

                // I'm going to use the recursive function solution for this
                let result = fib_calc(fibefore, fibby_number, option, count);
                println!("The Fibonacci number at {option} is {result}");

                break;
            }

            3 => {
                println!("You chose The Tweleve Day of Christmas, are you ready? [yes|no|q]");

                io::stdin()
                    .read_line(&mut option)
                    .expect("Failed to read line");

                println!("You are never ready for Christmas music...");
                println!("");

                let lyrics = [
                    "A partridge in a pear tree",
                    "Two turtle doves and",
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

                let numbers = [
                    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
                    "ninth", "tenth", "eleventh", "twelfth",
                ];

                // When using Range, the first number is included and the last excluded, the first
                // run of the below loops would have verse 0, line 0, and end with verse 11, line 11
                for verse in 0..12 {
                    let day = numbers[verse];
                    println!("On the {day} day of Christmas my true love gave to me:");
                    for line in 0..verse + 1 {
                        let new_line = lyrics[line];
                        println!("{new_line}");
                    }
                    println!("");
                }
            }
            4 => {
                break;
            }
            _ => println!("Try entering just a number for me."),
        };
    }
}
