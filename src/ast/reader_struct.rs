use std::{iter::Peekable, str::CharIndices};

use crate::token::structs::{
    context::{BraceContext, DivContext},
    token::Token,
};

#[derive(Debug, Clone)]
pub struct SourceCodeReader<'a> {
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> SourceCodeReader<'a> {
    pub fn new(source: &'a str) -> SourceCodeReader {
        SourceCodeReader {
            source,
            iter: source.char_indices().peekable(),
        }
    }

    pub fn read_next_token(&mut self, div: DivContext, brace: BraceContext) -> Token {
        todo!()
    }
}
