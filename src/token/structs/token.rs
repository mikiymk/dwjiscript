//! プログラムの文字列からトークン列にするモジュールです。

use super::comment::Comment;
use super::literal::Literal;
use super::punctuator::Punctuator;
use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TokenList {
    tokens: Vec<Token>,
}

impl TokenList {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenList { tokens }
    }

    pub fn append(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    token_start_index: usize,
    token_end_index: usize,
}

impl Token {
    pub fn try_new(
        token_type: Option<TokenType>,
        start_position: Option<usize>,
        end_position: Option<usize>,
    ) -> Option<Token> {
        let token_type = token_type?;
        let start_position = start_position?;
        let end_position = end_position?;

        Some(Token::new(token_type, start_position, end_position))
    }

    pub fn new(token_type: TokenType, start_position: usize, end_position: usize) -> Token {
        Token {
            token_type: token_type,
            token_start_index: start_position,
            token_end_index: end_position,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    /// https://tc39.es/ecma262/#sec-white-space
    WhiteSpace,
    LineTerminator,
    Comment(Comment),
    Identifier,
    Punctuator(Punctuator),
    Literal(Literal),
    Template,
}

impl TokenType {
    pub fn new_whitespace() -> TokenType {
        TokenType::WhiteSpace
    }
}

impl ToSourceString for TokenType {
    fn to_source_string(&self) -> String {
        match self {
            Self::WhiteSpace => " ".to_string(),
            TokenType::LineTerminator => "\n".to_string(),
            TokenType::Comment(c) => todo!(),
            TokenType::Identifier => todo!(),
            TokenType::Punctuator(_) => todo!(),
            TokenType::Literal(_) => todo!(),
            TokenType::Template => todo!(),
        }
    }
}
