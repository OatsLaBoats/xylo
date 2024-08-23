pub struct Scanner<'a> {
    pub chars: &'a [u8],
    pub current_pos: usize,
}

#[allow(dead_code)]
impl<'a> Scanner<'_> {
    pub fn advance(&mut self) -> char {
        self.current_pos += 1;
        return self.chars[self.current_pos - 1] as char;
    }

    pub fn advance_n(&mut self, n: usize) {
        self.current_pos += n;
    }

    pub fn backtrack(&mut self) -> usize {
        self.current_pos -= 1;
        return self.current_pos;
    }

    pub fn peek(&self) -> char {
        self.chars[self.current_pos] as char
    }

    pub fn peek_last(&self) -> char {
        self.chars[self.current_pos - 1] as char
    }

    pub fn peek_n(&self, n: usize) -> char {
        self.chars[self.current_pos + n] as char
    }

    pub fn check(&self, c: char) -> bool {
        self.peek() == c
    }

    pub fn check_next(&self, c: char) -> bool {
        if self.is_at_end() { false } 
        else { self.peek_n(1) == c }
    }

    pub fn check_s(&self, s: &str) -> bool {
        let bytes = s.as_bytes();
        if self.current_pos + bytes.len() >= self.chars.len() { return false }

        let start_pos = self.current_pos;
        let end_pos = self.current_pos + bytes.len();

        return &self.chars[start_pos..end_pos] == bytes;
    }

    pub fn is_at_end(&self) -> bool {
        self.current_pos >= self.chars.len()
    }

    pub fn get_lexeme(&self, start_pos: usize) -> String {
        let s = &self.chars[start_pos..self.current_pos];
        return String::from_utf8_lossy(s).into_owned();
    }

    pub fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }
}
