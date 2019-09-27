// Token name and (if applicable) the char associated with it
#[derive(Debug)]
pub enum Token {
    Illegal,
    Integer(isize),
    PlusSign,
    Semicolon,
    Str(String),
}