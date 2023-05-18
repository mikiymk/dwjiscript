use std::{iter::Peekable, str::CharIndices};

use crate::character_patterns;

use crate::token::structs::{
    error::TokenizeError,
    token::{Token, TokenType},
};

use crate::token::tokenize::next_peeked_character;

use super::char_utf8_length;

/// 空白の範囲をトークン化する。
pub(super) fn tokenize_whitespace(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    let mut start_index = None;
    let mut end_index = None;

    while let Some((index, char)) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        // "   " -> (0, 3)
        //  012
        match char {
            char @ character_patterns!(Whitespace) => {
                // 空白文字の場合
                if start_index == None {
                    // 最初の空白文字でインデックスを設定する
                    start_index = Some(*index);
                }
                end_index = Some(index + char_utf8_length(*char));

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
            _ => {
                // 空白文字ではない場合

                // "  a" -> (0, 2)
                //  012
                end_index = Some(*index);
                break;
            }
        }
    }

    let token_type = Some(TokenType::WhiteSpace);
    Token::try_new(token_type, start_index, end_index).ok_or(format!("err"))
}
