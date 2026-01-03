use hylo_core::compiler::HyloCompiler;

fn main() {
    let compiler = HyloCompiler::new();

    // TODO: Change
    let file_data = std::fs::read_to_string("test.hy").unwrap();
    compiler.compile_stdin(file_data);
}
