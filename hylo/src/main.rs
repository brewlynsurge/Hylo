use hylo_core::compiler::HyloCompiler;

fn main() {
    let compiler = HyloCompiler::new();
    compiler.compile_stdin(String::from("
    let a = 10;
    let b = 5;
    let c = a + b;
    
    print('Hello World);
    
    "));

}