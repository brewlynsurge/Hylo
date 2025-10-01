#[derive(Debug)]
struct Line {
    pub line: Vec<char>,
    pub start_pos: usize,
    pub end_pos: usize
}
impl Line {
    pub fn get_relative_pos(&self, pos: usize) -> usize {
        return pos - self.start_pos;
    }

    pub fn get_char_relative(&self, pos: usize) -> char {
        let rel_pos = self.get_relative_pos(pos);
        return self.line[rel_pos];
    }
}

#[derive(Debug)]
pub struct SourceCodeContainer {
    source_code: Vec<Line>,
    pub total_lines: usize,
    pub total_chars: usize
}

impl SourceCodeContainer {
    pub fn from(source_code: String) -> Self {
        let mut code_container= Self {
            source_code: Vec::new(),
            total_lines: 0,
            total_chars: 0
        };

        let mut pos_count: usize = 0;
        for l in source_code.split("\n") {
            let line_chars = {
                let mut chars:Vec<char> = l.chars().collect();
                chars.push('\n');
                chars
            };
            let end_pos = pos_count + line_chars.len() - 1;
            
            let s_line = Line {
                line: line_chars,
                start_pos: pos_count,
                end_pos: end_pos
            };
            pos_count = s_line.end_pos + 1;
            
            code_container.source_code.push(s_line);
            code_container.total_lines += 1;
        }
        code_container.total_chars = pos_count;

        return code_container;
    }

    pub fn char_at(&self, pos: usize) -> Option<char> {
        if pos >= self.total_chars { return None; }

        for l in self.source_code.iter() {
            if pos > l.end_pos {
                continue;
            } 

            return Some(l.get_char_relative(pos));
        }

        return None;
    }

    pub fn get_text(&self, start_pos: usize, end_pos: usize) -> Option<String> {
        if start_pos > end_pos || end_pos > self.total_chars-1 {
            return None;
        }

        let mut text_data = String::new();
        for (idx, l) in self.source_code.iter().enumerate() {
            if start_pos > l.end_pos { continue }

            let mut c_pos = start_pos;
            let mut vec_idx = idx;
            while c_pos <= end_pos {
                let rel_pos = self.source_code[vec_idx].get_relative_pos(c_pos);
                text_data.push(self.source_code[vec_idx].line[rel_pos]);
                
                if c_pos == self.source_code[vec_idx].end_pos {
                    vec_idx += 1;
                }

                c_pos += 1;
            }

            break;
        }

        return Some(text_data);
    }

    pub fn get_error_source(&self, start_pos: usize, end_pos: usize) -> ErrorSource {
        if start_pos > end_pos || end_pos > self.total_chars-1 {
            panic!("Expected a valid start or end pos");
        }

        let mut line: String = String::new();
        let mut rel_start_index = -1;
        for l in self.source_code.iter() {
            if start_pos > l.end_pos {
                continue
            } else if end_pos < l.start_pos {
                break;
            } else {
                if rel_start_index == -1 {
                    rel_start_index = l.get_relative_pos(start_pos) as i32;
                }
                line.extend(l.line.iter());
            }
        }

        let rel_start_pos = rel_start_index as usize;
        let rel_end_pos = rel_start_pos + (end_pos - start_pos);

        let mut error_src = ErrorSource {
            line: line,
            mark_start: rel_start_pos,
            mark_stop: rel_end_pos
        };
        error_src.trim_line();
        return error_src;
    }

    pub fn get_line_and_column(&self, pos: usize) -> (usize, usize) {
        let mut lineno: usize = 0;
        let mut columnno: usize = 0;
        for l in self.source_code.iter() {
            lineno += 1;
            if pos > l.end_pos { continue }

            columnno = l.get_relative_pos(pos) + 1;
            break;
        }

        return (lineno, columnno);
    }
}

#[derive(Debug)]
pub struct ErrorSource {
    pub line: String,
    pub mark_start: usize,
    pub mark_stop: usize
}

impl ErrorSource {
    pub fn trim_line(&mut self) {
        let original = self.line.as_str();

        // Find leading/trailing whitespace ranges
        let trimmed_start = original
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(0);
        let trimmed_end = original
            .rfind(|c: char| !c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(original.len());

        // Update line
        self.line = original[trimmed_start..trimmed_end].to_string();

        // Adjust marks
        if self.mark_start < trimmed_start {
            self.mark_start = 0;
        } else {
            self.mark_start -= trimmed_start;
        }

        if self.mark_stop < trimmed_start {
            self.mark_stop = 0;
        } else {
            self.mark_stop -= trimmed_start;
        }

        // Clamp to the new line length (in case trimming cut beyond mark_stop)
        let len = self.line.len();
        self.mark_start = self.mark_start.min(len);
        self.mark_stop = self.mark_stop.min(len);
    }
}