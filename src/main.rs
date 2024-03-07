use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io;

fn main() {
    //TODO
    // - [ ] build logic to generate password
    // - [ ] adequate testing of generated passwords

    //setup
    let mut input: String = String::new();
    let mut password: String = String::new();
    let mut index = 0;

    //bellow code generates the starting char of the password at random.
    let first_char: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1)
        .map(char::from)
        .collect();
    //huge mess but this the only method that worked so far.

    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read line");

    println!("{}", &first_char);

    let previous_char: String = first_char;
    password.push_str(&previous_char);
    while index < input.trim().parse().unwrap() {
        index += 1;
    }
}
