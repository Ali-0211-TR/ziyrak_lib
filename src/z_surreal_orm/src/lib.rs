//! Z_SURREAL_ORM специально разработонная SurrealQL конструктор для написание более удобней чем SQL запросов напримую и более гибкий чем от уже сущеествующего SDK в Rust для запросов.
//! 
//! По суте это всего лишь рассширение для клиента Surreal от Rust SDK `surrealdb`.
//! Добились этого через создание трейтов и специально реализовав их для клиента `surrealdb::Surreal` и `surrealdb::method::Query`.
//! 
//! #Examples
//! 
//! 
//! 
//! 
//! Реализованные методы SurrealQL:
//! | -------- | ---------
//! | `SELECT` |
//! | `CREATE` |
//! | `INSERT` |
//! | `DELETE` |
//! | `UPDATE` |
//! ------------------------


pub(crate) mod core;
pub(crate) mod query;
pub mod sql;

pub use sql::statement::{
    select::select,
    create::create,
    insert::insert,
    delete::delete,
    update::update,
    Statement
};

pub use query::{SqlQuery, StatementType};


/////////////////////////////////////////////
/////////////////////////////////////////////
/////////////////////////////////////////////

use async_trait::async_trait;
use surrealdb::{
    Connection, Response, Surreal, method
};
/// Трейт для рассширении Surreal
#[async_trait]
pub trait ZrkSurrealOrm<C: Connection> {
    type QueryType<'a>: 'a where Self: 'a;
    
    /// Метод аналог `query`, но с маленьким отличием, в аргументах ожидается STATEMENT из нашего ORM
    /// Возвращает то же что и `query`  
    fn zrk_orm_statement<'a, S: Statement>(&'a self, statement: S) -> Self::QueryType<'a>;


    /// Метод аналог `zrk_orm_statement`, но с отличием того что в принимает `SqlQuery` из нашего
    /// ORM, это своего рода целый набор SQL запросов, возвращает оно `surrealdb::Response`
    async fn zrk_orm_sql_query(&self, sql: SqlQuery) -> surrealdb::Result<Response>;
}
#[async_trait]
impl<C: Connection> ZrkSurrealOrm<C> for Surreal<C> {
    type QueryType<'a> = method::Query<'a, C>;
    fn zrk_orm_statement<'a, S: Statement>(&'a self, statement: S) -> Self::QueryType<'a> {
        let sql = statement.to_string();
        self.query(sql)
    }
    async fn zrk_orm_sql_query(&self,mut sql: SqlQuery) -> surrealdb::Result<Response> {
        let s = sql.get_statements();

        
        match s.first(){
            Some(val) => {
                let mut q = self.query(val.to_string());
                for i in 1..s.len(){
                    match s.get(i) {
                        Some(val) => {
                            q = q.query(val.to_string());
                        },
                        None => {break;}
                    }
                }
                Ok(q.await?)
            },
            None => {
                Ok(self.query("SELECT 'Empty Query in z_sql_query method'").await?)
            },
        }
    }
}
pub trait ZrkSurrealOrmQuery<'a, C: Connection> {
    /// Метод аналог `query`, но с маленьким отличием, в аргументах ожидается STATEMENT из нашего ORM
    /// Возвращает то же что и `query`  
    fn zrk_orm_statement<S: Statement>(self, statement: S) -> method::Query<'a, C>;
}

impl<'a, C: Connection> ZrkSurrealOrmQuery<'a, C> for method::Query<'a, C> {
    fn zrk_orm_statement<S: Statement>(self, statement: S) -> method::Query<'a, C> {
        let sql = statement.to_string();
        self.query(sql)
    }
}