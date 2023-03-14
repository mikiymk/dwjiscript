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
