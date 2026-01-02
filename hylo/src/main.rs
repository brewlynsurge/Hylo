use hylo_core::compiler::HyloCompiler;

fn main() {
    let compiler = HyloCompiler::new();
    compiler.compile_stdin(String::from("
        print('Hello world');
    "));

}