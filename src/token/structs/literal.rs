use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Literal {
    /// 123
    DecimalInteger,

    /// 12.3
    DecimalPointNumber,

    /// 12e3
    DecimalExponentNumber,

    /// 12.3e4
    DecimalPointExponentNumber,

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
            Literal::DecimalInteger => todo!(),
            Literal::DecimalPointNumber => todo!(),
            Literal::DecimalExponentNumber => todo!(),
            Literal::DecimalPointExponentNumber => todo!(),
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
