// Represents all possible tokens in Hylo
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Word(String),

    Operator(Operator),
    Punctuation(Punctuation)
}

//// Stores the token along with its starting position
#[derive(Debug, PartialEq, Clone)]
pub struct TokenContainer {
    pub token: Token,
    pub line: usize,
    pub column: usize
}

/*
Complex Tokens:
    - Keyword
    - Operator
    - Punctuation
*/

/// Represents operators in Hylo.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            Operator::Equals   => "=",
            Operator::Plus     => "+",
            Operator::Minus    => "-",
            Operator::Multiply => "*",
            Operator::Divide   => "/",
        }
    } 
}


/// Represents punctuation symbols in Hylo.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Punctuation {
    Semicolon,
    LParen, 
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

impl Punctuation {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            Punctuation::Semicolon => ";",
            Punctuation::LParen    => "(",
            Punctuation::RParen    => ")",
            Punctuation::LBrace    => "{",
            Punctuation::RBrace    => "}",
            Punctuation::LBracket  => "[",
            Punctuation::RBracket  => "]",
        }
    }
}