use crate::lexer::source_code::SourceCodeContainer;
use crate::lexer::lexer::Lexer;

/* HYLO COMPILER*/
pub struct HyloCompiler;

impl HyloCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_stdin(&self, source_code: String) {
        let source_code = SourceCodeContainer::from(source_code);
        source_code.get_error_source(22, 25);
        let tokens = Lexer::parse(&source_code);
    }
}