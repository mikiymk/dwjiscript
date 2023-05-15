use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Comment {
    SingleLineComment(String),
    MultiLineComment(String),
    HashBangComment(String),
}

impl ToSourceString for Comment {
    fn to_source_string(&self) -> String {
        match self {
            Comment::SingleLineComment(s) => format!("//{}", s),
            Comment::MultiLineComment(s) => format!("/*{}*/", s),
            Comment::HashBangComment(s) => format!("#!{}", s),
        }
    }
}
