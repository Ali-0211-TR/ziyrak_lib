use std::fmt;

#[derive(Debug, Clone)]
pub enum Output {
    None,
    Null,
    After,
    Before,
    Fields(String),
}

impl Default for Output {
    fn default() -> Self {
        Output::After
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RETURN ")?;
        match self {
            Output::None => write!(f, "NONE"),
            Output::Null => write!(f, "NULL"),
            Output::After => write!(f, "AFTER"),
            Output::Before => write!(f, "BEFORE"),
            Output::Fields(fields) => write!(f, "{}", fields),
        }
    }
}
