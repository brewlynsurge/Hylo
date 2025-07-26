use super::tokens;

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