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

use std::io;

fn main() {
    println!("Chapter 3 Exercises: Pick which one to run:");
    println!("[1]. Temperature Conversion");
    println!("[2]. Fibonacci Numbers");
    println!("[3]. The Tweleve Days of Christmas");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    println!("You selected: {selection}");
}
