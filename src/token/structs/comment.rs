#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Comment {
    SingleLineComment,
    MultiLineComment,
    HashBangComment,
}
