use std::fmt::format;
use std::{iter::Peekable, str::CharIndices};

use crate::character_patterns;

use crate::token::structs::punctuator::Punctuator;
use crate::token::structs::{
    error::TokenizeError,
    token::{Token, TokenType},
};

use crate::token::tokenize::next_peeked_character;

use super::char_utf8_length;
use super::comment::{tokenize_multiline_comment, tokenize_singleline_comment};

pub fn tokenize_punctuator(chars: &mut Peekable<CharIndices>) -> Result<Token, TokenizeError> {
    let mut token_type = None;
    let mut start_index = None;
    let mut end_index = None;

    if let Some((index, char)) = chars.peek() {
        let char_size = char_utf8_length(*char);
        start_index = Some(*index);
        end_index = Some(index + char_size);

        match char {
            '{' => {
                token_type = Some(TokenType::Punctuator(Punctuator::LeftBrace));
            }

            '}' => {
                token_type = Some(TokenType::Punctuator(Punctuator::RightBrace));
            }

            '(' => {
                token_type = Some(TokenType::Punctuator(Punctuator::LeftParen));
            }

            ')' => {
                token_type = Some(TokenType::Punctuator(Punctuator::RightParen));
            }

            '[' => {
                token_type = Some(TokenType::Punctuator(Punctuator::LeftBracket));
            }

            ']' => {
                token_type = Some(TokenType::Punctuator(Punctuator::RightBracket));
            }

            '.' => {
                next_peeked_character(chars)?;

                if let Some((_, '.')) = chars.peek() {
                    next_peeked_character(chars)?;
                    if let Some((index, char @ '.')) = chars.peek() {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::DotThree));
                    } else {
                        return Err(format!("Syntax error '..' is not punctuator"));
                    }
                } else {
                    token_type = Some(TokenType::Punctuator(Punctuator::Dot));
                    return Token::try_new(token_type, start_index, end_index)
                        .ok_or(format!("err"));
                }
            }

            ';' => {
                token_type = Some(TokenType::Punctuator(Punctuator::SemiColon));
            }

            ',' => {
                token_type = Some(TokenType::Punctuator(Punctuator::Comma));
            }

            '<' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::LessThanEqual));
                    }

                    Some((index, char @ '<')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::LeftShiftAssign));
                            }

                            _ => {
                                token_type = Some(TokenType::Punctuator(Punctuator::LeftShift));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::LessThan));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '>' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::GreaterThanEqual));
                    }

                    Some((index, char @ '>')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::RightShiftAssign));
                            }

                            Some((index, char @ '>')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                next_peeked_character(chars)?;

                                match chars.peek() {
                                    Some((index, char @ '=')) => {
                                        end_index = Some(index + char_utf8_length(*char));
                                        token_type = Some(TokenType::Punctuator(
                                            Punctuator::UnsignedRightShiftAssign,
                                        ));
                                    }

                                    _ => {
                                        token_type = Some(TokenType::Punctuator(
                                            Punctuator::UnsignedRightShift,
                                        ));
                                        return Token::try_new(token_type, start_index, end_index)
                                            .ok_or(format!("err"));
                                    }
                                }
                            }

                            _ => {
                                token_type = Some(TokenType::Punctuator(Punctuator::RightShift));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::GreaterThan));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '=' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type = Some(TokenType::Punctuator(Punctuator::StrictEqual));
                            }

                            _ => {
                                token_type = Some(TokenType::Punctuator(Punctuator::Equal));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    Some((index, char @ '>')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::Arrow));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::Assign));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '!' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::StrictNotEqual));
                            }

                            _ => {
                                token_type = Some(TokenType::Punctuator(Punctuator::NotEqual));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::LogicalNot));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '+' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::PlusAssign));
                    }

                    Some((index, char @ '+')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::Increment));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::Plus));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '-' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::MinusAssign));
                    }

                    Some((index, char @ '-')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::Decrement));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::Minus));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '*' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::MultiplyAssign));
                    }

                    Some((index, char @ '*')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::ExponentiationAssign));
                            }

                            _ => {
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::Exponentiation));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::Multiply));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '/' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '/')) => {
                        return tokenize_singleline_comment(chars);
                    }

                    Some((index, char @ '*')) => {
                        return tokenize_multiline_comment(chars);
                    }

                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::DivideAssign));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::Divide));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '%' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::RemainderAssign));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::Remainder));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '&' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::BitAndAssign));
                    }

                    Some((index, char @ '&')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::LogicalAndAssign));
                            }

                            _ => {
                                token_type = Some(TokenType::Punctuator(Punctuator::LogicalAnd));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::BitAnd));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '|' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::BitOrAssign));
                    }

                    Some((index, char @ '|')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::LogicalOrAssign));
                            }

                            _ => {
                                token_type = Some(TokenType::Punctuator(Punctuator::LogicalOr));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::BitOr));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '^' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '=')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::BitXorAssign));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::BitXor));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            '~' => {
                token_type = Some(TokenType::Punctuator(Punctuator::BitNot));
            }

            '?' => {
                next_peeked_character(chars)?;

                match chars.peek() {
                    Some((index, char @ '?')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        next_peeked_character(chars)?;

                        match chars.peek() {
                            Some((index, char @ '=')) => {
                                end_index = Some(index + char_utf8_length(*char));
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::NullishCoalesceAssign));
                            }

                            _ => {
                                token_type =
                                    Some(TokenType::Punctuator(Punctuator::NullishCoalesce));
                                return Token::try_new(token_type, start_index, end_index)
                                    .ok_or(format!("err"));
                            }
                        }
                    }

                    Some((index, char @ '.')) => {
                        end_index = Some(index + char_utf8_length(*char));
                        token_type = Some(TokenType::Punctuator(Punctuator::OptionalChain));
                    }

                    _ => {
                        token_type = Some(TokenType::Punctuator(Punctuator::TernaryQuestion));
                        return Token::try_new(token_type, start_index, end_index)
                            .ok_or(format!("err"));
                    }
                }
            }

            ':' => {
                token_type = Some(TokenType::Punctuator(Punctuator::TernaryColon));
            }

            c => {
                // 改行文字ではない場合

                println!("{c}");
                panic!();
            }
        }

        next_peeked_character(chars)?;
    }

    Token::try_new(token_type, start_index, end_index).ok_or(format!("err"))
}

