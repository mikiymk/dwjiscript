use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Literal {
    /// 123
    /// 12.3
    /// 12e3
    DecimalNumber(u64),

    /// 123n
    DecimalBigInteger(String),

    /// 0b011
    BinaryInteger(u64),

    /// 0o067
    OctalInteger(u64),

    /// 0x0ef
    HexInteger(u64),

    /// 0b011n
    BinaryBigInteger(String),

    /// 0o067n
    OctalBigInteger(String),

    /// 0x0efn
    HexBigInteger(String),

    /// 067
    LegacyOctalInteger(u64),

    /// 'abc'
    SingleString(Vec<u16>),

    /// "abc"
    DoubleString(Vec<u16>),
}

impl ToSourceString for Literal {
    fn to_source_string(&self) -> String {
        match self {
            Literal::DecimalNumber(_) => todo!(),
            Literal::DecimalBigInteger(_) => todo!(),
            Literal::BinaryInteger(_) => todo!(),
            Literal::OctalInteger(_) => todo!(),
            Literal::HexInteger(_) => todo!(),
            Literal::BinaryBigInteger(_) => todo!(),
            Literal::OctalBigInteger(_) => todo!(),
            Literal::HexBigInteger(_) => todo!(),
            Literal::LegacyOctalInteger(_) => todo!(),
            Literal::SingleString(_) => todo!(),
            Literal::DoubleString(_) => todo!(),
        }
    }
}
