// Token name and (if applicable) the char associated with it
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //TODO make this flat? and store a tuple of token type enum to literal?
    Illegal,
    Integer(isize),
    PlusSign,
    Semicolon,
    Str(String),
    Let,
    Assign,
    Identifier(String),
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn is_assign(&self) -> bool {
        match self {
            Token::Assign => true,
            _ => false,
        }
    }


    pub fn is_semicolon(&self) -> bool {
        match self {
            Token::Semicolon => true,
            _ => false,
        }
    }
}