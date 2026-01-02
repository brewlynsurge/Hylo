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
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    IsEqual,
    IsNotEqual,
    And,
    Or
}

impl Operator {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            Operator::Plus     => "+",
            Operator::Minus    => "-",
            Operator::Multiply => "*",
            Operator::Divide   => "/",
            
            Operator::Equals      => "=",
            Operator::GreaterThan => ">",
            Operator::LessThan    => "<",
            
            Operator::IsEqual            => "==",
            Operator::IsNotEqual         => "!=",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThanOrEqual    => "<=",
            Operator::And                => "&&",
            Operator::Or                 => "||"
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