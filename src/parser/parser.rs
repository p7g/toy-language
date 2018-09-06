use super::{ TokenStream, Token, Keyword };
use std::fmt;

impl fmt::Debug for Box<Fn(Vec<AST>) -> AST> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[native code]")
    }
}

impl PartialEq for Box<Fn(Vec<AST>) -> AST> {
    fn eq(&self, _: &Box<Fn(Vec<AST>) -> AST>) -> bool {
        false
    }
}

impl Clone for Box<Fn(Vec<AST>) -> AST> {
    fn clone(&self) -> Box<Fn(Vec<AST>) -> AST> {
        Box::new(|a| AST::Program(a))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Number(f64),
    String_(String),
    Boolean(bool),
    Variable(String),
    Function {
        parameters: Vec<String>,
        body: Box<AST>,
        native: Option<Box<Fn(Vec<AST>) -> AST>>
    },
    Call {
        function: Box<AST>,
        arguments: Vec<AST>
    },
    If {
        condition: Box<AST>,
        then: Box<AST>,
        otherwise: Option<Box<AST>>
    },
    Assign {
        operator: String,
        left: Box<AST>,
        right: Box<AST>
    },
    Binary {
        operator: String,
        left: Box<AST>,
        right: Box<AST>
    },
    Program(Vec<AST>)
    /*Let {
        variables: Vec<(String, AST)>,
        body: Box<AST>
    }*/
}

pub struct Parser<'a> {
    token_stream: TokenStream<'a>
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Parser<'a> {
        Parser {
            token_stream
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut program: Vec<AST> = Vec::new();

        while !self.token_stream.eof() {
            program.push(self.parse_expression());
            if !self.token_stream.eof() {
                self.skip_punctuation(';');
            }
        }

        AST::Program(program)
    }

    fn parse_expression(&mut self) -> AST {
        let atom = self.parse_atom();
        let result = self.maybe_binary(atom, 0);
        if self.is_punctuation('(') {
            self.parse_call(result)
        }
        else {
            result
        }
    }

    fn parse_atom(&mut self) -> AST {
        let result = {
            if self.is_punctuation('(') {
                self.token_stream.next();
                let result = self.parse_expression();
                self.skip_punctuation(')');
                result
            }
            else if self.is_punctuation('{') {
                self.parse_program()
            }
            else if self.is_keyword(Keyword::If) {
                self.parse_if()
            }
            else if self.is_keyword(Keyword::True) || self.is_keyword(Keyword::False) {
                self.parse_boolean()
            }
            else if self.is_keyword(Keyword::Fn) {
                self.token_stream.next();
                self.parse_function()
            }
            else {
                let token = self.token_stream.next();
                match token {
                    Some(Token::Identifier(name)) => AST::Variable(name),
                    Some(Token::Number(number)) => AST::Number(number),
                    Some(Token::String_(string)) => AST::String_(string),
                    token => self.token_stream.panic(format!("Unexpected token: {:?}", token))
                }
            }
        };
        if self.is_punctuation('(') {
            self.parse_call(result)
        }
        else {
            result
        }
    }

    fn parse_program(&mut self) -> AST {
        let mut program = self.delimited_expressions('{', '}', ';');
        match program.len() {
            0 => AST::Boolean(false),
            1 => program.remove(0),
            _ => AST::Program(program)
        }
    }

    fn parse_call(&mut self, ast: AST) -> AST {
        AST::Call {
            function: Box::new(ast),
            arguments: self.delimited_expressions('(', ')', ',')
        }
    }

    fn parse_boolean(&mut self) -> AST {
        match self.token_stream.next() {
            Some(Token::Keyword(Keyword::True)) => AST::Boolean(true),
            Some(Token::Keyword(Keyword::False))=> AST::Boolean(false),
            otherwise => self.token_stream.panic(format!("Unkown bool {:?}", otherwise))
        }
    }

