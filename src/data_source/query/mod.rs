pub(crate) mod statement_models;

use statement_models::{
    Select, Create, Update, Insert, Delete,
    StatementTrait
};

#[derive(Debug, Clone)]
pub struct Query {
    pub(crate) statements: Vec<String>,
}

impl Query {
    pub fn new() -> Query {
        Query { statements: vec![] }
    }
    pub fn add_statement<T: StatementTrait>(mut self, statement: T) -> Query {
        self.statements.push(statement.get_string());
        self
    }


}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.statements {
            write!(f, "{i};")?;
        }
        Ok(())
    }
}



//
/// пока в стадии разработки
enum StatementType{
    SelectStatement(Select),
    UpdateStatement(Update),
    InsertStatement(Insert),
    DeleteStatement(Delete),
    CreateStatement(Create),
}

struct Statement(StatementType);



impl Statement {
    pub fn select(&mut self, expr: Vec<&str>) -> Option<&mut Select> {
        self.0 = StatementType::SelectStatement(Select::init(expr));
        match &mut self.0 {
            StatementType::SelectStatement(s) => {
                Some(s)
            },
            _ => {
                None
            }
        }
    }
    pub fn create(&mut self, what: &str) -> Option<&mut Create> {
        self.0 = StatementType::CreateStatement(Create::init(what));
        match &mut self.0 {
            StatementType::CreateStatement(c) => {
                Some(c)
            },
            _ => {
                None
            }
        }
    }
    pub fn insert(&mut self, into: &str) -> Option<&mut Insert> {
        self.0 = StatementType::InsertStatement(Insert::init(into));
        match &mut self.0 {
            StatementType::InsertStatement(i) => {
                Some(i)
            },
            _ => {
                None
            }
        }
    }
    pub fn delete(&mut self, what: &str) -> Option<&mut Delete> {
        self.0 = StatementType::DeleteStatement(Delete::init(what));
        match &mut self.0 {
            StatementType::DeleteStatement(d) => {
                Some(d)
            },
            _ => {
                None
            }
        }
    }
    pub fn update(&mut self, what: &str) -> Option<&mut Update> {
        self.0 = StatementType::UpdateStatement(Update::init(what));
        match &mut self.0 {
            StatementType::UpdateStatement(s) => {
                Some(s)
            },
            _ => {
                None
            }
        }
    }
}
