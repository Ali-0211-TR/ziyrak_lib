use std::fmt;


/// Перечисления для возврата значений в Sql запросов
/// #Пример в Sql
/// ```
/// // Create a new record with a random id
/// CREATE person SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript'] RETURN AFTER; // default
/// // Create a new record with a specific numeric id
/// CREATE person:100 SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript'] RETURN BEFORE;
/// // Create a new record with a specific string id
/// CREATE person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript'] RETURN NONE;
/// ```
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
