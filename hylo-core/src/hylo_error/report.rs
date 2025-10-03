use std::process;
use crate::lexer::source_code::SourceCodeContainer;
use super::kind::ErrorKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub stop: usize
}

/*
Error
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
    pub file_name: Option<String>,
    pub message: String,
    pub notes: Vec<String>
}

impl Error {
    pub fn new(error_kind: ErrorKind, span: Span, file_name: Option<&str>) -> Self {
        Error {
            kind: error_kind,
            span: span,
            file_name: file_name.map(|s| s.to_string()),
            message: String::new(),
            notes: Vec::new()
        }
    }

    pub fn add_note(mut self, note: &str) -> Self {
        self.notes.push(String::from(note));
        return self;
    }

    pub fn add_msg(mut self, msg: &str) -> Self {
        self.message.push_str(msg);
        return self;
    }

    pub fn panic(&self, source_code:Option<&SourceCodeContainer>) -> ! {
        let pretty_error = self.pretty(source_code);
        eprintln!("{}", pretty_error);

        process::exit(self.kind.exit_code());
    }

    pub fn pretty(&self, source_code:Option<&SourceCodeContainer>) -> String {
        let mut output = String::new();

        let file_name = self.file_name.as_deref().unwrap_or("<unknown>");
        if source_code.is_none() {
            // Header
            output.push_str(&format!("❌ ERROR [{}]: {}\n", self.kind.code(), self.kind.as_str()));
            output.push_str(&format!("File: {} | Code: {}\n", file_name, self.kind.code()));
            output.push_str("   |\n");
            output.push_str(&format!("   | ❗ {}\n", self.message));

            // Note
            for note in &self.notes {
                output.push_str(&format!("   | 💡 {}\n", note));
            }
        } else {
            let source_code = source_code.unwrap();
            
            // 1. Error header
            output.push_str(&format!("❌ ERROR [{}]: {}\n", self.kind.code(), self.kind.as_str()));


            // 2. Location info (file:line:column)
            let (line_no, column_no) = source_code.get_line_and_column(self.span.start);
            output.push_str(&format!("   --> {}:{}:{}\n",
                file_name,
                line_no,
                column_no
            ));

            // 3. Add the margin bar
            output.push_str("    |\n");

            // 4. Get source line
            let error_source = ErrorSourceContainer::from(&source_code, self.span.start, self.span.stop);

            // 5. Print line number + code and underline error
            for (i, error_line) in error_source.lines.iter().enumerate() {
                output.push_str(&format!("{:>3} | {}\n", line_no+i, error_line));

                
                if error_source.start.0 + 1 == line_no+i {
                    let span_len = if error_source.end.1 > error_source.start.1 { error_source.end.1 - error_source.start.1} else { 1 };
                    
                    output.push_str("    | ");
                    output.push_str(&" ".repeat(error_source.start.1));
                    output.push_str(&"^".repeat(span_len));
                    output.push_str(&format!(" {}\n", self.message));
                }
            }
            
            // 6. Notes
            for note in &self.notes {
                output.push_str(&format!("💡 Hint: {}\n", note));
            }
        }

        return output;
    }
}

struct ErrorSourceContainer {
    lines: Vec<String>,
    start: (usize, usize),
    end: (usize, usize)
}

impl ErrorSourceContainer {
    pub fn from(source_code: &SourceCodeContainer, start_pos: usize, end_pos:usize) -> Self {
        if start_pos > end_pos || end_pos > source_code.total_chars-1 {
            panic!("Expected a valid start or end pos");
        }

        let mut error_source_container = Self {
            lines: Vec::new(),
            start: (0, 0),
            end: (0, 0)
        };
        
        let mut rel_start = (-1, -1);
        let mut rel_end_idx = -1;
        for (idx, l) in source_code.source_code.iter().enumerate() {
            if start_pos > l.end_pos {
                continue
            } else if end_pos < l.start_pos {
                rel_end_idx = idx as i32;
                break;
            } else {
                error_source_container.lines.push(l.line.iter().collect());
                if rel_start.0 == -1 {
                    rel_start.0 = idx as i32;
                    rel_start.1 = l.get_relative_pos(start_pos) as i32;
                }
            }
        }

        error_source_container.start = (rel_start.0 as usize, rel_start.1 as usize);
        error_source_container.end = (rel_end_idx as usize, error_source_container.start.1 + (end_pos - start_pos));
        error_source_container.trim();

        return error_source_container;
    }

    pub fn trim(&mut self) {
        if self.lines.is_empty() { return; }

        // Helper to find first non-whitespace byte index (or len if none)
        let first_non_ws = |s: &str| {
            s.char_indices()
                .find(|&(_, c)| !c.is_whitespace())
                .map(|(i, _)| i)
                .unwrap_or(s.len())
        };

        // Helper to find last non-whitespace byte index (end-exclusive),
        // or 0 if none.
        let last_non_ws_end = |s: &str| {
            s.char_indices()
                .rev()
                .find(|&(_, c)| !c.is_whitespace())
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(0)
        };

        // Special case: single line -> trim both ends
        if self.lines.len() == 1 {
            let s = &self.lines[0];
            let start = first_non_ws(s);
            let end = last_non_ws_end(s);

            let new = if start >= end { String::new() } else { s[start..end].to_string() };
            let new_len = new.len();

            // adjust byte offsets (assumes start.1 and end.1 are byte offsets)
            self.start.1 = self.start.1.saturating_sub(start).min(new_len);
            self.end.1 = self.end.1.saturating_sub(start).min(new_len);

            self.lines[0] = new;
            return;
        }

        // --- Trim first line (leading whitespace) ---
        {
            let first = &self.lines[0];
            let trimmed_start = first_non_ws(first); // byte offset, len if all whitespace
            if trimmed_start > 0 {
                self.lines[0] = first[trimmed_start..].to_string();
                // adjust start column (byte-offset)
                self.start.1 = self.start.1.saturating_sub(trimmed_start).min(self.lines[0].len());
            }
        }

        // --- Trim middle lines (trim both ends) ---
        if self.lines.len() > 2 {
            for i in 1..self.lines.len() - 1 {
                self.lines[i] = self.lines[i].trim().to_string(); // trim() removes whitespace incl. '\n', '\r'
            }
        }

        // --- Trim last line (trailing whitespace) ---
        {
            let last_idx = self.lines.len() - 1;
            let last = &self.lines[last_idx];
            let trimmed_end = last_non_ws_end(last); // end-exclusive byte index
            if trimmed_end < last.len() {
                self.lines[last_idx] = last[..trimmed_end].to_string();
                // clamp end column to the new length
                self.end.1 = self.end.1.min(trimmed_end);
            }
        }
    }

}