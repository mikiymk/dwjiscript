//! プログラムの文字列からトークン列にするモジュールです。

use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Number(String),
    Identifier(String),

    // =
    Assign,

    // +
    Plus,

    // -
    Minus,

    // *
    Multiple,

    // **
    Exponent,

    // /
    Divide,

    // %
    Remainder,

    // +=
    PlusAssign,

    // -=
    MinusAssign,

    // *=
    MultipleAssign,

    // **=
    ExponentAssign,

    // /=
    DivideAssign,

    // %=
    RemainderAssign,

    // ++
    PlusPlus,

    // --
    MinusMinus,

    // ~
    BitNot,

    // |
    BitOr,

    // &
    BitAnd,

    // ^
    BitXor,

    // |=
    BitOrAssign,

    // &=
    BitAndAssign,

    // ^=
    BitXorAssign,

    // <<
    LeftShift,

    // >>
    RightShift,

    // >>>
    UnsignedRightShift,

    // <<=
    LeftShiftAssign,

    // >>=
    RightShiftAssign,

    // >>>=
    UnsignedRightShiftAssign,

    // ==
    Equal,

    // !=
    NotEqual,

    // ===
    StrictEqual,

    // !==
    StrictNotEqual,

    // >
    GreaterThan,

    // >=
    GreaterThanEqual,

    // <
    LessThan,

    // <=
    LessThanEqual,

    // ||
    Or,

    // &&
    And,

    // !
    Not,

    // ||=
    OrAssign,

    // &&=
    AndAssign,

    // ?
    Question,

    // ??
    QuestionQuestion,

    // ??=
    QuestionQuestionAssign,

    // ?.
    QuestionDot,

    // (
    ParenStart,

    // )
    ParenEnd,

    // [
    BracketStart,

    // ]
    BracketEnd,

    // {
    BraceStart,

    // }
    BraceEnd,

    // .
    Dot,

    // ,
    Comma,

    // :
    Colon,

    // ;
    Semicolon,

    // ...
    SpreadDots,

    // =>
    Arrow,

    // ''
    SingleQuoteStringLiteral,

    // ""
    DoubleQuoteStringLiteral,

    // ``
    BackQuoteTemplateLiteral,

    // //
    LineComment(String),

    // /* */
    BlockComment(String),

    //
    Eof,
}

/// 文字列を読み込んでトークン列を作成します。
///
/// ```
/// make_token_list("1 + 2");
/// ```
pub fn make_token_list(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            '=' | '+' | '-' | '*' | '/' | '%' | '<' | '>' | '|' | '&' | '^' | '~' | '!' | '?'
            | '.' | ',' | ':' | ';' | '\'' | '"' | '`' | '(' | ')' | '[' | ']' | '{' | '}' => {
                tokens.push(tokenize_operator(&mut chars)?)
            }
            '0'..='9' => tokens.push(tokenize_decimal_number(&mut chars)?),
            ' ' | '\n' => {
                // 空白文字
                let _ = chars.next();
            }
            c => return Err(format!("unknown character {}", c)),
        }
    }

    Ok(tokens)
}

