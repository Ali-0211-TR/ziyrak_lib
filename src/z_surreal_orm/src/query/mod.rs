
use crate::sql::
    statement::{
        Select, Create, Update, Insert, Delete
    };

#[derive(Debug, Clone)]
pub enum StatementType{
    SelectStatement(Select),
    UpdateStatement(Update),
    InsertStatement(Insert),
    DeleteStatement(Delete),
    CreateStatement(Create),
}
impl std::fmt::Display for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelectStatement(s) => write!(f, "{}", s)?,
            Self::UpdateStatement(u) => write!(f, "{}", u)?,
            Self::InsertStatement(i) => write!(f, "{}", i)?,
            Self::DeleteStatement(d) => write!(f, "{}", d)?,
            Self::CreateStatement(c) => write!(f, "{}", c)?,
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct SqlQuery {
    pub(crate) statements: Vec<StatementType>,
}

impl SqlQuery {
    pub fn new() -> SqlQuery {
        SqlQuery { statements: vec![] }
    }
    pub fn get_statements(&mut self) -> &mut Vec<StatementType>{
        &mut self.statements
    }
    pub fn add_select(mut self, statement: Select) -> SqlQuery {
        self.statements.push(StatementType::SelectStatement(statement));
        self
    }
    pub fn add_update(mut self, statement: Update) -> SqlQuery {
        self.statements.push(StatementType::UpdateStatement(statement));
        self
    }
    pub fn add_insert(mut self, statement: Insert) -> SqlQuery {
        self.statements.push(StatementType::InsertStatement(statement));
        self
    }
    pub fn add_delete(mut self, statement: Delete) -> SqlQuery {
        self.statements.push(StatementType::DeleteStatement(statement));
        self
    }
    pub fn add_create(mut self, statement: Create) -> SqlQuery {
        self.statements.push(StatementType::CreateStatement(statement));
        self
    }
}

impl std::fmt::Display for SqlQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            write!(f, "{};", stmt)?;
        }
        Ok(())
    }
}
