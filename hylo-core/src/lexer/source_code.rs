#[derive(Debug)]
pub struct Line {
    pub line: Vec<char>,
    pub start_pos: usize,
    pub end_pos: usize,
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
    pub source_code: Vec<Line>,
    pub total_lines: usize,
    pub total_chars: usize,
}

impl SourceCodeContainer {
    pub fn from(source_code: String) -> Self {
        let mut code_container = Self {
            source_code: Vec::new(),
            total_lines: 0,
            total_chars: 0,
        };

        let mut pos_count: usize = 0;
        for l in source_code.split("\n") {
            let line_chars = {
                let mut chars: Vec<char> = l.chars().collect();
                chars.push('\n');
                chars
            };
            let end_pos = pos_count + line_chars.len() - 1;

            let s_line = Line {
                line: line_chars,
                start_pos: pos_count,
                end_pos: end_pos,
            };
            pos_count = s_line.end_pos + 1;

            code_container.source_code.push(s_line);
            code_container.total_lines += 1;
        }
        code_container.total_chars = pos_count;

        return code_container;
    }

    pub fn char_at(&self, pos: usize) -> Option<char> {
        if pos >= self.total_chars {
            return None;
        }

        for l in self.source_code.iter() {
            if pos > l.end_pos {
                continue;
            }

            return Some(l.get_char_relative(pos));
        }

        return None;
    }

    pub fn get_text(&self, start_pos: usize, end_pos: usize) -> Option<String> {
        if start_pos > end_pos || end_pos > self.total_chars - 1 {
            return None;
        }

        let mut text_data = String::new();
        for (idx, l) in self.source_code.iter().enumerate() {
            if start_pos > l.end_pos {
                continue;
            }

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

    pub fn get_line_and_column(&self, pos: usize) -> (usize, usize) {
        let mut lineno: usize = 0;
        let mut columnno: usize = 0;
        for l in self.source_code.iter() {
            lineno += 1;
            if pos > l.end_pos {
                continue;
            }

            columnno = l.get_relative_pos(pos) + 1;
            break;
        }

        return (lineno, columnno);
    }
}
