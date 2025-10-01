use std::ffi::c_char;

use super::tokens;
use super::source_code::SourceCodeContainer;

pub struct Lexer;
impl Lexer {
    pub fn parse(source_code: &SourceCodeContainer) -> Result<Vec<TokenContainer>, LexerError> {
        let mut generated_tokens = Vec::new();
        let mut pos:usize = 0;

        while pos < source_code.total_chars {
            let c_char = source_code.get_char(pos).unwrap();
            if c_char.is_whitespace() {
                pos += 1;
                continue;
            }

            let generated_token_container = match c_char {
                '=' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Operator(tokens::Operator::Equals),
                        start: pos - 1,
                        end: pos -1
                    }
                }
                '+' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Operator(tokens::Operator::Plus),
                        start: pos - 1,
                        end: pos -1
                    }
                }
                '-' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Operator(tokens::Operator::Minus),
                        start: pos - 1,
                        end: pos -1
                    }
                }
                '*' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Operator(tokens::Operator::Multiply),
                        start: pos - 1,
                        end: pos -1
                    }
                }
                '/' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Operator(tokens::Operator::Divide),
                        start: pos - 1,
                        end: pos -1
                    }
                }
                ';' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::Semicolon),
                        start: pos - 1,
                        end: pos - 1
                    }
                }
                '(' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::LParen),
                        start: pos - 1,
                        end: pos - 1
                    }
                }
                ')' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::RParen),
                        start: pos - 1,
                        end: pos - 1
                    }
                }
                '[' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::LBracket),
                        start: pos - 1,
                        end: pos - 1
                    }
                }
                ']' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::RBracket),
                        start: pos - 1,
                        end: pos - 1
                    }
                }
                '{' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::LBrace),
                        start: pos - 1,
                        end: pos - 1
                    }
                }
                '}' => {
                    pos += 1;
                    TokenContainer {
                        token: tokens::Token::Punctuation(tokens::Punctuation::RBrace),
                        start: pos - 1,
                        end: pos - 1
                    }
                },
                '"' => Self::handle_string(source_code, &mut pos)?,
                '\'' => Self::handle_string(source_code, &mut pos)?,
                current_char if current_char.is_alphabetic() || current_char == '_' => {
                    Self::handle_word(source_code, &mut pos)
                },
                current_char if current_char.is_digit(10) => Self::handle_number(source_code, &mut pos)?,

                _ => return Err(LexerError::InvalidToken { start:pos.clone(), stop: pos.clone() })
            };

            println!("{:?}", generated_token_container.token);
            generated_tokens.push(generated_token_container);
        }

        return Ok(generated_tokens);
    }

    fn handle_string(source_code: &SourceCodeContainer, pos: &mut usize) -> Result<TokenContainer, LexerError> {
        let start_pos = pos.clone();
        let start_sym = source_code.get_char(*pos).unwrap();
        *pos += 1;

        let mut string_terminated = false;
        let mut first_line_end_pos: i32 = -1;

        while *pos < source_code.total_chars {
            if source_code.get_char(*pos).unwrap() == start_sym {
                string_terminated = true;
                break;
            }

            if first_line_end_pos == -1 && source_code.get_char(*pos).unwrap() == '\n' {
                first_line_end_pos = pos.clone() as i32
            }
            *pos += 1;
        }
        
        if !string_terminated {
            if first_line_end_pos == -1 {
                first_line_end_pos = pos.clone() as i32
            }

            return Err(LexerError::StringNotTerminated { start: start_pos, stop: first_line_end_pos as usize});
        }

        let string_data = source_code.get_text(start_pos+1, pos.clone()-1).unwrap();
        let t_container = TokenContainer {
            token: tokens::Token::String(string_data),
            start: start_pos,
            end: pos.clone()
        };
        
        *pos += 1;
        return Ok(t_container);
    }
    

    fn handle_word(source_code: &SourceCodeContainer, pos: &mut usize) -> TokenContainer {
        let start_pos = pos.clone();
        while *pos < source_code.total_chars {
            let c_char = source_code.get_char(*pos).unwrap();
            if !c_char.is_alphanumeric() && c_char != '_' {
                break;
            }
            *pos += 1
        }

        let word = source_code.get_text(start_pos, pos.clone()-1).unwrap();
        let token = match word.as_str() {
            "true" => tokens::Token::Boolean(true),
            "false" => tokens::Token::Boolean(false),
            _ => tokens::Token::Word(word)
        };

        TokenContainer {
            token: token,
            start: start_pos,
            end: pos.clone()-1
        }
    }

    fn handle_number(source_code: &SourceCodeContainer, pos: &mut usize) -> Result<TokenContainer, LexerError> {
        let start_pos = pos.clone();

        let mut is_float = false;
        while *pos < source_code.total_chars {
            let c_char = source_code.get_char(*pos).unwrap();
            if !(c_char.is_digit(10) || c_char == '.') {
                break;
            } else if c_char == '.' { is_float = true; }

            *pos += 1;
        }

        let number_str = source_code.get_text(start_pos, pos.clone()-1).unwrap();
        if is_float {
            match number_str.parse::<f32>() {
                Ok(value) => {
                    return Ok(TokenContainer { token: tokens::Token::Float(value),
                        start: start_pos,
                        end: pos.clone()-1
                    })
                }
                Err(_) => return Err(LexerError::InvalidFloat { start: start_pos, stop: pos.clone() - 1})
            }
        } else {
            match number_str.parse::<i32>() {
                Ok(value) => {
                    return Ok(TokenContainer { token: tokens::Token::Int(value),
                        start: start_pos,
                        end: pos.clone()-1
                    })
                }
                Err(_) => return Err(LexerError::InvalidInteger { start: start_pos, stop: pos.clone() - 1})
            }
        }
    }
}

