use std::str::Chars;
use std::iter::Peekable;

pub struct InputStream<'a> {
    input: Peekable<Chars<'a>>,
    line: i32,
    column: i32
}

impl<'a> InputStream<'a> {
    pub fn new(input: &'a String) -> InputStream<'a> {
        InputStream {
            input: input.chars().peekable(),
            line: 1,
            column: 0
        }
    }

    pub fn next(&mut self) -> char {
        match self.input.next() {
            Some(c) if c == '\n' => {
                self.line += 1;
                self.column = 0;
                c
            },
            Some(c) => {
                self.column += 1;
                c
            },
            None => '\0'
        }
    }

    pub fn peek(&mut self) -> char {
        if let Some(c) = self.input.peek() {
            c.clone()
        }
        else {
            '\0'
        }
    }

    pub fn eof(&mut self) -> bool {
        self.input.peek().is_none()
    }

    pub fn panic(&self, message: &String) -> ! {
        panic!(
            "{message} ({line}:{column})",
            message = message,
            line = self.line,
            column = self.column
        );
    }
}
