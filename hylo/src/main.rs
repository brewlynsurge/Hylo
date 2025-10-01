use hylo_core::compiler::HyloCompiler;

fn main() {
    let compiler = HyloCompiler::new();
    compiler.compile_stdin(String::from("print('Hello World)"));

}



/* 
SyntaxError:
  --> hylo_test.hy:9:15
   |
 9 | let x = a + 3;
   |         ^ a was not defined

exited the program with status code: 1
*/


/*
error: StringNotTerminated
 --> line 2, column 10
let x = "Hello
         ^^^^^
Expected closing `"` for string literal
note: Strings must be terminated with a double quote.

*/