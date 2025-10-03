use std::process::exit;

use crate::lexer::source_code::SourceCodeContainer;
use crate::lexer::lexer::Lexer;
use crate::hylo_error;

/* HYLO COMPILER*/
pub struct HyloCompiler;

impl HyloCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_stdin(&self, source_code: String) {
        let source_code = SourceCodeContainer::from(source_code);


        let tokens = match Lexer::parse(&source_code, "<stdin>") {
            Ok(t) => t,
            Err(e) => {
                e.panic(Some(&source_code));
            }
        };
    }
}