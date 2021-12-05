mod value;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};
use anyhow::anyhow;
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Peekable;
use thiserror::Error;
use value::Value;

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Error, Debug)]
enum ParserError {
    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),
    #[error("no token")]
    NoToken,
}

pub struct Parser<'c> {
    lexer: Peekable<Lexer<'c>>,
}

impl<'c> Parser<'c> {
    pub fn new(lexer: Lexer<'c>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Value> {
        let tok = self.next_token()?;

        match &tok.token_type {
            TokenType::Null => self.parse_null(),
            TokenType::String(val) => self.parse_string(val.clone()),
            TokenType::Number(val) => self.parse_number(*val),
            TokenType::LBracket => self.parse_array(),
            TokenType::LBrace => self.parse_object(),
            _ => return Err(ParserError::UnexpectedToken(tok)),
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        self.lexer.next().ok_or(ParserError::NoToken)
    }

    fn peek_token(&mut self) -> Result<&Token> {
        self.lexer.peek().ok_or(ParserError::NoToken)
    }

    fn parse_null(&mut self) -> Result<Value> {
        Ok(Value::Null)
    }

    fn parse_string(&mut self, val: String) -> Result<Value> {
        Ok(Value::String(val))
    }

    fn parse_number(&mut self, val: f64) -> Result<Value> {
        Ok(Value::Number(val))
    }

    fn parse_array(&mut self) -> Result<Value> {
        let mut res = vec![];

        while let Ok(tok) = self.peek_token() {
            match tok.token_type {
                TokenType::RBracket => {
                    self.next_token();
                    break;
                }
                TokenType::Comma => {
                    self.next_token();
                    if let Ok(tok) = self.peek_token() {
                        if tok.token_type == TokenType::RBracket {
                            let tok = self.next_token().expect("should exist");
                            return Err(ParserError::UnexpectedToken(tok));
                        }
                    }
                }
                _ => {
                    let v = self.parse()?;
                    res.push(v);
                }
            }
        }

        Ok(Value::Array(res))
    }

    fn parse_object(&mut self) -> Result<Value> {
        let mut res: HashMap<String, Value> = HashMap::new();

        let key_tok = self.next_token()?;
        let key = match key_tok.token_type {
            TokenType::String(v) => v,
            _ => return Err(ParserError::UnexpectedToken(key_tok)),
        };
        let colon_tok = self.next_token()?;
        match colon_tok.token_type {
            TokenType::Colon => {}
            _ => return Err(ParserError::UnexpectedToken(colon_tok)),
        };

        let val = self.parse()?;

        let rbrace_tok = self.next_token()?;
        match rbrace_tok.token_type {
            TokenType::RBrace => {}
            _ => return Err(ParserError::UnexpectedToken(rbrace_tok)),
        };

        res.insert(key, val);

        Ok(Value::Object(res))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tests: Vec<(&str, Value)> = vec![
            (r#""aaa""#, Value::String("aaa".to_string())),
            (r#"1e10"#, Value::Number(1e10_f64)),
            (
                r#"["aaa", 1e10]"#,
                Value::Array(vec![
                    Value::String("aaa".to_string()),
                    Value::Number(1e10_f64),
                ]),
            ),
            (
                r#"{"foo": ["bar", {"baz": "ok"}]}"#,
                Value::Object(HashMap::<_, _>::from_iter(IntoIter::new([(
                    "foo".to_string(),
                    Value::Array(vec![
                        Value::String("bar".to_string()),
                        Value::Object(HashMap::<_, _>::from_iter(IntoIter::new([(
                            "baz".to_string(),
                            Value::String("ok".to_string()),
                        )]))),
                    ]),
                )]))),
            ),
        ];

        for (t, e) in tests {
            let l = Lexer::new(t);
            let mut p = Parser::new(l);
            let v = p.parse();

            assert!(v.is_ok());
            assert_eq!(v.unwrap(), e);
        }
    }
}
