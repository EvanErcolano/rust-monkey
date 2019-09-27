// will check for lexer.rs or check whether theres a dir called lexer
mod lexer;
mod token;

use std::fs;
use lexer::Lexer;

fn main() {
    println!("Lexer starting up....");
    println!(
        "Input {:?}",
        fs::read_to_string("test_file.mky")
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
    );
    let lexer = Lexer::new(&String::from("test_file.mky")).unwrap();
    for token in lexer {
        println!("{:?}", token);
    }
}
