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
// Z  ->   
#[async_trait]
trait ZrkSurrealOrm<C: Connection> {
    type QueryType<'a>: 'a where Self: 'a;

    fn zrk_orm_statement<'a, S: Statement>(&'a self, statement: S) -> Self::QueryType<'a>;
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
trait ZrkSurrealOrmQuery<'a, C: Connection> {

    fn zrk_orm_statement<S: Statement>(self, statement: S) -> method::Query<'a, C>;
}

impl<'a, C: Connection> ZrkSurrealOrmQuery<'a, C> for method::Query<'a, C> {
    fn zrk_orm_statement<S: Statement>(self, statement: S) -> method::Query<'a, C> {
        let sql = statement.to_string();
        self.query(sql)
    }
}