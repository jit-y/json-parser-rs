mod token;

use anyhow::{anyhow, Result};
use std::iter::Peekable;
use std::str::Chars;
use token::{Token, TokenType};

struct Lexer<'c> {
    chars: Peekable<Chars<'c>>,
}

impl<'c> Lexer<'c> {
    pub fn new(input: &'c str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn read_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let c = self.peek_char().ok_or(anyhow!("chars are run out"))?;

        if c.is_ascii_lowercase() {
            return self.build_keyword();
        }

        if matches!(c, '0'..='9' | '-') {
            return self.build_number();
        }

        let tok = match c {
            '{' => Token::new(TokenType::LBrace, c),
            '}' => Token::new(TokenType::RBrace, c),
            '[' => Token::new(TokenType::LBracket, c),
            ']' => Token::new(TokenType::RBracket, c),
            ':' => Token::new(TokenType::Colon, c),
            ',' => Token::new(TokenType::Comma, c),
            '.' => Token::new(TokenType::Period, c),
            _ => unimplemented!(),
        };

        self.next_char();

        Ok(tok)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn build_keyword(&mut self) -> Result<Token> {
        let mut res = String::new();

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_lowercase() {
                break;
            }

            res.push(*c);
            self.next_char();
        }

        Token::lookup_keyword(res.as_str()).ok_or(anyhow!("unknown keyword {}", res))
    }

    fn build_number(&mut self) -> Result<Token> {
        let mut res = String::new();

        while let Some(c) = self.peek_char() {
            if matches!(c, '-' | '0'..='9' | '.' | 'e' | 'E' | '+') {
                res.push(*c);

                self.next_char();
            } else {
                break;
            }
        }

        let v = res.parse::<f64>()?;

        Ok(Token::new(TokenType::Number(v), res))
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                None => break,
                Some(c) => {
                    if matches!(c, ' ' | '\t' | '\n' | '\r') {
                        self.next_char();
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = r#"[true, false, null, -1.0e+9, {:}]"#;
        let mut l = Lexer::new(text);
        let exptected = vec![
            Token::new(TokenType::LBracket, "["),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Null, "null"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Number(-1.0e+9_f64), "-1.0e+9"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::RBracket, "]"),
        ];

        for e in exptected {
            let tok = l.read_token();

            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), e);
        }
    }
}
