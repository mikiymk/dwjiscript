#![allow(dead_code, unused)]

use crate::ast::{reader_struct::SourceCodeReader, script::parse_script};

fn main() {
    println!("Hello, world!");

    let source = read_line().unwrap_or("0".to_string());

    println!("{:?}", source);

    let reader = SourceCodeReader::new(&source);
    let ast = parse_script(reader);

    println!("{:?}", ast);

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
    pub mod structs;
    pub mod tokenize;
}
mod pattern_macros;
mod to_source_string;
