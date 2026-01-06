use std::str::Matches;

use crate::parser::components::{BinaryOp, Expr, Literal, Span, Stmt, UnaryOp};
use crate::lexer::lexer::TokenContainer;
use crate::lexer::tokens;
use crate::hylo_error;

/* HYLO PARSER */

pub struct Parser {
    token_containers: Vec<TokenContainer>,
    pos: usize,
    file_name: String
}

impl Parser {
    pub fn new(token_containers: Vec<TokenContainer>, file_name: &str) -> Self {
        Parser {
            token_containers,
            pos: 0,
            file_name: String::from(file_name)
        }
    }
    
    fn is_available(&self, pos: usize) -> bool {
        pos < self.token_containers.len()
    }
    
    
    fn peek(&self) -> Option<&tokens::Token> {
        self.token_containers
            .get(self.pos)
            .map(|t_con| &t_con.token)
    }
    
    fn advance(&mut self) -> Option<TokenContainer> {
        if let Some(t_con) = self.token_containers.get(self.pos) {
            self.pos += 1;
            return Some(t_con.clone())
        } else {return None} 
    }
    
    pub fn parse_program(&mut self) {
        let mut statements = Vec::new();
        while self.is_available(self.pos) {
            statements.push(self.parse_statement());
        }
    }
    
    fn parse_statement(&mut self) -> Stmt {
        self.parse_term()
    }
    
    fn parse_term(&mut self) -> Stmt {
        let r = self.parse_factor();
        println!("{:?}", r.unwrap());
        
        todo!()
    }
    
    fn parse_factor(&mut self) -> Result<Expr, hylo_error::Error> {
        let mut expr = self.parse_unary()?;
        
        if self.peek().is_some() {
            while self.peek().is_some() && matches!(self.peek().unwrap().clone(), tokens::Token::Operator(tokens::Operator::Multiply) | tokens::Token::Operator(tokens::Operator::Divide)) {
                let op_parsed = {
                    let op_container = self.advance().unwrap();
                    // Convertion of lexer operator into parser form
                    match op_container.token {
                        tokens::Token::Operator(tokens::Operator::Multiply) => BinaryOp::Mul(Span { start: op_container.start, end: op_container.end }),
                        _ => BinaryOp::Div(Span { start: op_container.start, end: op_container.end })
                    }
                };
                
                let rhs = self.parse_unary()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: op_parsed,
                    right: Box::new(rhs)
                };
            }
        }
        
        return Ok(expr);
    }
    
    fn parse_unary(&mut self) -> Result<Expr, hylo_error::Error> {
        match self.peek() {
            Some(c_token) => {
                if matches!(c_token, tokens::Token::Operator(tokens::Operator::Minus) | tokens::Token::Operator(tokens::Operator::Exclamation)) {
                    let op_parsed = {
                        let op_container = self.advance().unwrap();
                        // Convertion of lexer operator into parser form
                        match op_container.token {
                            tokens::Token::Operator(tokens::Operator::Minus) => UnaryOp::Negative(Span { start: op_container.start, end: op_container.end }),
                            _ => UnaryOp::Not(Span { start: op_container.start, end: op_container.end })
                        }
                    };
                    
                    let expr = self.parse_unary()?;
                    return Ok(Expr::Unary {
                        op: op_parsed,
                        expr: Box::new(expr)
                    });
                } else { return self.parse_primary() }
            },
            None => { return self.parse_primary() }
        };
        
    }
    
    fn parse_primary(&mut self) -> Result<Expr, hylo_error::Error> {
        if let Some(token_con) = self.advance() {
            let pos = Span {
                start: token_con.start,
                end: token_con.end
            };
            
            let parsed_primary = match token_con.token {
                tokens::Token::Int(value) => Expr::Literal(Literal::Int(value, pos)),
                tokens::Token::Float(value) => Expr::Literal(Literal::Float(value, pos)),
                tokens::Token::String(value) => Expr::Literal(Literal::String(value, pos)),
                tokens::Token::Boolean(value) => Expr::Literal(Literal::Bool(value, pos)),
                tokens::Token::Word(value) => Expr::Literal(Literal::Word(value, pos)),
                unknown_token => {
                    return Err(hylo_error::Error::new(
                        hylo_error::ErrorKind::SyntaxError,
                        hylo_error::Span { start: token_con.start, stop: token_con.end},
                        Some(&self.file_name)
                    ).add_msg(&format!("Unexpected token: {:?}", unknown_token))); // TODO: Add formating to the unknown_token
                }
            };
            
            return Ok(parsed_primary);
        } else { return Ok(Expr::EOL) }
    }
    
}