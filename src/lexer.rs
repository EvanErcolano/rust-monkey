use crate::token::Token;
use std::error::Error;

#[derive(Debug)]
pub struct Lexer {
    // str has to be on the  heap
    // &str (str ref) is for if we don't want to own the thing
    // String allows it to just manage itself and add mem onto the heap
    input: String,
    start_position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Result<Lexer, Box<dyn Error>> {
        Result::Ok(Lexer {
            input: input.to_string(),
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
                    // The index is at the starting double quote, so we push it forward
                    // by 1 and read until we find the index of the ending double quote.
                    let ending_quote_index = self.input[next_index + 1..]
                        .char_indices()
                        .find(|&(_, c)| c == '"')
                        .map(|(i, _)| i + next_index + 1)
                        .unwrap();

                    self.start_position = ending_quote_index + 1;
                    Option::Some(Token::Str(
                        self.input[next_index + 1..ending_quote_index].to_string(),
                    ))
                }
                '0'..='9' => {
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
                _ => {
                    self.start_position = next_index + 1;
                    Option::Some(Token::Illegal)
                }
            }
        } else {
            Option::None
        }
    }
}

#[cfg(test)]
mod test {
    // do the testing stuff
    // nothing is in scope by default so we can pull everythign we need
    // super means modules declarared above this one
    // use the rest of the stuff in this file?
    use super::*;

    #[test]
    fn test_() {
        let input = "
        1000 + 5;
        ";
        let mut lexer = Lexer::new(input).unwrap();
        assert_eq!(lexer.next().unwrap(), Token::Integer(1000));
        assert_eq!(lexer.next().unwrap(), Token::PlusSign);
        assert_eq!(lexer.next().unwrap(), Token::Integer(5));
        assert_eq!(lexer.next().unwrap(), Token::Semicolon);
    }
}
