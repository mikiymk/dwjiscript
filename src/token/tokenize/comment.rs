use std::{iter::Peekable, str::CharIndices};

use crate::character_patterns;

use crate::token::structs::comment::Comment;
use crate::token::structs::{
    error::TokenizeError,
    token::{Token, TokenType},
};

use crate::token::tokenize::next_peeked_character;

use super::char_utf8_length;

/// 一行コメント
pub(super) fn tokenize_singleline_comment(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    let mut start_index = None;
    let mut end_index = None;

    while let Some((index, char)) = chars.peek() {
        match char {
            char @ character_patterns!(LineTerminator) => {
                // 改行文字の場合
                end_index = Some(*index);
                break;
            }

            _ => {
                if start_index == None {
                    // 最初の空白文字でインデックスを設定する
                    start_index = Some(index - char_utf8_length('/'));
                }
                // 次の文字がない場合のインデックスを指定しておく
                end_index = Some(index - char_utf8_length(*char));

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
        }
    }

    let token_type = Some(TokenType::WhiteSpace);
    Token::try_new(token_type, start_index, end_index).ok_or(format!("err"))
}

/// 複数行コメント
pub(super) fn tokenize_multiline_comment(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    let mut start_index = None;
    let mut end_index = None;

    let mut is_prev_asterisk = false;

    // 最初の*を飛ばす (/*/で終了するのを防ぐ)
    match chars.next() {
        Some((_, '*')) => (),
        _ => return Err(format!("")),
    }

    while let Some((index, char)) = chars.peek() {
        match char {
            '/' => {
                // 前が*ならコメントを終了する
                if is_prev_asterisk {
                    end_index = Some(index + char_utf8_length(*char));
                    break;
                }
            }

            '*' => {
                // 次が/ならコメントを終了する
                is_prev_asterisk = true;

                // 次の文字に進む
                next_peeked_character(chars)?;
            }

            _ => {
                is_prev_asterisk = false;

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
        }
    }

    let token_type = Some(TokenType::Comment(Comment::SingleLineComment));
    Token::try_new(token_type, start_index, end_index).ok_or(format!("err"))
}
