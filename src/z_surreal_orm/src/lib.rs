//! Z_SURREAL_ORM специально разработонная SurrealQL конструктор для написание более удобней чем SQL запросов напримую и более гибкий чем от уже сущеествующего SDK в Rust для запросов.
//! 
//! По суте это всего лишь рассширение для клиента Surreal от Rust SDK `surrealdb`.
//! Добились этого через создание трейтов и специально реализовав их для клиента `surrealdb::Surreal` и `surrealdb::method::Query`.
//! 
//! Реализованные методы SurrealQL:
//! -------------------------------
//! - `SELECT` -- Выборка элементов из записей
//! - `CREATE` -- Создание записей
//! - `INSERT` -- Вставка данных в записи
//! - `DELETE` -- Удаление записей
//! - `UPDATE` -- Обновление записей
//! 
//! 
//! 


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
/// Трейт для рассширении `surrealdb::Surreal` and `surrealdb::method::Query`
#[async_trait]
pub trait ZrkSurrealOrm<C: Connection> {
    type QueryType<'a>: 'a where Self: 'a;
    /// Метод аналогичен `query`, но с небольшим отличием: в аргументах ожидается `Statement` из нашего ORM.
    /// Возвращает то же, что и `query`.
    ///
    /// # Arguments
    ///
    /// * `statement`: Объект типов которые реализовали `Statement` из нашего ORM, представляющий SQL-запрос.
    ///
    /// # Example
    /// ```rust
    /// // Including trait and statements
    /// use z_surreal_orm::{
    ///     ZrkSurrealOrm,
    ///     create, select
    /// };
    ///
    /// // Select the namespace/database to use
    /// db.use_ns("namespace").use_db("database").await?;
    ///
    /// let tb_name = "person";
    ///
    /// // Run queries
    /// let mut ress = db
    ///     .zrk_orm_statement(create(tb_name))
    ///     .zrk_orm_statement(select(vec![]).from(vec![tb_name]))
    ///     .await?;
    ///
    /// // Get the first result from the first query
    /// let created: Option<Person> = ress.take(0)?;
    ///
    /// // Get all of the results from the second query
    /// let people: Vec<Person> = ress.take(1)?;
    /// ```
    fn zrk_orm_statement<'a, S: Statement>(&'a self, statement: S) -> Self::QueryType<'a>;

    /// Метод аналогичен `zrk_orm_statement`, но принимает `SqlQuery` из нашего ORM, представляющий набор SQL-запросов.
    /// Возвращает `surrealdb::Response`.
    ///
    /// # Arguments
    ///
    /// * `sql`: Объект `SqlQuery` из нашего ORM, представляющий целый набор SQL-запросов.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Including trait and statements
    /// use z_surreal_orm::{
    ///     ZrkSurrealOrmQuery,
    ///     create, select,
    ///     SqlQuery
    /// };
    /// 
    /// // Select the namespace/database to use
    /// db.use_ns("namespace").use_db("database").await?;
    /// 
    /// // Run queries
    /// let mut ress = db.zrk_orm_sql_query(
    ///     SqlQuery::new()
    ///     .add_create(create("person"))
    ///     .add_select(select(vec![]).from(vec!["person"]))
    /// ).await?;
    /// 
    /// // Get the first result from the first query
    /// let created: Option<Person> = ress.take(0)?;
    /// 
    /// // Get all of the results from the second query
    /// let people: Vec<Person> = ress.take(1)?;
    /// ```
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

/// Трейт для рассширении `surrealdb::Surreal` 
pub trait ZrkSurrealOrmQuery<'a, C: Connection> {
    /// Метод аналогичен `query`, но с небольшим отличием: в аргументах ожидается `Statement` из нашего ORM.
    /// Возвращает то же, что и `query`.
    ///
    /// # Arguments
    ///
    /// * `statement`: Объект типов которые реализовали `Statement` из нашего ORM, представляющий SQL-запрос.
    ///
    /// # Example
    /// ```rust
    /// // Including trait and statements
    /// use z_surreal_orm::{
    ///     ZrkSurrealOrm,
    ///     create, select
    /// };
    ///
    /// // Select the namespace/database to use
    /// db.use_ns("namespace").use_db("database").await?;
    ///
    /// let tb_name = "person";
    ///
    /// // Run queries
    /// let mut ress = db
    ///     .zrk_orm_statement(create(tb_name))
    ///     .zrk_orm_statement(select(vec![]).from(vec![tb_name]))
    ///     .await?;
    ///
    /// // Get the first result from the first query
    /// let created: Option<Person> = ress.take(0)?;
    ///
    /// // Get all of the results from the second query
    /// let people: Vec<Person> = ress.take(1)?;
    /// ```
    fn zrk_orm_statement<S: Statement>(self, statement: S) -> method::Query<'a, C>;
}

impl<'a, C: Connection> ZrkSurrealOrmQuery<'a, C> for method::Query<'a, C> {
    fn zrk_orm_statement<S: Statement>(self, statement: S) -> method::Query<'a, C> {
        let sql = statement.to_string();
        self.query(sql)
    }
}