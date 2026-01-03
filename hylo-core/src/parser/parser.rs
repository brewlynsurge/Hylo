use crate::parser::components::Stmt;
use crate::lexer::lexer::TokenContainer;
//use crate::lexer::tokens;
//use crate::hylo_error;

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
    
    pub fn parse_program(&mut self) {
        let mut statements = Vec::new();
        while self.is_available(self.pos) {
            statements.push(self.parse_statement());
        }
    }
    
    fn parse_statement(&mut self) -> Stmt {
        for i in self.token_containers.iter() {
            println!("T: {:?}", i.token)
        }
        
        todo!()
    }
}