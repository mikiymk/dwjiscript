pub mod reader_struct;
pub mod script;
pub mod structs;
pub mod token;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ast {
    ExpressionPlus { left: Box<Ast>, right: Box<Ast> },
    Number(u32),
}

pub fn make_ast(source: &str) -> Result<Ast, String> {
    let tokens = token::make_token_list(source)?;

    let (ast, _) = parse_expression_plus_minus(&tokens);
    Ok(ast)
}

fn parse_expression_plus_minus(tokens: &[token::Token]) -> (Ast, &[token::Token]) {
    let (left, tokens) = parse_expression_time_div(tokens);
    let (token, new_tokens) = token::pop_token(tokens);
    match token {
        token::Token::Plus | token::Token::Minus => {
            let (right, tokens) = parse_expression_time_div(new_tokens);
            (
                Ast::ExpressionPlus {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                tokens,
            )
        }
        _ => (left, tokens),
    }
}

fn parse_expression_time_div(tokens: &[token::Token]) -> (Ast, &[token::Token]) {
    let (left, tokens) = parse_number(tokens);
    let (token, new_tokens) = token::pop_token(tokens);
    match token {
        token::Token::Plus | token::Token::Minus => {
            let (right, tokens) = parse_number(new_tokens);
            (
                Ast::ExpressionPlus {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                tokens,
            )
        }
        _ => (left, tokens),
    }
}

fn parse_number(tokens: &[token::Token]) -> (Ast, &[token::Token]) {
    let (token, new_tokens) = token::pop_token(tokens);
    match token {
        token::Token::Number(n) => (Ast::Number(n.parse().unwrap()), new_tokens),
        _ => (Ast::Number(0), tokens),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn make_astは文字列からastを作成します() {
        let test_case = "3 + 2";
        let ast = super::make_ast(test_case).unwrap();
        let expected = super::Ast::ExpressionPlus {
            left: Box::new(super::Ast::Number(3)),
            right: Box::new(super::Ast::Number(2)),
        };
        assert_eq!(ast, expected, r#"parse "3 + 2""#);
    }
}