pub struct TokenContainer {
    token: tokens::Token,
    start: usize,
    end: usize
}


pub enum LexerError {
    StringNotTerminated {start: usize, stop: usize},
    InvalidInteger {start: usize, stop: usize},
    InvalidFloat {start: usize, stop: usize},
    InvalidToken {start: usize, stop: usize}
}




/* 
pub struct HyloLexer {
    characters: Vec<char>, // Entire code is stored as char vector
    position: usize,       // Tracks position of lexer in the characters vec
    line: usize,           // Represents real line number in the source code
    column: usize,         // Represents real column number in the source code
}

impl HyloLexer {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn parse(&mut self, code: String) -> Result<Vec<tokens::TokenContainer>, LexicalError> {
        let mut tokens = Vec::new();
        self.characters.extend(code.chars());

        while self.position < self.characters.len() {
            self.manage_spaces(); // Manage white spaces and new lines

            let current_char = self.characters[self.position];
            let current_line = self.line;
            let current_column = self.column;

            let generated_token = match current_char {
                '=' => {
                    self.advance(false);
                    tokens::Token::Operator(tokens::Operator::Equals)
                }
                '+' => {
                    self.advance(false);
                    tokens::Token::Operator(tokens::Operator::Plus)
                }
                '-' => {
                    self.advance(false);
                    tokens::Token::Operator(tokens::Operator::Minus)
                }
                '*' => {
                    self.advance(false);
                    tokens::Token::Operator(tokens::Operator::Multiply)
                }
                '/' => {
                    self.advance(false);
                    tokens::Token::Operator(tokens::Operator::Divide)
                }
                ';' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::Semicolon)
                }
                '(' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::LParen)
                }
                ')' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::RParen)
                }
                '[' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::LBracket)
                }
                ']' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::RBracket)
                }
                '{' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::LBrace)
                }
                '}' => {
                    self.advance(false);
                    tokens::Token::Punctuation(tokens::Punctuation::RBrace)
                }
                '"' => self.read_string()?,
                '\'' => self.read_string()?,
                current_char if current_char.is_alphabetic() || current_char == '_' => self.read_word(),
                current_char if current_char.is_digit(10) => self.read_number()?,
                _ => {
                    self.advance(false);
                    return Err(LexicalError {
                        msg: format!("UNEXPECTED_CHARACTER"),
                        line: current_line,
                        column: current_column
                    })
                }
            };

            let token_container = tokens::TokenContainer {
                token: generated_token,
                line: current_line,
                column: current_column
            };
            tokens.push(token_container);
        }
        return Ok(tokens);
    }

    fn read_string(&mut self) -> Result<tokens::Token, LexicalError> {
        let start_line = self.line;
        let start_column = self.column;
        let string_sym = self.characters[self.position];

        self.advance(false); // Skip opening quote
        let start_pos = self.position;

        while self.position < self.characters.len() && self.characters[self.position] != string_sym
        {
            if self.characters[self.position] == '\n' {
                self.advance(true);
            } else {
                self.advance(false);
            }
        }

        if self.position >= self.characters.len() {
            return Err(LexicalError {
                msg: String::from("STRING_NOT_TERMINATED"),
                line: start_line,
                column: start_column,
            });
        }

        let string_data: String = self.characters[start_pos..self.position].iter().collect();
        self.advance(false); // Skip closing quote
        return Ok(tokens::Token::String(string_data));
    }

    fn read_word(&mut self) -> tokens::Token {
        let start_pos = self.position;

        while self.position < self.characters.len() {
            let c = self.characters[self.position];
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance(false);
        }

        let word: String = self.characters[start_pos..self.position].iter().collect();

        match word.as_str() {
            "true" => tokens::Token::Boolean(true),
            "false" => tokens::Token::Boolean(false),
            _ => tokens::Token::Word(word)
        }
    }

    fn read_number(&mut self) -> Result<tokens::Token, LexicalError> {
        let start_pos = self.position;
        let start_line = self.line;
        let start_column = self.column;

        let mut is_float = false;
        let lexical_error = LexicalError {
            msg: String::from("INVALID_NUMBER"),
            line: start_line,
            column: start_column,
        };

        while self.position < self.characters.len() && (self.characters[self.position].is_digit(10) || self.characters[self.position] == '.'){
            if self.characters[self.position] == '.' {
                is_float = true;
            }
            self.advance(false);
        }

        let num_str: String = self.characters[start_pos..self.position].iter().collect();
        if is_float {
            match num_str.parse::<f32>() {
                Ok(value) => return Ok(tokens::Token::Float(value)),
                Err(_) => return Err(lexical_error)
            }
        } else {
            match num_str.parse::<i32>() {
                Ok(value) => return Ok(tokens::Token::Int(value)),
                Err(_) => return Err(lexical_error)
            }
        }
    }

    fn manage_spaces(&mut self) {
        let current_char = self.characters[self.position];

        if current_char.is_whitespace() {
            match current_char {
                ' ' => self.advance(false),
                '\n' => self.advance(true),
                _ => self.advance(false),
            }
        }
    }

    fn advance(&mut self, new_line: bool) {
        if new_line {
            self.line += 1;
            self.column = 1;
        } else { self.column += 1; }

        self.position += 1;
    }
}

/* LexicalError */
#[derive(Debug, PartialEq, Clone)]
pub struct LexicalError {
    msg: String,
    line: usize,
    column: usize,
}
    */