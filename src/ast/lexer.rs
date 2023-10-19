use crate::ast::token::{Token, TokenType};

pub struct Lexer {
    pub(crate) input: String,
    pub(crate) position: usize,      // current position in input (points to current char)
    pub(crate) read_position: usize, // current reading position in input (after current char)
    pub(crate) ch: char              // current char under examination
}

impl Lexer {
    pub fn new(&mut self, input: String) {
        self.input = input;
        self.read_char();
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = char::from(0);
        } else {
            // self.ch = self.input[self.read_position]
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_white_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        return if self.read_position >= self.input.len() {
            char::from(0)
        } else {
            self.input.chars().nth(self.read_position).unwrap_or('\0')
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.is_letter(self.ch) {
            self.read_char()
        }
        return self.input[pos..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.is_digit(self.ch) {
            self.read_char()
        }
        return self.input[pos..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' {
                break
            }
        }
        return self.input[pos..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token { token_type: TokenType::ILLEGAL, literal: "".to_string() };
        self.skip_white_space();
        match self.ch {
            '+' => tok = Token::new_token(TokenType::PLUS, String::from(self.ch)),
            '-' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '/' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '*' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '<' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '>' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            ';' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            ',' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '{' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '}' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '(' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            ')' => tok = Token::new_token(TokenType::MINUS, String::from(self.ch)),
            '"' => {
                tok.token_type = TokenType::STRING;
                tok.literal = self.read_string();
            },
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok.token_type = TokenType::EQ;
                    tok.literal = ch.to_string() + &self.ch.to_string();
                } else {
                    tok = Token::new_token(TokenType::ASSIGN, String::from(self.ch));
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = Token::new_token(TokenType::NOT_EQ,ch.to_string() + &self.ch.to_string());
                } else {
                    tok = Token::new_token(TokenType::BANG, String::from(self.ch));
                }
            },
            '\0' => {
                tok.token_type = TokenType::EOF
            },
            _ => {
                if self.is_letter(self.ch) {
                    tok.literal = self.read_identifier();
                    tok.token_type = TokenType::lookup_ident(&tok.literal);
                    return tok;
                } else if self.is_digit(self.ch) {
                    tok.literal = self.read_number();
                    tok.token_type = TokenType::INT;
                    return tok;
                } else {
                    tok = Token::new_token(TokenType::ILLEGAL, String::from(self.ch));
                    return tok;
                }
            }
        }

        self.read_char();
        return tok;
    }


    fn is_letter(&self, ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(&self, ch: char) -> bool {
        return '0' <= ch && ch <= '9'
    }
}