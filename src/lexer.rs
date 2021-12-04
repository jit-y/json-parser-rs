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

        if matches!(c, '"') {
            return self.build_string();
        }

        let tok = match c {
            '{' => Token::new(TokenType::LBrace, c),
            '}' => Token::new(TokenType::RBrace, c),
            '[' => Token::new(TokenType::LBracket, c),
            ']' => Token::new(TokenType::RBracket, c),
            ':' => Token::new(TokenType::Colon, c),
            ',' => Token::new(TokenType::Comma, c),
            '.' => Token::new(TokenType::Period, c),
            _ => return Err(anyhow!("invalid symbol")),
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

    fn build_unicode(&mut self) -> Result<u16> {
        let mut s = String::new();

        for _ in 0..4 {
            let c = self.next_char().ok_or(anyhow!(""))?;
            if !c.is_ascii_hexdigit() {
                return Err(anyhow!("Invalid char"));
            }

            s.push(c);
        }

        let res = u16::from_str_radix(s.as_str(), 16)?;

        Ok(res)
    }

    fn build_string(&mut self) -> Result<Token> {
        let mut res = String::new();
        let mut unicode_buf = vec![];

        self.next_char().ok_or(anyhow!("invalid char"))?;

        fn append_unicode(buf: &mut Vec<u16>, target: &mut String) -> Result<()> {
            if buf.is_empty() {
                return Ok(());
            }

            let s = String::from_utf16(buf)?;
            target.push_str(s.as_str());

            Ok(())
        }

        while let Some(c) = self.next_char() {
            match c {
                '"' => {
                    append_unicode(&mut unicode_buf, &mut res)?;
                    break;
                }
                '\\' => {
                    let c2 = self.next_char().ok_or(anyhow!("unexpected token"))?;
                    match c2 {
                        'u' => {
                            let u = self.build_unicode()?;
                            unicode_buf.push(u);
                        }
                        '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {
                            append_unicode(&mut unicode_buf, &mut res)?;
                            res.push('\\');
                            res.push(c2);
                        }
                        _ => {
                            append_unicode(&mut unicode_buf, &mut res)?;
                            res.push(c2);
                        }
                    }
                }
                _ => {
                    append_unicode(&mut unicode_buf, &mut res)?;
                    res.push(c)
                }
            }
        }

        Ok(Token::new(TokenType::String, res))
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
        let text = r#"[true, false, null, -1.0e+9, {"foo": "bar"}]"#;
        let mut l = Lexer::new(text);
        let expected = vec![
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
            Token::new(TokenType::String, "foo"),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::String, "bar"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::RBracket, "]"),
        ];

        for e in expected {
            let tok = l.read_token();

            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), e);
        }
    }

    fn test_tokenize_escaped_string() {
        let text = r#"\u1F600\u1F601\t\n\f "#;
        let mut l = Lexer::new(text);
        let expected = vec![
            Token::new(TokenType::String, "😀"),
            Token::new(TokenType::String, "😁"),
            Token::new(TokenType::String, "\\t"),
            Token::new(TokenType::String, "\\n"),
            Token::new(TokenType::String, "\\f"),
            Token::new(TokenType::String, " "),
        ];

        for e in expected {
            let tok = l.read_token();

            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), e);
        }
    }
}