#[cfg(test)]
mod test {
    use crate::token::structs::punctuator::Punctuator;
    use crate::token::structs::token::{Token, TokenType};

    use super::tokenize_punctuator;

    macro_rules! test_case {
        ($test_name:ident, $source:expr, $token_type:expr, $len:expr) => {
            #[test]
            fn $test_name() {
                use crate::token::structs::punctuator::Punctuator::*;

                let param = $source;
                let expected = Token::new(TokenType::Punctuator($token_type), 0, $len);

                let mut chars = param.char_indices().peekable();

                let result = tokenize_punctuator(&mut chars);

                assert_eq!(result, Ok(expected));
            }
        };
    }

    test_case!(left_brace, "{abc", LeftBrace, 1);
    test_case!(right_brace, "}abc", RightBrace, 1);
    test_case!(left_paren, "(abc", LeftParen, 1);
    test_case!(right_paren, ")abc", RightParen, 1);
    test_case!(left_bracket, "[abc", LeftBracket, 1);
    test_case!(right_bracket, "]abc", RightBracket, 1);
    test_case!(dot, ".abc", Dot, 1);
    test_case!(dot_three, "...abc", DotThree, 3);
    test_case!(semicolon, ";abc", SemiColon, 1);
    test_case!(comma, ",abc", Comma, 1);
    test_case!(less_than, "<abc", LessThan, 1);
    test_case!(less_than_equal, "<=abc", LessThanEqual, 2);
    test_case!(left_shift, "<<abc", LeftShift, 2);
    test_case!(left_shift_assign, "<<=abc", LeftShiftAssign, 3);
    test_case!(greater_than, ">abc", GreaterThan, 1);
    test_case!(greater_than_equal, ">=abc", GreaterThanEqual, 2);
    test_case!(right_shift, ">>abc", RightShift, 2);
    test_case!(right_shift_assign, ">>=abc", RightShiftAssign, 3);
    test_case!(unsigned_right_shift, ">>>abc", UnsignedRightShift, 3);
    test_case!(
        unsigned_right_shift_assign,
        ">>>=abc",
        UnsignedRightShiftAssign,
        4
    );
    test_case!(assign, "=abc", Assign, 1);
    test_case!(equal, "==abc", Equal, 2);
    test_case!(strict_equal, "===abc", StrictEqual, 3);
    test_case!(arrow, "=>abc", Arrow, 2);
    test_case!(logical_not, "!abc", LogicalNot, 1);
    test_case!(not_equal, "!=abc", NotEqual, 2);
    test_case!(strict_not_equal, "!==abc", StrictNotEqual, 3);
    test_case!(plus, "+abc", Plus, 1);
    test_case!(plus_assign, "+=abc", PlusAssign, 2);
    test_case!(increment, "++abc", Increment, 2);
    test_case!(minus, "-abc", Minus, 1);
    test_case!(minus_assign, "-=abc", MinusAssign, 2);
    test_case!(decrement, "--abc", Decrement, 2);
    test_case!(multiply, "*abc", Multiply, 1);
    test_case!(multiply_assign, "*=abc", MultiplyAssign, 2);
    test_case!(exponentiation, "**abc", Exponentiation, 2);
    test_case!(exponentiation_assign, "**=abc", ExponentiationAssign, 3);
    test_case!(divide, "/abc", Divide, 1);
    test_case!(divide_assign, "/=abc", DivideAssign, 2);
    test_case!(remainder, "%abc", Remainder, 1);
    test_case!(remainder_assign, "%=abc", RemainderAssign, 2);
    test_case!(bit_and, "&abc", BitAnd, 1);
    test_case!(bit_and_assign, "&=abc", BitAndAssign, 2);
    test_case!(logical_and, "&&abc", LogicalAnd, 2);
    test_case!(logical_and_assign, "&&=abc", LogicalAndAssign, 3);
    test_case!(bit_or, "|abc", BitOr, 1);
    test_case!(bit_or_assign, "|=abc", BitOrAssign, 2);
    test_case!(logical_or, "||abc", LogicalOr, 2);
    test_case!(logical_or_assign, "||=abc", LogicalOrAssign, 3);
    test_case!(bit_xor, "^abc", BitXor, 1);
    test_case!(bit_xor_assign, "^=abc", BitXorAssign, 2);
    test_case!(bit_not, "~abc", BitNot, 1);
    test_case!(ternary_colon, ":abc", TernaryColon, 1);
    test_case!(ternary_question, "?abc", TernaryQuestion, 1);
    test_case!(nullish_coalesce, "??abc", NullishCoalesce, 2);
    test_case!(nullish_coalesce_assign, "??=abc", NullishCoalesceAssign, 3);
    test_case!(optional_chain, "?.abc", OptionalChain, 2);
}
