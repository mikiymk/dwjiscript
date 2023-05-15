use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Punctuator {
    /// {
    LeftBrace,

    /// }
    RightBrace,

    /// (
    LeftParen,

    /// )
    RightParen,

    /// [
    LeftBracket,

    /// ]
    RightBracket,

    /// .
    Dot,

    /// ...
    DotThree,

    /// ;
    SemiColon,

    /// ,
    Comma,

    /// <
    LessThan,

    /// >
    GreaterThan,

    /// <=
    LessThanEqual,

    /// >=
    GreaterThanEqual,

    /// ==
    Equal,

    /// !=
    NotEqual,

    /// ===
    StrictEqual,

    /// !==
    StrictNotEqual,

    /// +
    Plus,

    /// -
    Minus,

    /// *
    Multiply,

    /// /
    Divide,

    /// %
    Remainder,

    /// **
    Exponentiation,

    /// ++
    Increment,

    /// --
    Decrement,

    /// <<
    LeftShift,

    /// >>
    RightShift,

    /// >>>
    UnsignedRightShift,

    /// &
    BitAnd,

    /// |
    BitOr,

    /// ^
    BitXor,

    /// !
    LogicalNot,

    /// ~
    BitNot,

    /// &&
    LogicalAnd,

    /// ||
    LogicalOr,

    /// ??
    NullishCoalesce,

    /// ?
    TernaryQuestion,

    /// :
    TernaryColon,

    /// =
    Assign,

    /// +=
    PlusAssign,

    /// -=
    MinusAssign,

    /// *=
    MultiplyAssign,

    /// /=
    DivideAssign,

    /// %=
    RemainderAssign,

    /// **=
    ExponentiationAssign,

    /// <<=
    LeftShiftAssign,

    /// >>=
    RightShiftAssign,

    /// >>>=
    UnsignedRightShiftAssign,

    /// &=
    BitAndAssign,

    /// |=
    BitOrAssign,

    /// ^=
    BitXorAssign,

    /// &&=
    LogicalAndAssign,

    /// ||=
    LogicalOrAssign,

    /// ??=
    NullishCoalesceAssign,

    /// =>
    Arrow,
}

impl ToSourceString for Punctuator {
    fn to_source_string(&self) -> String {
        match self {
            Punctuator::LeftBrace => todo!(),
            Punctuator::RightBrace => todo!(),
            Punctuator::LeftParen => todo!(),
            Punctuator::RightParen => todo!(),
            Punctuator::LeftBracket => todo!(),
            Punctuator::RightBracket => todo!(),
            Punctuator::Dot => todo!(),
            Punctuator::DotThree => todo!(),
            Punctuator::SemiColon => todo!(),
            Punctuator::Comma => todo!(),
            Punctuator::LessThan => todo!(),
            Punctuator::GreaterThan => todo!(),
            Punctuator::LessThanEqual => todo!(),
            Punctuator::GreaterThanEqual => todo!(),
            Punctuator::Equal => todo!(),
            Punctuator::NotEqual => todo!(),
            Punctuator::StrictEqual => todo!(),
            Punctuator::StrictNotEqual => todo!(),
            Punctuator::Plus => todo!(),
            Punctuator::Minus => todo!(),
            Punctuator::Multiply => todo!(),
            Punctuator::Divide => todo!(),
            Punctuator::Remainder => todo!(),
            Punctuator::Exponentiation => todo!(),
            Punctuator::Increment => todo!(),
            Punctuator::Decrement => todo!(),
            Punctuator::LeftShift => todo!(),
            Punctuator::RightShift => todo!(),
            Punctuator::UnsignedRightShift => todo!(),
            Punctuator::BitAnd => todo!(),
            Punctuator::BitOr => todo!(),
            Punctuator::BitXor => todo!(),
            Punctuator::LogicalNot => todo!(),
            Punctuator::BitNot => todo!(),
            Punctuator::LogicalAnd => todo!(),
            Punctuator::LogicalOr => todo!(),
            Punctuator::NullishCoalesce => todo!(),
            Punctuator::TernaryQuestion => todo!(),
            Punctuator::TernaryColon => todo!(),
            Punctuator::Assign => todo!(),
            Punctuator::PlusAssign => todo!(),
            Punctuator::MinusAssign => todo!(),
            Punctuator::MultiplyAssign => todo!(),
            Punctuator::DivideAssign => todo!(),
            Punctuator::RemainderAssign => todo!(),
            Punctuator::ExponentiationAssign => todo!(),
            Punctuator::LeftShiftAssign => todo!(),
            Punctuator::RightShiftAssign => todo!(),
            Punctuator::UnsignedRightShiftAssign => todo!(),
            Punctuator::BitAndAssign => todo!(),
            Punctuator::BitOrAssign => todo!(),
            Punctuator::BitXorAssign => todo!(),
            Punctuator::LogicalAndAssign => todo!(),
            Punctuator::LogicalOrAssign => todo!(),
            Punctuator::NullishCoalesceAssign => todo!(),
            Punctuator::Arrow => todo!(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_add_2() {
        assert_eq!(1 + 1, 2);
    }
}
