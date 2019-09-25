use std::error::Error;
use std::fs;

fn main() {
    println!("Lexer starting up....");
    println!(
        "Input {:?}",
        fs::read_to_string("int_add.mky")
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
    );
    let lexer = Lexer::new(&String::from("int_add.mky"));
    println!("Output: {:?}", lexer.unwrap().input)
}

// Token name and (if applicable) the char associated with it
#[derive(Debug)]
enum Token {
    Illegal,
    Eof,
    Integer(char),
    PlusSign,
    Semicolon,
}

struct Lexer {
    input: Vec<Token>,    // Vec<Token>
    position: isize,      // our position in the input
    read_position: isize, // the position we are reading a char from
    ch: char,             // the current char
}

impl Lexer {
    fn new(filename: &str) -> Result<Lexer, Box<dyn Error>> {
        Result::Ok(Lexer {
            input: fs::read_to_string(filename)?
                .chars()
                .filter(|c| !char::is_whitespace(*c))
                .map(|c| match c {
                    '+' => Token::PlusSign,
                    ';' => Token::Semicolon,
                    c => {
                        if char::is_numeric(c) {
                            return Token::Integer(c);
                        } // implement another else caluse to read an identifier
                        Token::Illegal
                    }
                })
                .collect::<Vec<Token>>(),
            position: 0,
            read_position: 0,
            ch: ' ',
        })
    }

    fn nextToken() -> Option<Token> {
        Option::None
    }
}
