//! プログラムの文字列からトークン列にするモジュールです。

use super::comment::Comment;
use super::punctuator::Punctuator;
use crate::to_source_string::ToSourceString;

pub struct TokenList {
    tokens: Vec<Token>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    WhiteSpace,
    LineTerminator,
    Comment(Comment),
    Identifier(String),
    Punctuator(Punctuator),
    Literal,
    Template,
}

impl Token {
    pub fn new_whitespace() -> Token {
        Token::WhiteSpace
    }
}

impl ToSourceString for Token {
    fn to_source_string(&self) -> String {
        match self {
            Self::WhiteSpace => " ".to_string(),
            Token::LineTerminator => "\n".to_string(),
            Token::Comment(c) => c.to_source_string(),
            Token::Identifier(_) => todo!(),
            Token::Punctuator(_) => todo!(),
            Token::Literal => todo!(),
            Token::Template => todo!(),
        }
    }
}
