use super::InputStream;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Then,
    Else,
    Fn,
    True,
    False
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token { // TODO: add line/column to enums, pass up through parser for usable errors
    Punctuation(char),
    Number(f64),
    String_(String),
    Identifier(String),
    Operator(String),
    Keyword(Keyword)
}


pub struct TokenStream<'a> {
    input_stream: InputStream<'a>,
    current: Option<Token>
}

impl<'a> TokenStream<'a> {
    pub fn new(input_stream: InputStream) -> TokenStream {
        TokenStream {
            input_stream,
            current: None
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if let Some(_) = self.current {
            self.current.clone()
        }
        else {
            self.current = self.read_next();
            if let Some(_) = self.current {
                self.current.clone()
            }
            else {
                None
            }
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let token = self.current.to_owned();
        self.current = None;
        if let tok @ Some(_) = token {
            tok
        }
        else if let tok @ Some(_) = self.read_next() {
            tok
        }
        else {
            None
        }
    }

    pub fn eof(&mut self) -> bool {
        self.peek().is_none()
    }

    fn read_while<F>(&mut self, mut p: F) -> String where F: FnMut(&char) -> bool {
        let mut string = "".to_string();
        while !self.input_stream.eof() && p(&self.input_stream.peek()) {
            string.push(self.input_stream.next());
        }
        string
    }

    pub fn read_number(&mut self) -> Token {
        let mut has_dot = false;
        let number = self.read_while(|c|
            if c.clone() == '.' {
                if has_dot {
                    false
                }
                else {
                    has_dot = true;
                    true
                }
            }
            else {
                c.is_digit(10)
            }
        );
        Token::Number(number.parse::<f64>().unwrap())
    }

    pub fn read_identifier(&mut self) -> Token {
        let identifier = self.read_while(is_identifier);
        if let Some(keyword) = get_keyword(&identifier) {
            Token::Keyword(keyword)
        }
        else {
            Token::Identifier(identifier)
        }
    }

    pub fn read_escaped(&mut self, end: char) -> String {
        let mut escaped = false;
        let mut string = "".to_string();
        self.input_stream.next();
        while !self.input_stream.eof() {
            let c = self.input_stream.next();
            if escaped {
                string.push(c);
                escaped = false;
            }
            else if c == '\\' {
                escaped = true;
            }
            else if c == end {
                break;
            }
            else {
                string.push(c);
            }
        }
        string
    }

    pub fn read_string(&mut self) -> Token {
        let string = self.read_escaped('"');
        Token::String_(string)
    }

    pub fn skip_comment(&mut self) {
        self.read_while(|c|
            c.clone() != '\n'
        );
    }

    pub fn read_next(&mut self) -> Option<Token> {
        self.read_while(|c| c.is_whitespace());
        if self.input_stream.eof() {
            return None;
        }
        match self.input_stream.peek() {
            c if c == '#' => {
                self.skip_comment();
                self.read_next()
            },
            c if c == '"' =>
                Some(self.read_string()),
            c if c.is_digit(10) =>
                Some(self.read_number()),
            c if is_identifier_start(&c) =>
                Some(self.read_identifier()),
            c if is_punctuation(&c) =>
                Some(Token::Punctuation(self.input_stream.next())),
            c if is_operator(&c) =>
                Some(Token::Operator(self.read_while(is_operator))),
            c => self.input_stream.panic(&format!("Cannot handle char: '{}'", c.escape_debug()))
        }
    }

    pub fn panic(&self, message: String) -> ! {
        self.input_stream.panic(&message)
    }
}

fn is_operator(c: &char) -> bool {
    match c.clone() {
        '+' | '-' | '*' | '/' | '%' | '=' | '|' | '&' | '<' | '>' | '!' => true,
        _ => false
    }
}

fn is_punctuation(c: &char) -> bool {
    match c.clone() {
        ',' | ';' | '(' | ')' | '{' | '}' | '[' | ']' => true,
        _ => false
    }
}

fn get_keyword(identifier: &String) -> Option<Keyword> {
    match identifier.clone().as_ref() {
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "then" => Some(Keyword::Then),
        "fn" => Some(Keyword::Fn),
        "true" => Some(Keyword::True),
        "false" => Some(Keyword::False),
        _ => None
    }
}

fn is_identifier_start(c: &char) -> bool {
    let c = c.clone();
    c.is_alphabetic() || c == '_'
}

fn is_identifier(c: &char) -> bool {
    let c = c.clone();
    c.is_alphanumeric() || c == '_'
}
