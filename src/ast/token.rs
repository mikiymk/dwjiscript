#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Times,
    Divide,
    Eof,
}

pub fn make_token_list(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for c in source.chars() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Times),
            '/' => tokens.push(Token::Divide),
            c => tokens.push(Token::Number(c.into())),
        }
    }

    tokens
}

pub fn pop_token<'a>(tokens: &'a [Token]) -> (Token, &'a [Token]) {
    if tokens.len() == 0 {
        return (Token::Eof, tokens);
    }
    let token = &tokens[0];
    let tokens = &tokens[1..];
    (*token, tokens)
}

#[cfg(test)]
mod test {
    use crate::ast::token::make_token_list;
    use crate::ast::token::Token;

    #[test]
    fn 一つの数字と四則演算をtoken列にする() {
        let test_case = "1 + 2 * 4 - 6 / 3";

        let result = make_token_list(test_case);
        let expected = vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Times,
            Token::Number(4),
            Token::Minus,
            Token::Number(6),
            Token::Divide,
            Token::Number(3),
        ];

        assert_eq!(result, expected, r#"tokenize "1 + 2 * 4 - 6 / 3""#);
    }

    #[test]
    fn 二桁以上の数字を含む式をtoken列にする() {
        let test_case = "10 * 25 - 306";

        let result = make_token_list(test_case);
        let expected = vec![
            Token::Number(10),
            Token::Times,
            Token::Number(25),
            Token::Minus,
            Token::Number(306),
        ];

        assert_eq!(result, expected, r#"tokenize "10 * 25 * 306""#);
    }
}
