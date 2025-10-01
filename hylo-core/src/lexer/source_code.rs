pub struct SourceCodeContainer {
    pub source_code: Vec<Vec<char>>,
    pub total_lines: usize,
    pub total_chars: usize
}

impl SourceCodeContainer {
    pub fn from(source_code: String) -> Self {
        let mut code_container = Self {
            source_code: vec![Vec::new()],
            total_lines: 0,
            total_chars: 0
        };

        let characters: Vec<char> = source_code.chars().collect();
        let mut pos:usize = 0;
        let mut vec_idx: usize = 0;

        code_container.increment_line();
        while pos < characters.len() {
            if characters[pos] == '\n' {
                code_container.insert_char(characters[pos], vec_idx);
                pos += 1;
                vec_idx += 1;
                code_container.increment_line();
            } else if characters[pos] == '\'' || characters[pos] == '"' {
                code_container.insert_char(characters[pos], vec_idx);
                pos += 1;

                while pos < characters.len() {
                    code_container.insert_char(characters[pos], vec_idx);
                    pos += 1;

                    if characters.get(pos) == Some(&'\'') || characters.get(pos) == Some(&'"') {
                        break;
                    } else if characters.get(pos) == Some(&'\n') {
                        vec_idx += 1;
                        code_container.increment_line();
                    }
                }
    
            }
            else {
                code_container.insert_char(characters[pos], vec_idx);
                pos += 1;
            }
        }

        return code_container;
    }

    pub fn get_char(&self, pos: usize) -> Option<char> {
        let mut pos = pos;
        if pos > self.total_chars-1 {
            return None;
        }

        let mut vec_idx: usize = 0;
        while vec_idx < self.total_lines {
            let line_len = self.source_code[vec_idx].len();
            if pos < line_len {
                return Some(self.source_code[vec_idx][pos]);
            } else {
                pos -= line_len;
                vec_idx += 1;
            }
        }
        
        return None;
    }

    fn insert_char(&mut self, c: char, line: usize) {
        self.source_code[line].push(c);
        self.total_chars += 1;
    }

    fn increment_line(&mut self) {
        self.source_code.push(Vec::new());
        self.total_lines += 1;
    }

    pub fn get_text(&self, start_pos: usize, end_pos: usize) -> Option<String> { // TODO: Rewrite for optimisation
        if start_pos > end_pos || end_pos > self.total_chars-1 {
            return None;
        }

        let mut c_pos = 0;
        let mut text = String::new();
        while c_pos < self.total_chars {
            if c_pos >= start_pos && c_pos <= end_pos {
                text.push(self.get_char(c_pos).unwrap());
            }

            c_pos += 1;
        }
        
        return Some(text);
    }

    pub fn get_lines(&self, start_pos: usize, end_pos: usize) -> Option<Vec<String>> {
        if start_pos > end_pos {
            return None;
        }

        let start_line_no = self.get_line_no(start_pos)?;
        let end_line_no = self.get_line_no(end_pos)?;

        let mut lines: Vec<String> = Vec::new();
        let mut c_line_no = start_line_no;
         
        while c_line_no <= end_line_no {
            let c_line: String = self.source_code[c_line_no-1].iter().collect();
            lines.push(c_line);
            c_line_no += 1;
        }
        
        return Some(lines);
    }

    fn get_line_no(&self, pos: usize) -> Option<usize> {
        let mut pos = pos;
        if pos > self.total_chars {
            return None;
        }

        let mut vec_idx: usize = 0;
        while vec_idx < self.total_lines {
            let line_len = self.source_code[vec_idx].len();
            if pos <= line_len {
                return Some(vec_idx+1);
            } else {
                pos -= line_len;
                vec_idx += 1;
            }
        }

        return None;
    }
}