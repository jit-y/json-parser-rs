mod value;

use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};
use anyhow::{anyhow, Result};
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Peekable;
use value::Value;

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
        match self.next_token() {
            None => return Err(anyhow!("err")),
            Some(tok) => match &tok.token_type {
                TokenType::Null => self.parse_null(),
                TokenType::String(val) => self.parse_string(val.clone()),
                TokenType::Number(val) => self.parse_number(*val),
                TokenType::LBracket => self.parse_array(),
                TokenType::LBrace => self.parse_object(),
                _ => return Err(anyhow!("unexpected token {:?}", tok)),
            },
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek()
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

        while let Some(tok) = self.peek_token() {
            match tok.token_type {
                TokenType::RBracket => {
                    self.next_token();
                    break;
                }
                TokenType::Comma => {
                    self.next_token();
                    if let Some(tok) = self.peek_token() {
                        if tok.token_type == TokenType::RBracket {
                            return Err(anyhow!("Invalid token {}", tok.literal));
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

        let key_tok = self.next_token().ok_or(anyhow!("err1"))?;
        let key = match key_tok.token_type {
            TokenType::String(v) => v,
            _ => return Err(anyhow!("err2")),
        };
        let colon_tok = self.next_token().ok_or(anyhow!("err3"))?;
        match colon_tok.token_type {
            TokenType::Colon => {}
            _ => return Err(anyhow!("err4")),
        };

        let val = self.parse()?;

        let rbrace_tok = self.next_token().ok_or(anyhow!("err5"))?;
        match rbrace_tok.token_type {
            TokenType::RBrace => {}
            _ => return Err(anyhow!("err4")),
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
