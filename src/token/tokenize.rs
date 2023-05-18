use std::{iter::Peekable, str::CharIndices};

use super::structs::{
    punctuator::Punctuator,
    token::{Token, TokenList, TokenType},
};

pub mod error;

macro_rules! whitespace_patterns {
    () => {
        // 空白文字にマッチする全てのパターン
        '\u{0009}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{FEFF}'
            | '\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205f}' | '\u{3000}'
    };
}

/// | code point | name                | abbr  |
/// | ---------- | ------------------- | ----- |
/// | U+000A     | LINE FEED           | \<LF> |
/// | U+000D     | CARRIAGE RETURN     | \<CR> |
/// | U+2028     | LINE SEPARATOR      | \<LS> |
/// | U+2029     | PARAGRAPH SEPARATOR | \<PS> |
macro_rules! line_terminator_patterns {
    () => {
        // 改行文字にマッチする全てのパターン
        '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}'
    };

    (No CR) => {
        // <CR>を除いたパターン
        '\u{000A}' | '\u{2028}' | '\u{2029}'
    };
}

/// | code point | name                | abbr  |
/// | ---------- | ------------------- | ----- |
/// | U+000A     | LINE FEED           | \<LF> |
/// | U+000D     | CARRIAGE RETURN     | \<CR> |
/// | U+2028     | LINE SEPARATOR      | \<LS> |
/// | U+2029     | PARAGRAPH SEPARATOR | \<PS> |
macro_rules! punctuator_patterns {
    () => {
        // 割り算文字、右波括弧以外の全ての演算子の開始文字
        '{' | '('
            | ')'
            | '['
            | ']'
            | '.'
            | ';'
            | ','
            | '<'
            | '>'
            | '='
            | '!'
            | '+'
            | '-'
            | '*'
            | '%'
            | '&'
            | '|'
            | '^'
            | '~'
            | '?'
    };

    (No CR) => {
        // <CR>を除いたパターン
        '\u{000A}' | '\u{2028}' | '\u{2029}'
    };
}

/// 文字列を読み込んでトークン列を作成します。
///
/// ```
/// make_token_list("1 + 2");
/// ```
pub fn tokenize(source: &str) -> Result<TokenList, String> {
    let mut chars = source.char_indices().peekable();
    let mut list = Vec::new();

    while let Some((index, char)) = chars.peek() {
        let index = *index;
        let token = match char {
            whitespace_patterns!() => tokenize_whitespace(&mut chars),
            line_terminator_patterns!() => tokenize_line_terminator(&mut chars),
            punctuator_patterns!() => tokenize_punctuator(&mut chars),
            // コメント、正規表現リテラル、割り算記号で場合分けをする
            '/' => {
                next_peeked_character(&mut chars)?;

                match chars.peek() {
                    Some((_, '/')) => tokenize_singleline_comment(&mut chars),
                    Some((_, '*')) => tokenize_multiline_comment(&mut chars),

                    Some((i, '=')) => {
                        let i = *i;
                        next_peeked_character(&mut chars)?;

                        Ok(Token::new(
                            &TokenType::Punctuator(Punctuator::DivideAssign),
                            index,
                            i + 1,
                        ))
                    }

                    _ => Ok(Token::new(
                        &TokenType::Punctuator(Punctuator::Divide),
                        index,
                        index + 1,
                    )),
                }
            }

            _ => {
                println!("{:?}", list);
                panic!();
            }
        }?;

        list.push(token);
    }

    Ok(TokenList::new(list))
}

