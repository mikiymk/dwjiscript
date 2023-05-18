use std::{iter::Peekable, str::CharIndices};

use crate::character_patterns;

use crate::token::structs::{
    context::{BraceContext, DivContext},
    error::TokenizeError,
    token::Token,
};

use self::{
    line_terminator::tokenize_line_terminator, literal::tokenize_numeric_literal,
    punctuator::tokenize_punctuator, whitespace::tokenize_whitespace,
};

mod comment;
pub mod error;
mod line_terminator;
mod literal;
mod punctuator;
mod whitespace;

pub fn next_token(
    chars: &mut Peekable<CharIndices>,
    div: DivContext,
    brace: BraceContext,
) -> Result<Token, TokenizeError> {
    match chars.peek() {
        Some((_, character_patterns!(Whitespace))) => tokenize_whitespace(chars),
        Some((_, character_patterns!(LineTerminator))) => tokenize_line_terminator(chars),
        Some((_, character_patterns!(Punctuator))) => tokenize_punctuator(chars),
        Some((_, character_patterns!(Numeric))) => tokenize_numeric_literal(chars),

        Some((index, char)) => Err("".to_string()),
        None => Err("reach to EOF".to_string()),
    }
}

pub(self) fn next_peeked_character(
    chars: &mut Peekable<CharIndices>,
) -> Result<(usize, char), TokenizeError> {
    chars.next().ok_or(format!(
        "error occurred. peek is has next but next is not have next."
    ))
}

pub(self) fn char_utf8_length(char: char) -> usize {
    if char <= '\u{007F}' {
        1
    } else if char <= '\u{07FF}' {
        2
    } else if char <= '\u{FFFF}' {
        3
    } else {
        4
    }
}
