use std::fmt;

pub struct LimitedDisplay<'a> {
    s: &'a str,
    limit: usize,
}

impl <'a> LimitedDisplay<'a> {
    pub fn new(s: &'a str, limit: usize) -> Self {
        Self { s, limit }
    }
}

impl<'a> fmt::Display for LimitedDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let limited_str = &self.s.chars().take(self.limit).collect::<String>();
        write!(f, "{}", limited_str)
    }
}