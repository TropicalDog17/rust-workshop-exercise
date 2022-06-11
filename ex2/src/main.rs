use regex::Regex;
use std::fs;
use std::io;
fn main() {
    let filename = "text.txt";
    let mut count = 0;
    let mut word = String::new();
    println!("Please provide your search term: ");
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    let word: &str = word.trim(); //Remove new line character;

    let pattern: String = r"(?i)\b".to_string() + word + &r"\b".to_string();
    let re = Regex::new(&pattern.to_owned()).unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for s in contents.split_whitespace() {
        if re.is_match(s) {
            count = count + 1;
        }
    }

    println!("There are {} of \"{}\" in {}", count, word, filename);
}
