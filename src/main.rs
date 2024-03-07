use rand::Rng;
use std::io;

fn main() {
    //setting up parameters
    let mut i = 1;
    let mut input: String = String::new();
    let mut pass_string: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let int_input = input.trim().parse().unwrap();

    while i <= int_input {
        print!("{}", i);
        //code idea: have an empty string, loop over characters and see what the previous character was.
        //i.e a symbol, number, abd...
        //depending on the previous character it will generate a different character and go to the next iteration.

        i += 1;
    }
}
