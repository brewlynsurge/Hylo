use hylo_core::lexer::lexer::HyloLexer;

fn main() {
    let mut lexer = HyloLexer::new();
    let result = lexer.parse(String::from("let a = 10;"));
    println!("{:?}", result);
}