/// 演算子をトークンにします。
fn tokenize_operator(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    if let Some(c) = chars.next() {
        match c {
            '+' => match chars.peek() {
                Some('+') => tokenize_const_token(chars, Token::PlusPlus),
                Some('=') => tokenize_const_token(chars, Token::PlusAssign),
                _ => Ok(Token::Plus),
            },
            '-' => match chars.peek() {
                Some('-') => tokenize_const_token(chars, Token::MinusMinus),
                Some('=') => tokenize_const_token(chars, Token::MinusAssign),
                _ => Ok(Token::Minus),
            },
            '*' => match chars.peek() {
                Some('*') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::ExponentAssign),
                        _ => Ok(Token::Exponent),
                    }
                }
                Some('=') => tokenize_const_token(chars, Token::MultipleAssign),
                _ => Ok(Token::Multiple),
            },
            '/' => match chars.peek() {
                Some('*') => tokenize_block_comment(chars),
                Some('/') => tokenize_line_comment(chars),
                Some('=') => tokenize_const_token(chars, Token::DivideAssign),
                _ => Ok(Token::Divide),
            },
            '%' => match chars.peek() {
                Some('=') => tokenize_const_token(chars, Token::RemainderAssign),
                _ => Ok(Token::Remainder),
            },
            '~' => Ok(Token::BitNot),
            '&' => match chars.peek() {
                Some('&') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::AndAssign),
                        _ => Ok(Token::And),
                    }
                }
                Some('=') => tokenize_const_token(chars, Token::BitAndAssign),
                _ => Ok(Token::BitAnd),
            },
            '|' => match chars.peek() {
                Some('|') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::OrAssign),
                        _ => Ok(Token::Or),
                    }
                }
                Some('=') => tokenize_const_token(chars, Token::BitOrAssign),
                _ => Ok(Token::BitOr),
            },
            '^' => match chars.peek() {
                Some('=') => tokenize_const_token(chars, Token::BitXorAssign),
                _ => Ok(Token::BitXor),
            },
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::StrictEqual),
                        _ => Ok(Token::Equal),
                    }
                }
                Some('>') => tokenize_const_token(chars, Token::Arrow),
                _ => Ok(Token::Assign),
            },
            '!' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::StrictNotEqual),
                        _ => Ok(Token::NotEqual),
                    }
                }
                _ => Ok(Token::Not),
            },
            '<' => match chars.peek() {
                Some('<') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::LeftShiftAssign),
                        _ => Ok(Token::LeftShift),
                    }
                }
                Some('=') => tokenize_const_token(chars, Token::LessThanEqual),
                _ => Ok(Token::LessThan),
            },
            '>' => match chars.peek() {
                Some('>') => {
                    chars.next();
                    match chars.peek() {
                        Some('>') => {
                            chars.next();
                            match chars.peek() {
                                Some('=') => {
                                    tokenize_const_token(chars, Token::UnsignedRightShiftAssign)
                                }
                                _ => Ok(Token::UnsignedRightShift),
                            }
                        }
                        Some('=') => tokenize_const_token(chars, Token::RightShiftAssign),
                        _ => Ok(Token::RightShift),
                    }
                }
                Some('=') => tokenize_const_token(chars, Token::GreaterThanEqual),
                _ => Ok(Token::GreaterThan),
            },
            '?' => match chars.peek() {
                Some('?') => {
                    chars.next();
                    match chars.peek() {
                        Some('=') => tokenize_const_token(chars, Token::QuestionQuestionAssign),
                        _ => Ok(Token::QuestionQuestion),
                    }
                }
                Some('.') => tokenize_const_token(chars, Token::QuestionDot),
                _ => Ok(Token::Question),
            },
            '(' => Ok(Token::ParenStart),
            ')' => Ok(Token::ParenEnd),
            '[' => Ok(Token::BracketStart),
            ']' => Ok(Token::BracketEnd),
            '{' => Ok(Token::BraceStart),
            '}' => Ok(Token::BraceEnd),
            '.' => match chars.peek() {
                Some('.') => {
                    chars.next();
                    match chars.peek() {
                        Some('.') => tokenize_const_token(chars, Token::SpreadDots),
                        _ => Ok(Token::Dot),
                    }
                }
                _ => Ok(Token::Dot),
            },
            ',' => Ok(Token::Comma),
            ':' => Ok(Token::Colon),
            ';' => Ok(Token::Semicolon),
            '\'' => tokenize_single_quote_string_literal(chars),
            '"' => tokenize_double_quote_string_literal(chars),
            '`' => tokenize_back_quote_template_literal(chars),
            c => Err(format!("{} is not operator", c)),
        }
    } else {
        Err("no operator, end of source".to_string())
    }
}

