fn main() {
    println!("Hello, world!");

    let source = read_line().unwrap_or("0".to_string());

    let ast = ast::make_ast(&source);

    println!("{:?}", ast);
}

fn read_line() -> Option<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok()?;
    Some(s.trim_end().to_string())
}

mod ast {
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Ast {
        ExpressionPlus { left: Box<Ast>, right: Box<Ast> },
        Number(u32),
    }

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    enum Token {
        Number(u32),
        Plus,
        Minus,
        Times,
        Divide,
        Eof,
    }

    fn make_token_list(source: &str) -> Vec<Token> {
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

    fn pop_token<'a>(tokens: &'a [Token]) -> (Token, &'a [Token]) {
        if tokens.len() == 0 {
            return (Token::Eof, tokens);
        }
        let token = &tokens[0];
        let tokens = &tokens[1..];
        (*token, tokens)
    }

    pub fn make_ast(source: &str) -> Ast {
        let tokens = make_token_list(source);

        let (ast, _) = parse_expression_plus_minus(&tokens);
        ast
    }

    fn parse_expression_plus_minus<'a>(tokens: &'a [Token]) -> (Ast, &'a [Token]) {
        let (left, tokens) = parse_expression_time_div(tokens);
        let (token, new_tokens) = pop_token(&tokens);
        match token {
            Token::Plus | Token::Minus => {
                let (right, tokens) = parse_expression_time_div(new_tokens);
                (
                    Ast::ExpressionPlus {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    &tokens,
                )
            }
            _ => (left, &tokens),
        }
    }

    fn parse_expression_time_div<'a>(tokens: &'a [Token]) -> (Ast, &'a [Token]) {
        let (left, tokens) = parse_number(tokens);
        let (token, new_tokens) = pop_token(&tokens);
        match token {
            Token::Plus | Token::Minus => {
                let (right, tokens) = parse_number(new_tokens);
                (
                    Ast::ExpressionPlus {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    &tokens,
                )
            }
            _ => (left, &tokens),
        }
    }

    fn parse_number<'a>(tokens: &'a [Token]) -> (Ast, &'a [Token]) {
        let (token, new_tokens) = pop_token(&tokens);
        match token {
            Token::Number(n) => (Ast::Number(n), &new_tokens),
            _ => (Ast::Number(0), &tokens),
        }
    }

    #[test]
    fn make_astは文字列からastを作成します() {
        let test_case = "3 + 2";
        let ast = make_ast(test_case);
        let expected = Ast::ExpressionPlus {
            left: Box::new(Ast::Number(3)),
            right: Box::new(Ast::Number(4)),
        };
        assert!(ast == expected, r#"parse "3 + 2""#);
    }
}