    fn parse_if(&mut self) -> AST {
        self.skip_keyword(Keyword::If);
        let condition = self.parse_expression();
        self.skip_keyword(Keyword::Then);
        let then = self.parse_expression();
        let otherwise = if self.is_keyword(Keyword::Else) {
            self.token_stream.next();
            Some(Box::new(self.parse_expression()))
        }
        else {
            None
        };
        AST::If {
            condition: Box::new(condition),
            then: Box::new(then),
            otherwise
        }
    }

    fn parse_function(&mut self) -> AST {
        AST::Function {
            parameters: self.delimited_identifiers('(', ')', ','),
            body: Box::new(self.parse_expression()),
            native: None
        }
    }

    fn parse_variable(&mut self) -> String {
        let identifier = self.token_stream.next();
        if let Some(Token::Identifier(name)) = identifier {
            name
        }
        else {
            self.token_stream.panic(format!("Expected variable name, got: {:?}", identifier))
        }
    }

    fn maybe_binary(&mut self, left: AST, prec_left: i8) -> AST {
        if let Some(operator) = self.is_operator() {
            let prec_right = precedence(operator.clone());
            if prec_right > prec_left {
                self.token_stream.next();
                let atom = self.parse_atom();
                let next_bin = self.maybe_binary(atom, prec_right);
                self.maybe_binary(match operator.clone().as_ref() {
                    "=" =>
                        AST::Assign {
                            operator,
                            left: Box::new(left),
                            right: Box::new(next_bin)
                        },
                    op =>
                        AST::Binary {
                            operator: op.to_string(),
                            left: Box::new(left),
                            right: Box::new(next_bin)
                        }
                }, prec_left)
            }
            else {
                left
            }
        }
        else {
            left
        }
    }

    fn delimited_expressions(&mut self, start: char, end: char, separator: char) -> Vec<AST> {
        let mut output: Vec<AST> = Vec::new();
        let mut first = true;

        self.skip_punctuation(start);

        while !self.token_stream.eof() {
            if self.is_punctuation(end) {
                break;
            }
            if first {
                first = false;
            }
            else {
                self.skip_punctuation(separator);
            }
            if self.is_punctuation(end) {
                break;
            }
            output.push(self.parse_expression());
        }

        self.skip_punctuation(end);
        output
    }

    fn delimited_identifiers(&mut self, start: char, end: char, separator: char) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut first = true;

        self.skip_punctuation(start);

        while !self.token_stream.eof() {
            if self.is_punctuation(end) {
                break;
            }
            if first {
                first = false;
            }
            else {
                self.skip_punctuation(separator);
            }
            if self.is_punctuation(end) {
                break;
            }
            output.push(self.parse_variable());
        }

        self.skip_punctuation(end);
        output
    }

    fn is_punctuation(&mut self, punc: char) -> bool {
        if let Some(Token::Punctuation(p)) = self.token_stream.peek() {
            punc == p
        }
        else {
            false
        }
    }

    fn is_keyword(&mut self, keyword: Keyword) -> bool {
        if let Some(Token::Keyword(k)) = self.token_stream.peek() {
            k == keyword
        }
        else {
            false
        }
    }

    fn is_operator(&mut self) -> Option<String> {
        if let Some(Token::Operator(op)) = self.token_stream.peek() {
            Some(op)
        }
        else {
            None
        }
    }

    fn skip_keyword(&mut self, keyword: Keyword) {
        if self.is_keyword(keyword.clone()) {
            self.token_stream.next();
        }
        else {
            self.token_stream.panic(format!("Expected keyword {:?}", keyword));
        }
    }

    fn skip_punctuation(&mut self, punc: char) {
        if self.is_punctuation(punc) {
            self.token_stream.next();
        }
        else {
            self.token_stream.panic(format!("Expected punctuation {}", punc));
        }
    }
}

fn precedence(op: String) -> i8 {
    match op.as_ref() {
        "=" => 1,
        "||" => 2,
        "&&" => 3,
        "<" | ">" | "<=" | ">=" | "==" | "!=" => 7,
        "+" | "-" => 10,
        "*" | "/" | "%" => 20,
        _ => panic!(format!("Unknown operator: {}", op))
    }
}