/// 文字イテレータの先頭から数字が連続する間、読み込みます。
fn tokenize_decimal_number(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut num_string = String::new();

    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            let c = chars.next().ok_or("peeked char is changed")?;
            num_string.push(c);
        } else {
            // 数字ではない文字の場合
            break;
        }
    }
    Ok(Token::Number(num_string))
}

fn tokenize_line_comment(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    chars.next();
    let str = chars.take_while(|c| c != &'\n').collect();
    Ok(Token::LineComment(str))
}

fn tokenize_block_comment(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut str = "/".to_string();

    let mut is_prev_asterisk = false;
    while let Some(c) = chars.next() {
        str.push(c);

        if is_prev_asterisk && c == '/' {
            break;
        } else if c == '*' {
            is_prev_asterisk = true;
        } else {
            is_prev_asterisk = false;
        }
    }

    Ok(Token::BlockComment(str))
}

fn tokenize_single_quote_string_literal(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut str = String::new();

    while let Some(c) = chars.next() {
        str.push(c);

        if c == '\\' {
            if let Some(c) = chars.next() {}
        }
        if c == '\'' {
            break;
        }
    }

    todo!();
}

fn tokenize_double_quote_string_literal(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    todo!();
}

fn tokenize_back_quote_template_literal(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    todo!();
}

/// 文字イテレータを一つ進めて渡されたトークンをそのまま返します。
/// ２操作を１つの式でするため
fn tokenize_const_token(chars: &mut Peekable<Chars>, token: Token) -> Result<Token, String> {
    chars.next().ok_or("next char expected, but reached end")?;
    Ok(token)
}

/// トークン列からトークンを１つ取り出します。
pub fn pop_token(tokens: &[Token]) -> (Token, &[Token]) {
    if tokens.is_empty() {
        return (Token::Eof, tokens);
    }
    let token = &tokens[0];
    let tokens = &tokens[1..];
    (token.clone(), tokens)
}

#[cfg(test)]
mod test {
    // ============  ============   ==========   ============
    // ============  ============  ============  ============
    //      ==       ==            ==        ==       ==
    //      ==       ============   ======            ==
    //      ==       ============       ======        ==
    //      ==       ==            ==        ==       ==
    //      ==       ============  ============       ==
    //      ==       ============   ==========        ==

    use crate::ast::token::make_token_list;
    use crate::ast::token::Token;

