use crate::lexer::source_code::SourceCodeContainer;
use crate::lexer::lexer::Lexer;

use crate::parser::parser::Parser;

/* HYLO COMPILER*/
pub struct HyloCompiler;

impl HyloCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_stdin(&self, source_code: String) {
        let source_code = SourceCodeContainer::from(source_code);


        let token_containers = match Lexer::parse(&source_code, "<stdin>") {
            Ok(t) => t,
            Err(e) => {
                e.panic(Some(&source_code));
            }
        };
        
        let mut hylo_parser = Parser::new(token_containers, "<stdin>");
        hylo_parser.parse_program();
    }
}