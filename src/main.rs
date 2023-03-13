fn main() {
    println!("Hello, world!");
}

mod ast {

    #[test]
    fn make_astは文字列からastを作成します() {
        let test_case = "3 + 2";
        let ast = make_ast(test_case);
        let expected = Ast::ExpressionPlus {
            left: Ast::Number { value: 3 },
            right: Ast::Number { value: 4 },
        };
        assert!(ast == expected, r#"parse "3 + 2""#);
    }
}