    #[test]
    fn test_tokenize_decimal_numbers() {
        let test_case =
            "1 2 3 4 5 6 7 8 9 10 20 50 100 5000 9999 999999999999999999999999999999999999999999";

        let result = make_token_list(test_case).unwrap();
        let expected = vec![
            Token::Number("1".to_string()),
            Token::Number("2".to_string()),
            Token::Number("3".to_string()),
            Token::Number("4".to_string()),
            Token::Number("5".to_string()),
            Token::Number("6".to_string()),
            Token::Number("7".to_string()),
            Token::Number("8".to_string()),
            Token::Number("9".to_string()),
            Token::Number("10".to_string()),
            Token::Number("20".to_string()),
            Token::Number("50".to_string()),
            Token::Number("100".to_string()),
            Token::Number("5000".to_string()),
            Token::Number("9999".to_string()),
            Token::Number("999999999999999999999999999999999999999999".to_string()),
        ];

        assert_eq!(result, expected, r#"tokenize numbers"#);
    }

    #[test]
    fn test_tokenize_operators() {
        let test_cases = vec![
            ("=", Token::Assign),
            ("+", Token::Plus),
            ("-", Token::Minus),
            ("*", Token::Multiple),
            ("**", Token::Exponent),
            ("/", Token::Divide),
            ("%", Token::Remainder),
            ("+=", Token::PlusAssign),
            ("-=", Token::MinusAssign),
            ("*=", Token::MultipleAssign),
            ("**=", Token::ExponentAssign),
            ("/=", Token::DivideAssign),
            ("%=", Token::RemainderAssign),
            ("++", Token::PlusPlus),
            ("--", Token::MinusMinus),
            ("~", Token::BitNot),
            ("|", Token::BitOr),
            ("&", Token::BitAnd),
            ("^", Token::BitXor),
            ("|=", Token::BitOrAssign),
            ("&=", Token::BitAndAssign),
            ("^=", Token::BitXorAssign),
            ("<<", Token::LeftShift),
            (">>", Token::RightShift),
            (">>>", Token::UnsignedRightShift),
            ("<<=", Token::LeftShiftAssign),
            (">>=", Token::RightShiftAssign),
            (">>>=", Token::UnsignedRightShiftAssign),
            ("==", Token::Equal),
            ("!=", Token::NotEqual),
            ("===", Token::StrictEqual),
            ("!==", Token::StrictNotEqual),
            (">", Token::GreaterThan),
            (">=", Token::GreaterThanEqual),
            ("<", Token::LessThan),
            ("<=", Token::LessThanEqual),
            ("||", Token::Or),
            ("&&", Token::And),
            ("!", Token::Not),
            ("||=", Token::OrAssign),
            ("&&=", Token::AndAssign),
            ("?", Token::Question),
            ("??", Token::QuestionQuestion),
            ("??=", Token::QuestionQuestionAssign),
            ("?.", Token::QuestionDot),
            ("(", Token::ParenStart),
            (")", Token::ParenEnd),
            ("[", Token::BracketStart),
            ("]", Token::BracketEnd),
            ("{", Token::BraceStart),
            ("}", Token::BraceEnd),
            (".", Token::Dot),
            (",", Token::Comma),
            (":", Token::Colon),
            (";", Token::Semicolon),
            ("...", Token::SpreadDots),
            ("=>", Token::Arrow),
        ];

        for (test_case, expected) in test_cases {
            let result = make_token_list(test_case);
            assert_eq!(result, Ok(vec![expected]), r#"tokenize {}"#, test_case);
        }
    }

    #[test]
    fn test_tokenize_string() {
        let test_cases = vec![
            ("'foo'", Token::Arrow),
            (r#""bar""#, Token::Arrow),
            ("`baz`", Token::Arrow),
            (r#"'aaa bbb "\'\" !?=+=))'"#, Token::Arrow),
            (r#""aaa bbb '\'\" !?=+=))""#, Token::Arrow),
            (r#"`aaa bbb '\'\" !?=+=))`"#, Token::Arrow),
            (
                r#" // foo bar
            "#,
                Token::Arrow,
            ),
            (r#"/* foo bar baz */"#, Token::Arrow),
        ];

        for (test_case, expected) in test_cases {
            let result = make_token_list(test_case);
            assert_eq!(result, Ok(vec![expected]), r#"tokenize {}"#, test_case);
        }
    }

    #[test]
    fn test_tokenize_template_literal() {
        let test_case = r#"`aaa ${2 + 3} bbb ${`d ${`ff ${5 - 4}`} e`} ccc`"#;
        let expected = vec![Token::Arrow];

        let result = make_token_list(test_case);
        assert_eq!(result, Ok(expected), r#"tokenize {}"#, test_case);
    }

    #[test]
    fn tokenize_number_and_operator_with_no_spaces() {
        let test_case = "3+2+=+5**++6";

        let result = make_token_list(test_case);
        let expected = vec![
            Token::Number("3".to_string()),
            Token::Plus,
            Token::Number("2".to_string()),
            Token::PlusAssign,
            Token::Plus,
            Token::Number("5".to_string()),
            Token::Exponent,
            Token::PlusPlus,
            Token::Number("6".to_string()),
        ];

        assert_eq!(result, Ok(expected), r#"tokenize 3+2+=+5**++6"#);
    }
}
