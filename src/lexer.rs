use crate::token::Token;
use crate::token::TokenKind;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    character: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            character: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.character = 0;
        } else {
            self.character = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        return self.input.as_bytes()[self.read_position];
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while is_letter(self.character) {
            self.read_char();
        }
        &self.input[start..self.position]
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while is_number(self.character) {
            self.read_char();
        }
        let number = &self.input[start..self.position];
        number.parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.character {
            0 => Token::new(TokenKind::Eof),
            b'+' => Token::new(TokenKind::Plus),
            b'-' => Token::new(TokenKind::Minus),
            b'*' => Token::new(TokenKind::Asterisk),
            b'/' => Token::new(TokenKind::Slash),
            b'<' => Token::new(TokenKind::LessThan),
            b'>' => Token::new(TokenKind::GreaterThan),
            b',' => Token::new(TokenKind::Comma),
            b':' => Token::new(TokenKind::Colon),
            b';' => Token::new(TokenKind::Semicolon),
            b'(' => Token::new(TokenKind::Lparen),
            b')' => Token::new(TokenKind::Rparen),
            b'[' => Token::new(TokenKind::Lbracket),
            b']' => Token::new(TokenKind::Rbracket),
            b'{' => Token::new(TokenKind::Lbrace),
            b'}' => Token::new(TokenKind::Rbrace),
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenKind::Equal)
                } else {
                    Token::new(TokenKind::Assign)
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenKind::NotEqual)
                } else {
                    Token::new(TokenKind::Bang)
                }
            }
            b'"' => {
                let position = self.position + 1;
                loop {
                    self.read_char();
                    if self.character == b'"' || self.character == 0 {
                        break;
                    }
                }
                let string = self.input[position..self.position].to_string();
                Token::new(TokenKind::String(string))
            }
            c if is_letter(c) => {
                let literal = self.read_identifier();
                let kind = TokenKind::from_letters(literal);
                return Token::new(kind);
            }
            c if is_number(c) => {
                let number = self.read_number();
                return Token::new(TokenKind::Int(number));
            }
            _ => return Token::new(TokenKind::Illegal),
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.character.is_ascii_whitespace() {
            self.read_char()
        }
    }
}

fn is_letter(character: u8) -> bool {
    character.is_ascii_alphabetic() || character == b'_'
}

fn is_number(character: u8) -> bool {
    character.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::token::TokenKind;

    fn test_next_token(input: &str, expected: &[TokenKind]) {
        let mut lexer = Lexer::new(input);

        for expected_token in expected.iter() {
            let token = lexer.next_token();

            assert_eq!(&token.kind, expected_token,);
        }
    }

    #[test]
    fn test_basic_tokens() {
        let input = "=+(){},;";
        let expected = vec![
            TokenKind::Assign,
            TokenKind::Plus,
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];
        test_next_token(input, &expected);
    }

    #[test]
    fn test_basic_source_code() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
x + y;
};

let result = add(five, ten);
"#;

        let expected = vec![
            TokenKind::Let,
            TokenKind::Ident(String::from("five")),
            TokenKind::Assign,
            TokenKind::Int(5),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident(String::from("ten")),
            TokenKind::Assign,
            TokenKind::Int(10),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident(String::from("add")),
            TokenKind::Assign,
            TokenKind::Function,
            TokenKind::Lparen,
            TokenKind::Ident(String::from("x")),
            TokenKind::Comma,
            TokenKind::Ident(String::from("y")),
            TokenKind::Rparen,
            TokenKind::Lbrace,
            TokenKind::Ident(String::from("x")),
            TokenKind::Plus,
            TokenKind::Ident(String::from("y")),
            TokenKind::Semicolon,
            TokenKind::Rbrace,
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident(String::from("result")),
            TokenKind::Assign,
            TokenKind::Ident(String::from("add")),
            TokenKind::Lparen,
            TokenKind::Ident(String::from("five")),
            TokenKind::Comma,
            TokenKind::Ident(String::from("ten")),
            TokenKind::Rparen,
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];
        test_next_token(input, &expected);
    }

    #[test]
    fn test_chapter_1_4() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
{"foo": "bar"}
"#;
        let expected = vec![
            TokenKind::Let,
            TokenKind::Ident(String::from("five")),
            TokenKind::Assign,
            TokenKind::Int(5),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident(String::from("ten")),
            TokenKind::Assign,
            TokenKind::Int(10),
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident(String::from("add")),
            TokenKind::Assign,
            TokenKind::Function,
            TokenKind::Lparen,
            TokenKind::Ident(String::from("x")),
            TokenKind::Comma,
            TokenKind::Ident(String::from("y")),
            TokenKind::Rparen,
            TokenKind::Lbrace,
            TokenKind::Ident(String::from("x")),
            TokenKind::Plus,
            TokenKind::Ident(String::from("y")),
            TokenKind::Semicolon,
            TokenKind::Rbrace,
            TokenKind::Semicolon,
            TokenKind::Let,
            TokenKind::Ident(String::from("result")),
            TokenKind::Assign,
            TokenKind::Ident(String::from("add")),
            TokenKind::Lparen,
            TokenKind::Ident(String::from("five")),
            TokenKind::Comma,
            TokenKind::Ident(String::from("ten")),
            TokenKind::Rparen,
            TokenKind::Semicolon,
            TokenKind::Bang,
            TokenKind::Minus,
            TokenKind::Slash,
            TokenKind::Asterisk,
            TokenKind::Int(5),
            TokenKind::Semicolon,
            TokenKind::Int(5),
            TokenKind::LessThan,
            TokenKind::Int(10),
            TokenKind::GreaterThan,
            TokenKind::Int(5),
            TokenKind::Semicolon,
            TokenKind::If,
            TokenKind::Lparen,
            TokenKind::Int(5),
            TokenKind::LessThan,
            TokenKind::Int(10),
            TokenKind::Rparen,
            TokenKind::Lbrace,
            TokenKind::Return,
            TokenKind::True,
            TokenKind::Semicolon,
            TokenKind::Rbrace,
            TokenKind::Else,
            TokenKind::Lbrace,
            TokenKind::Return,
            TokenKind::False,
            TokenKind::Semicolon,
            TokenKind::Rbrace,
            TokenKind::Int(10),
            TokenKind::Equal,
            TokenKind::Int(10),
            TokenKind::Semicolon,
            TokenKind::Int(10),
            TokenKind::NotEqual,
            TokenKind::Int(9),
            TokenKind::Semicolon,
            TokenKind::Lbrace,
            TokenKind::String(String::from("foo")),
            TokenKind::Colon,
            TokenKind::String(String::from("bar")),
            TokenKind::Rbrace,
            TokenKind::Eof,
        ];

        test_next_token(input, &expected);
    }

    #[test]
    fn text_strings() {
        let input = r#""foobar"
            "foo bar""#;
        let expected = vec![
            TokenKind::String("foobar".into()),
            TokenKind::String("foo bar".into()),
            TokenKind::Eof,
        ];

        test_next_token(input, &expected);
    }

    #[test]
    fn text_arrays() {
        let input = "[1, 2];";
        let expected = vec![
            TokenKind::Lbracket,
            TokenKind::Int(1),
            TokenKind::Comma,
            TokenKind::Int(2),
            TokenKind::Rbracket,
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];

        test_next_token(input, &expected);
    }
}
