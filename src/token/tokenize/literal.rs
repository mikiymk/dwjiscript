use std::fmt::format;
use std::{iter::Peekable, str::CharIndices};

use crate::character_patterns;

use crate::token::structs::literal::Literal;
use crate::token::structs::punctuator::Punctuator;
use crate::token::structs::{
    error::TokenizeError,
    token::{Token, TokenType},
};

use crate::token::tokenize::next_peeked_character;

use super::char_utf8_length;
use super::comment::{tokenize_multiline_comment, tokenize_singleline_comment};

pub fn tokenize_numeric_literal(chars: &mut Peekable<CharIndices>) -> Result<Token, TokenizeError> {
    match chars.peek() {
        Some((index, char @ '0')) => {
            let index = *index;
            let char = *char;
            next_peeked_character(chars)?;

            match chars.peek() {
                Some((index, 'b' | 'B')) => tokenize_binary_number_literal(chars),
                Some((index, 'o' | 'O')) => tokenize_octal_number_literal(chars),
                Some((index, 'x' | 'X')) => tokenize_hex_number_literal(chars),
                Some((index, '1'..='9')) => tokenize_legacy_octal_number_literal(chars),
                Some((next_index, char @ 'n')) => {
                    // 0n
                    let token_type = TokenType::Literal(Literal::DecimalBigInteger);
                    let start_position = index;
                    let end_position = next_index + char_utf8_length(*char);

                    next_peeked_character(chars);

                    Ok(Token::new(token_type, start_position, end_position))
                }
                Some((_, _)) => {
                    // 0
                    let token_type = TokenType::Literal(Literal::DecimalBigInteger);
                    let start_position = index;
                    let end_position = index + char_utf8_length(char);

                    Ok(Token::new(token_type, start_position, end_position))
                }

                _ => Err(format!("")),
            }
        }

        Some((index, '1'..='9' | '.')) => tokenize_decimal_number_literal(chars),

        _ => Err(format!("")),
    }
}

fn tokenize_decimal_number_literal(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    let mut token_type = None;
    let mut start_position = None;
    let mut end_position = None;

    let mut is_prev_separator = false;

    while let Some((index, char)) = chars.peek() {
        match char {
            '0'..='9' => {
                if start_position == None {
                    start_position = Some(*index);
                }

                end_position = Some(index + char_utf8_length(*char));
                is_prev_separator = false;
                next_peeked_character(chars)?;
            }
            '_' => {
                if is_prev_separator {
                    return Err(format!(
                        "error number cannot contain multiple adjacent underscores"
                    ));
                } else {
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }
            _ => break,
        }
    }

    if is_prev_separator {
        return Err(format!("error underscore can appear only between digits"));
    }

    if let Some((index, char @ '.')) = chars.peek() {
        next_peeked_character(chars)?;
        is_prev_separator = false;

        while let Some((index, char)) = chars.peek() {
            match char {
                '0'..='9' => {
                    if start_position == None {
                        start_position = Some(*index);
                    }

                    end_position = Some(index + char_utf8_length(*char));
                    is_prev_separator = false;
                    next_peeked_character(chars)?;
                }
                '_' => {
                    if is_prev_separator {
                        return Err(format!(
                            "error number cannot contain multiple adjacent underscores"
                        ));
                    } else {
                        is_prev_separator = true;
                        next_peeked_character(chars)?;
                    }
                }
                _ => break,
            }
        }
    }

    Token::try_new(token_type, start_position, end_position).ok_or(format!("err"))
}

fn tokenize_binary_number_literal(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    Err(format!(""))
}

fn tokenize_octal_number_literal(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    Err(format!(""))
}

fn tokenize_hex_number_literal(chars: &mut Peekable<CharIndices>) -> Result<Token, TokenizeError> {
    Err(format!(""))
}

fn tokenize_legacy_octal_number_literal(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    Err(format!(""))
}

#[cfg(test)]
mod test {
    use crate::token::structs::literal::Literal;
    use crate::token::structs::punctuator::Punctuator;
    use crate::token::structs::token::{Token, TokenType};

    use super::tokenize_numeric_literal;

    macro_rules! test_case {
        ($test_name:ident, $source:expr, $token_type:expr, $len:expr) => {
            #[test]
            fn $test_name() {
                let param = $source;
                let expected = Token::new(TokenType::Literal($token_type), 0, $len);

                let mut chars = param.char_indices().peekable();

                let result = tokenize_numeric_literal(&mut chars);

                assert_eq!(result, Ok(expected));
            }
        };
    }

    test_case!(dec_int, "1234", Literal::DecimalNumber, 4);
}
