use std::{iter::Peekable, str::CharIndices};

use crate::character_patterns;

use crate::token::structs::{
    error::TokenizeError,
    token::{Token, TokenType},
};

use crate::token::tokenize::next_peeked_character;

use super::char_utf8_length;

pub fn tokenize_line_terminator(chars: &mut Peekable<CharIndices>) -> Result<Token, TokenizeError> {
    let mut start_index = None;
    let mut end_index = None;

    if let Some((index, char @ character_patterns!(LineTerminator))) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        match char {
            // <CR><LF>のパターンは１つの改行とみなす
            '\u{000D}' => {
                start_index = Some(*index);
                next_peeked_character(chars)?;

                if let Some((index, char @ '\u{000A}')) = chars.peek() {
                    end_index = Some(index + char_utf8_length(*char));
                }
            }
            char @ character_patterns!(LineTerminator without CR) => {
                start_index = Some(*index);
                end_index = Some(index + char_utf8_length(*char));

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
            _ => {
                // 改行文字ではない場合
            }
        }
    }

    let token_type = Some(TokenType::LineTerminator);
    Token::try_new(token_type, start_index, end_index).ok_or(format!("err"))
}
