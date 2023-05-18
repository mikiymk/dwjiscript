use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Literal {
    /// 123
    /// 12.3
    /// 12e3
    DecimalNumber,

    /// 123n
    DecimalBigInteger,

    /// 0b011
    BinaryInteger,

    /// 0o067
    OctalInteger,

    /// 0x0ef
    HexInteger,

    /// 0b011n
    BinaryBigInteger,

    /// 0o067n
    OctalBigInteger,

    /// 0x0efn
    HexBigInteger,

    /// 067
    LegacyOctalInteger,

    /// 'abc'
    SingleString,

    /// "abc"
    DoubleString,
}

impl ToSourceString for Literal {
    fn to_source_string(&self) -> String {
        match self {
            Literal::DecimalNumber => todo!(),
            Literal::DecimalBigInteger => todo!(),
            Literal::BinaryInteger => todo!(),
            Literal::OctalInteger => todo!(),
            Literal::HexInteger => todo!(),
            Literal::BinaryBigInteger => todo!(),
            Literal::OctalBigInteger => todo!(),
            Literal::HexBigInteger => todo!(),
            Literal::LegacyOctalInteger => todo!(),
            Literal::SingleString => todo!(),
            Literal::DoubleString => todo!(),
        }
    }
}
