#[derive(Debug, PartialEq)]
pub enum TokenType {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Period,

    Null,
    True,
    False,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new<L>(token_type: TokenType, literal: L) -> Self
    where
        L: ToString,
    {
        Self {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn lookup_keyword(literal: &str) -> Option<Self> {
        let tt = match literal {
            "null" => TokenType::Null,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => return None,
        };

        Some(Self::new(tt, literal))
    }
}
