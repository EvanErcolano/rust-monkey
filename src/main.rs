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
    let lexer = Lexer::new(&String::from("int_add.mky")).unwrap();
    for token in lexer {
        println!("{:?}", token);
    }
}

// Token name and (if applicable) the char associated with it
#[derive(Debug)]
enum Token {
    Illegal,
    Integer(isize),
    PlusSign,
    Semicolon,
    Str(String),
}

struct Lexer {
    // str has to be on the  heap
    // &str (str ref) is for if we don't want to own the thing
    // String allows it to just manage itself and add mem onto the heap
    input: String,
    start_position: usize,
}

impl Lexer {
    fn new(filename: &str) -> Result<Lexer, Box<dyn Error>> {
        Result::Ok(Lexer {
            input: fs::read_to_string(filename)?,
            start_position: 0,
        })
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_non_whitespace_index = self.input[self.start_position..]
            .char_indices()
            .find(|&(_, c)| !c.is_whitespace())
            .map(|(i, _)| i + self.start_position);

        if let Some(next_index) = next_non_whitespace_index {
            // Check if it's a single token or a multi char one
            match self.input.chars().nth(next_index).unwrap() {
                '+' => {
                    self.start_position = next_index + 1;
                    Option::Some(Token::PlusSign)
                }
                ';' => {
                    self.start_position = next_index + 1;
                    Option::Some(Token::Semicolon)
                }
                '"' => {
                    // push ourselves past the first double quote and read until the next one
                    let ending_quote_index = self.input[next_index + 1..]
                        .char_indices()
                        .find(|&(_, c)| c == '"')
                        .map(|(i, _)| i + next_index)
                        .unwrap();

                    // debug this 
                    self.start_position = ending_quote_index + 2;
                    Option::Some(Token::Str(
                        self.input[next_index + 1..=ending_quote_index].to_string(),
                    ))
                }
                _ => {
                    let end_position = self.input[next_index..]
                        .char_indices()
                        .find(|&(_, c)| c.is_whitespace() || !c.is_ascii_digit())
                        .map(|(i, _)| i + next_index)
                        .unwrap_or_else(|| self.input.len());

                    let token = Option::Some(Token::Integer(
                        self.input[next_index..end_position].parse().unwrap(),
                    ));
                    self.start_position = end_position;
                    token
                }
            }
        } else {
            Option::None
        }
    }
}
