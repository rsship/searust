pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        return Self { content };
    }

    fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, idx: usize) -> &'a [char] {
        let token = &self.content[0..idx];
        self.content = &self.content[idx..];
        return token;
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }

        return self.chop(n);
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        };

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphabetic()));
        };

        return Some(self.chop(1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.next_token()
    }
}
