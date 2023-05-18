use crate::to_source_string::ToSourceString;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Template {
    /// `abc`
    NoSubstitution,

    /// `abc${
    Head,

    /// }abc${
    Middle,

    /// }abc`
    Tail,
}

impl ToSourceString for Template {
    fn to_source_string(&self) -> String {
        match self {
            Template::NoSubstitution => todo!(),
            Template::Head => todo!(),
            Template::Middle => todo!(),
            Template::Tail => todo!(),
        }
    }
}
