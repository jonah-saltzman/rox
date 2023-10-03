use strum_macros::{EnumString, AsRefStr, EnumVariantNames};

#[derive(Debug, PartialEq, Eq, EnumString, AsRefStr, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}