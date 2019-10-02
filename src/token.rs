// Token name and (if applicable) the char associated with it
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Integer(isize),
    PlusSign,
    Semicolon,
    Str(String),
}