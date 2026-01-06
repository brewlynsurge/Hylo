use super::tokens;
use super::source_code::SourceCodeContainer;
use crate::hylo_error;

pub struct Lexer;
impl Lexer {
    pub fn parse(source_code: &SourceCodeContainer, file_name: &str) -> Result<Vec<TokenContainer>, hylo_error::Error> {
        let mut generated_tokens = Vec::new();
        let mut pos:usize = 0;

        while pos < source_code.total_chars {
            let c_char = source_code.char_at(pos).unwrap();
            if c_char.is_whitespace() {
                pos += 1;
                continue;
            }

            let generated_token_container = match c_char {
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
                '"' => Self::handle_string(source_code, &mut pos, file_name)?,
                '\'' => Self::handle_string(source_code, &mut pos, file_name)?,
                current_char if current_char.is_alphabetic() || current_char == '_' => {
                    Self::handle_word(source_code, &mut pos)
                },
                current_char if current_char.is_digit(10) => Self::handle_number(source_code, &mut pos, file_name)?,
                
                // Matching operators
                current_char if matches!(current_char, '+' | '-' | '*' | '/' | '>' |'<' | '=' | '&' | '!' | '|') => {
                    let next_char = source_code.char_at(pos+1);
                    let mut op_char = String::new();
                    if let Some(next_char) = next_char  && matches!(next_char, '+' | '-' | '*' | '/' | '>' |'<' | '=' | '&' | '!' | '|') {
                        op_char.extend([current_char, next_char].iter());
                    }  else { op_char.push(current_char); }
                    
                    let op_token = match Self::handle_operators(&op_char) {
                        Some(t) => t,
                        None => {
                            return Err(hylo_error::Error::new(
                                hylo_error::ErrorKind::SyntaxError,
                                hylo_error::Span { start: pos.clone(), stop: pos.clone() + op_char.len() - 1 },
                                Some(file_name)
                            ).add_msg("The operator is invalid"));
                        }
                    };
                    
                    pos += op_char.len();
                    TokenContainer {
                        token: op_token,
                        start: pos - op_char.len(),
                        end: pos - 1
                    }
                    
                }
                _ => {
                    return Err(hylo_error::Error::new(
                        hylo_error::ErrorKind::SyntaxError,
                        hylo_error::Span { start: pos.clone(), stop: pos.clone() },
                        Some(file_name)
                    ).add_msg("The token is invalid"));

                }
            };
            generated_tokens.push(generated_token_container);
        }

        return Ok(generated_tokens);
    }
    
    fn handle_operators(op_char: &str) -> Option<tokens::Token> {
        let op_token = match op_char {
            "+" => tokens::Token::Operator(tokens::Operator::Plus),
            "-"=> tokens::Token::Operator(tokens::Operator::Minus),
            "*" => tokens::Token::Operator(tokens::Operator::Multiply),
            "/" => tokens::Token::Operator(tokens::Operator::Divide),
            
            "=" => tokens::Token::Operator(tokens::Operator::Equals),
            "!" => tokens::Token::Operator(tokens::Operator::Exclamation),
            ">" => tokens::Token::Operator(tokens::Operator::GreaterThan),
            "<" => tokens::Token::Operator(tokens::Operator::LessThan),
            
            "==" => tokens::Token::Operator(tokens::Operator::IsEqual),
            "!=" => tokens::Token::Operator(tokens::Operator::IsNotEqual),
            ">=" => tokens::Token::Operator(tokens::Operator::GreaterThanOrEqual),
            "<=" => tokens::Token::Operator(tokens::Operator::LessThanOrEqual),
            "&&" => tokens::Token::Operator(tokens::Operator::And),
            "||" => tokens::Token::Operator(tokens::Operator::Or),
            "."  => tokens::Token::Operator(tokens::Operator::Dot),
            "->" => tokens::Token::Operator(tokens::Operator::Arrow),
            
            _ => return None
        };
        
        return Some(op_token)
    }
    
    fn handle_string(source_code: &SourceCodeContainer, pos: &mut usize, file_name: &str) -> Result<TokenContainer, hylo_error::Error> {
        let start_pos = pos.clone();
        let start_sym = source_code.char_at(*pos).unwrap();
        *pos += 1;

        let mut string_terminated = false;
        let mut first_line_end_pos: i32 = -1;

        while *pos < source_code.total_chars {
            if source_code.char_at(*pos).unwrap() == start_sym {
                string_terminated = true;
                break;
            }

            if first_line_end_pos == -1 && source_code.char_at(*pos).unwrap() == '\n' {
                first_line_end_pos = pos.clone() as i32
            }
            *pos += 1;
        }
        
        if !string_terminated {
            if first_line_end_pos == -1 {
                first_line_end_pos = pos.clone() as i32
            }

            // StringNotTerminated Error
            return Err(hylo_error::Error::new(
                hylo_error::ErrorKind::StringNotTerminated,
                hylo_error::Span { start: start_pos, stop: first_line_end_pos as usize },
                Some(file_name)
            ).add_msg("Expected end of the string"));

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
            let c_char = source_code.char_at(*pos).unwrap();
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

    fn handle_number(source_code: &SourceCodeContainer, pos: &mut usize, file_name: &str) -> Result<TokenContainer, hylo_error::Error> {
        let start_pos = pos.clone();

        let mut is_float = false;
        while *pos < source_code.total_chars {
            let c_char = source_code.char_at(*pos).unwrap();
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
                Err(_) => {
                    // InvalidNumber Error for float
                    return Err(hylo_error::Error::new(
                        hylo_error::ErrorKind::SyntaxError,
                        hylo_error::Span { start: start_pos, stop: pos.clone() - 1 },
                        Some(file_name)
                    ).add_msg("The float is not valid"));
                }
            }
        } else {
            match number_str.parse::<i32>() {
                Ok(value) => {
                    return Ok(TokenContainer { token: tokens::Token::Int(value),
                        start: start_pos,
                        end: pos.clone()-1
                    })
                }
                Err(_) => {
                    // InvalidNumber Error for integer
                    return Err(hylo_error::Error::new(
                        hylo_error::ErrorKind::SyntaxError,
                        hylo_error::Span { start: start_pos, stop: pos.clone() - 1 },
                        Some(file_name)
                    ).add_msg("The integer is not valid"));
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenContainer {
    pub token: tokens::Token,
    pub start: usize,
    pub end: usize
}