/// 空白の範囲をトークン化する。
fn tokenize_whitespace(chars: &mut Peekable<CharIndices>) -> Result<Token, String> {
    let mut start_index = None;
    let mut end_index = None;

    while let Some((index, char)) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        // "   " -> (0, 3)
        //  012
        end_index = Some(index + 1);
        match char {
            whitespace_patterns!() => {
                // 空白文字の場合
                if start_index == None {
                    // 最初の空白文字でインデックスを設定する
                    start_index = Some(*index);
                }

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

    start_index
        .zip(end_index)
        .map(|(s, e)| Token::new(&TokenType::WhiteSpace, s, e))
        .ok_or(format!("err"))
}

fn tokenize_line_terminator(chars: &mut Peekable<CharIndices>) -> Result<Token, String> {
    let mut start_index = None;
    let mut end_index = None;

    if let Some((index, char @ line_terminator_patterns!())) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        match char {
            // <CR><LF>のパターンは１つの改行とみなす
            '\u{000D}' => {
                start_index = Some(*index);
                next_peeked_character(chars)?;

                if let Some((index, '\u{000A}')) = chars.peek() {
                    end_index = Some(index + 1);
                }
            }
            line_terminator_patterns!(No CR) => {
                start_index = Some(*index);
                end_index = Some(index + 1);

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
            _ => {
                // 改行文字ではない場合
            }
        }
    }

    start_index
        .zip(end_index)
        .map(|(s, e)| Token::new(&TokenType::LineTerminator, s, e))
        .ok_or(format!("err"))
}

fn tokenize_singleline_comment(chars: &mut Peekable<CharIndices>) -> Result<Token, String> {
    let mut start_index = None;
    let mut end_index = None;

    if let Some((index, char @ line_terminator_patterns!())) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        match char {
            // <CR><LF>のパターンは１つの改行とみなす
            '\u{000D}' => {
                start_index = Some(*index);
                next_peeked_character(chars)?;

                if let Some((index, '\u{000A}')) = chars.peek() {
                    end_index = Some(index + 1);
                }
            }
            line_terminator_patterns!(No CR) => {
                start_index = Some(*index);
                end_index = Some(index + 1);

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
            _ => {
                // 改行文字ではない場合
            }
        }
    }

    start_index
        .zip(end_index)
        .map(|(s, e)| Token::new(&TokenType::LineTerminator, s, e))
        .ok_or(format!("err"))
}

fn tokenize_multiline_comment(chars: &mut Peekable<CharIndices>) -> Result<Token, String> {
    let mut start_index = None;
    let mut end_index = None;

    if let Some((index, char @ line_terminator_patterns!())) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        match char {
            // <CR><LF>のパターンは１つの改行とみなす
            '\u{000D}' => {
                start_index = Some(*index);
                next_peeked_character(chars)?;

                if let Some((index, '\u{000A}')) = chars.peek() {
                    end_index = Some(index + 1);
                }
            }
            line_terminator_patterns!(No CR) => {
                start_index = Some(*index);
                end_index = Some(index + 1);

                // 次の文字に進む
                next_peeked_character(chars)?;
            }
            _ => {
                // 改行文字ではない場合
            }
        }
    }

    start_index
        .zip(end_index)
        .map(|(s, e)| Token::new(&TokenType::LineTerminator, s, e))
        .ok_or(format!("err"))
}

/// 空白の範囲をトークン化する。
fn tokenize_punctuator(chars: &mut Peekable<CharIndices>) -> Result<Token, String> {
    let mut start_index = None;
    let mut end_index = None;

    while let Some((index, char)) = chars.peek() {
        // 次の文字がない場合のインデックスを指定しておく
        // "   " -> (0, 3)
        //  012
        end_index = Some(index + 1);
        match char {
            whitespace_patterns!() => {
                // 空白文字の場合
                if start_index == None {
                    // 最初の空白文字でインデックスを設定する
                    start_index = Some(*index);
                }

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

    start_index
        .zip(end_index)
        .map(|(s, e)| Token::new(&TokenType::WhiteSpace, s, e))
        .ok_or(format!("err"))
}

fn next_peeked_character(chars: &mut Peekable<CharIndices>) -> Result<(usize, char), String> {
    chars.next().ok_or(format!(
        "error occurred. peek is has next but next is not have next."
    ))
}
