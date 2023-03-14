use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Number(String),
    Plus,
    Minus,
    Times,
    Divide,
    Eof,
}

pub fn make_token_list(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            '+' => {
                let _ = chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                let _ = chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                let _ = chars.next();
                tokens.push(Token::Times);
            }
            '/' => {
                let _ = chars.next();
                tokens.push(Token::Divide);
            }
            '0'..='9' => tokens.push(tokenize_decimal_number(&mut chars)?),
            ' ' | '\n' => {
                let _ = chars.next();
            }
            c => return Err(format!("unknown character {}", c)),
        }
    }

    Ok(tokens)
}

fn tokenize_decimal_number(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut num_string = String::new();

    while let Some(c) = chars.peek() {
        if matches!(c, '0'..='9') {
            let c = chars.next().ok_or("peeked char is changed")?;
            num_string.push(c);
        } else {
            break;
        }
    }
    Ok(Token::Number(num_string))
}

pub fn pop_token<'a>(tokens: &'a [Token]) -> (Token, &'a [Token]) {
    if tokens.len() == 0 {
        return (Token::Eof, tokens);
    }
    let token = &tokens[0];
    let tokens = &tokens[1..];
    (token.clone(), tokens)
}

#[cfg(test)]
mod test {
    use crate::ast::token::make_token_list;
    use crate::ast::token::Token;

    #[test]
    fn 一つの数字と四則演算をtoken列にする() {
        let test_case = "1 + 2 * 4 - 6 / 3";

        let result = make_token_list(test_case).unwrap();
        let expected = vec![
            Token::Number("1".to_string()),
            Token::Plus,
            Token::Number("2".to_string()),
            Token::Times,
            Token::Number("4".to_string()),
            Token::Minus,
            Token::Number("6".to_string()),
            Token::Divide,
            Token::Number("3".to_string()),
        ];

        assert_eq!(result, expected, r#"tokenize "1 + 2 * 4 - 6 / 3""#);
    }

    #[test]
    fn 二桁以上の数字を含む式をtoken列にする() {
        let test_case = "10 * 25 - 306";

        let result = make_token_list(test_case).unwrap();
        let expected = vec![
            Token::Number("10".to_string()),
            Token::Times,
            Token::Number("25".to_string()),
            Token::Minus,
            Token::Number("306".to_string()),
        ];

        assert_eq!(result, expected, r#"tokenize "10 * 25 * 306""#);
    }
}
