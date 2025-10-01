use crate::lexer::source_code::SourceCodeContainer;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    SyntaxError,
    StringNotTerminated,
}
impl ErrorKind {
    fn as_str(&self) -> &str {
        match self {
            ErrorKind::SyntaxError => "SyntaxError",
            ErrorKind::StringNotTerminated => "StringNotTerminated",
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
    pub message: String,
    pub notes: Vec<String>,
    pub file: String
}

impl Error {
    pub fn new(error_kind: ErrorKind, span:Span, file: String) -> Self {
        Error { kind: error_kind,
            span: span,
            message: String::new(),
            notes: Vec::new(),
            file: file
        }
    }

    pub fn add_note(mut self, note: String) -> Self {
        self.notes.push(note);
        return self;
    }

    pub fn inject_msg(mut self, msg: &str) -> Self {
        self.message.push_str(msg);
        return self;
    }

    pub fn prettify(&self, sc: &SourceCodeContainer) -> String {
        let mut output = String::new();

        // 1. Error header
        output.push_str(&format!("Error: {}\n", self.kind.as_str()));

        // 2. Location info (file:line:column)
        let (line_no, column_no) = sc.get_line_and_column(self.span.start);
        output.push_str(&format!("   --> {}:{}:{}\n",
            self.file,
            line_no,
            column_no
        ));

        // 3. Add the margin bar
        output.push_str("    |\n");

        // 4. Get source line
        let error_source = sc.get_error_source(self.span.start, self.span.end);

        
        // 5. Print line number + code
        output.push_str(&format!("{:>3} | {}\n", line_no, &error_source.line));

        // 6. Underline with ^
        output.push_str("    | ");
        output.push_str(&" ".repeat(error_source.mark_start));
        
        let span_len = if error_source.mark_stop > error_source.mark_start {
            error_source.mark_stop - error_source.mark_start + 1
        } else { 1 };
        
        output.push_str(&"^".repeat(span_len));
        output.push_str(&format!(" {}\n", self.message));

        // 7. Notes
        for note in &self.notes {
            output.push_str(&format!("note: {}\n", note));
        }
        return output;
    }
}