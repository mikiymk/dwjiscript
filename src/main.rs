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

mod ast;
mod token {
    mod structs {
        mod comment;
        mod literal;
        mod punctuator;
        mod token;
    }
}
mod to_source_string;
