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
                Some((_, 'b' | 'B')) => tokenize_binary_number_literal(chars),
                Some((_, 'o' | 'O')) => tokenize_octal_number_literal(chars),
                Some((_, 'x' | 'X')) => tokenize_hex_number_literal(chars),
                Some((_, '1'..='9')) => tokenize_legacy_octal_number_literal(chars),
                Some((_, _)) => tokenize_decimal_number_literal(chars),

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
    let mut token_type = Literal::BinaryInteger;
    let mut start_position = None;
    let mut end_position = None;

    let mut is_prev_separator = false;

    match chars.peek() {
        Some((index, 'b' | 'B')) => {
            start_position = Some(index - char_utf8_length('0'));
            next_peeked_character(chars);
        }
        _ => {
            return Err(format!("err"));
        }
    }

    while let Some((index, char)) = chars.peek() {
        match char {
            '0' | '1' => {
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
                    end_position = Some(index + char_utf8_length(*char));
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }

            'n' => {
                if is_prev_separator {
                    return Err(format!("error"));
                } else {
                    end_position = Some(index + char_utf8_length(*char));
                    token_type = Literal::BinaryBigInteger;
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }

            _ => {
                if is_prev_separator {
                    return Err(format!("error"));
                } else {
                    break;
                }
            }
        }
    }

    Token::try_new(
        Some(TokenType::Literal(token_type)),
        start_position,
        end_position,
    )
    .ok_or(format!("err"))
}

fn tokenize_octal_number_literal(
    chars: &mut Peekable<CharIndices>,
) -> Result<Token, TokenizeError> {
    let mut token_type = Literal::OctalInteger;
    let mut start_position = None;
    let mut end_position = None;

    let mut is_prev_separator = false;

    match chars.peek() {
        Some((index, 'o' | 'O')) => {
            start_position = Some(index - char_utf8_length('0'));
            next_peeked_character(chars);
        }
        _ => {
            return Err(format!("err"));
        }
    }

    while let Some((index, char)) = chars.peek() {
        match char {
            '0'..='7' => {
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
                    end_position = Some(index + char_utf8_length(*char));
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }

            'n' => {
                if is_prev_separator {
                    return Err(format!("error"));
                } else {
                    end_position = Some(index + char_utf8_length(*char));
                    token_type = Literal::OctalBigInteger;
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }

            _ => {
                if is_prev_separator {
                    return Err(format!("error"));
                } else {
                    break;
                }
            }
        }
    }

    Token::try_new(
        Some(TokenType::Literal(token_type)),
        start_position,
        end_position,
    )
    .ok_or(format!("err"))
}

fn tokenize_hex_number_literal(chars: &mut Peekable<CharIndices>) -> Result<Token, TokenizeError> {
    let mut token_type = Literal::HexInteger;
    let mut start_position = None;
    let mut end_position = None;

    let mut is_prev_separator = false;

    match chars.peek() {
        Some((index, 'x' | 'X')) => {
            start_position = Some(index - char_utf8_length('0'));
            next_peeked_character(chars);
        }
        _ => {
            return Err(format!("err"));
        }
    }

    while let Some((index, char)) = chars.peek() {
        match char {
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
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
                    end_position = Some(index + char_utf8_length(*char));
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }

            'n' => {
                if is_prev_separator {
                    return Err(format!("error"));
                } else {
                    end_position = Some(index + char_utf8_length(*char));
                    token_type = Literal::HexBigInteger;
                    is_prev_separator = true;
                    next_peeked_character(chars)?;
                }
            }

            _ => {
                if is_prev_separator {
                    return Err(format!("error"));
                } else {
                    break;
                }
            }
        }
    }

    Token::try_new(
        Some(TokenType::Literal(token_type)),
        start_position,
        end_position,
    )
    .ok_or(format!("err"))
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

    macro_rules! test_case_fail {
        ($test_name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let param = $source;
                let expected = $expected.to_string();

                let mut chars = param.char_indices().peekable();

                let result = tokenize_numeric_literal(&mut chars);

                assert_eq!(result, Err(expected));
            }
        };
    }

    test_case!(dec_int_zero, "0", Literal::DecimalInteger, 1);
    test_case!(dec_int, "1234", Literal::DecimalInteger, 4);
    test_case!(dec_int_sep, "12_34", Literal::DecimalInteger, 5);
    test_case!(dec_point, "12.34", Literal::DecimalPointNumber, 5);
    test_case!(dec_exp_lower, "12e34", Literal::DecimalExponentNumber, 5);
    test_case!(dec_exp_upper, "12E34", Literal::DecimalExponentNumber, 5);
    test_case!(
        dec_point_exp,
        "12.34e56",
        Literal::DecimalPointExponentNumber,
        8
    );
    test_case!(
        dec_point_exp_sep,
        "1_2.3_4e5_6",
        Literal::DecimalPointExponentNumber,
        11
    );
    test_case!(dec_bigint_zero, "0n", Literal::DecimalBigInteger, 2);
    test_case!(dec_bigint, "1234n", Literal::DecimalBigInteger, 5);
    test_case!(dec_bigint_sep, "12_34n", Literal::DecimalBigInteger, 6);
    test_case!(bin_int_lower, "0b1101", Literal::BinaryInteger, 6);
    test_case!(bin_int_upper, "0B1101", Literal::BinaryInteger, 6);
    test_case!(bin_int_sep, "0b11_01", Literal::BinaryInteger, 7);
    test_case!(bin_int_zero, "0B0", Literal::BinaryInteger, 3);
    test_case!(bin_bigint_lower, "0b1101n", Literal::BinaryBigInteger, 7);
    test_case!(bin_bigint_upper, "0B1101n", Literal::BinaryBigInteger, 7);
    test_case!(bin_bigint_sep, "0B11_01n", Literal::BinaryBigInteger, 8);
    test_case!(bin_bigint_zero, "0b0n", Literal::BinaryBigInteger, 4);
    test_case!(oct_int_lower, "0o1267", Literal::OctalInteger, 6);
    test_case!(oct_int_upper, "0O1267", Literal::OctalInteger, 6);
    test_case!(oct_int_sep, "0O12_67", Literal::OctalInteger, 7);
    test_case!(oct_int_zero, "0o0", Literal::OctalInteger, 3);
    test_case!(oct_bigint_lower, "0o1267n", Literal::OctalBigInteger, 7);
    test_case!(oct_bigint_upper, "0O1267n", Literal::OctalBigInteger, 7);
    test_case!(oct_bigint_sep, "0O12_67n", Literal::OctalBigInteger, 8);
    test_case!(oct_bigint_zero, "0o0n", Literal::OctalBigInteger, 4);
    test_case!(hex_int_lower, "0x12ef", Literal::HexInteger, 6);
    test_case!(hex_int_upper, "0X12ef", Literal::HexInteger, 6);
    test_case!(hex_int_sep, "0x12_ef", Literal::HexInteger, 7);
    test_case!(hex_int_zero, "0x0", Literal::HexInteger, 3);
    test_case!(hex_bigint_lower, "0x12efn", Literal::HexBigInteger, 7);
    test_case!(hex_bigint_upper, "0S12efn", Literal::HexBigInteger, 7);
    test_case!(hex_bigint_sep, "0x12_efn", Literal::HexBigInteger, 8);
    test_case!(hex_bigint_zero, "0x0n", Literal::HexBigInteger, 4);
    test_case!(legacy_octal_int_1, "01267", Literal::LegacyOctalInteger, 5);
    test_case!(legacy_octal_int_2, "01268", Literal::DecimalInteger, 5);
}
