use super::util::LimitedDisplay;

#[derive(Debug)]
pub enum Object {
    Bool(bool),
    Number(f64),
    String(String),
    Nil
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => f.write_fmt(format_args!("{}", b)),
            Self::Number(n) => f.write_fmt(format_args!("{}", n)),
            Self::String(s) => f.write_fmt(format_args!("\"{}\"", LimitedDisplay::new(s, 10))),
            Self::Nil => f.write_fmt(format_args!("Nil"))
        }
    }
}