use super::query::Query;

pub enum TableName {
    Camera,
    Video,
    Photo,
}
impl TableName {
    pub fn as_str(&self) -> &str {
        match self {
            TableName::Camera => "camera",
            TableName::Video => "video",
            TableName::Photo => "photo",
        }
    }
}

pub struct DataBaseManager {
    pub db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    //pub(crate) sql_query: Option<Query>
}

impl DataBaseManager {
    pub async fn init() -> surrealdb::Result<Self> {
        let fdb = DataBaseManager {
            db: surrealdb::Surreal::new::<surrealdb::engine::local::File>("ziyrak.db").await?,
            //sql_query: None
        };
        fdb.db.use_ns("namespace").use_db("database").await?;
        Ok(fdb)
    }


    /// В стадии разработки
    pub async fn orm_query(&mut self, sql_query: Query)-> surrealdb::Result<surrealdb::Response> {
        //self.sql_query = Some(sql_query.clone());
        let ress = self.db.query(
            format!("{}", sql_query).as_str()
        ).await?;

        Ok(ress)
    }

    
    pub async fn create_table<T>(
        &mut self,
        table_name: TableName,
        object: T,
    ) -> surrealdb::Result<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let sql = format!("CREATE {} CONTENT $data", table_name.as_str());

        let mut ress = self.db.query(sql).bind(("data", object)).await?;

        let obj: Option<T> = ress.take(0)?;

        match obj {
            Some(x) => Ok(x),
            None => Err(surrealdb::Error::Db(surrealdb::error::Db::RecordExists {
                thing: "Failed to create table.".to_string(),
            })),
        }
    }
    pub async fn insert_into_table<T>(
        &mut self,
        table_name: TableName,
        data: Vec<T>,
    ) -> surrealdb::Result<Vec<T>>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let sql = format!("INSERT INTO {} $data", table_name.as_str());
        let mut ress = self.db.query(sql).bind(("data", &data)).await?;

        let vc: Vec<T> = ress.take(0)?;

        Ok(vc)
    }

    pub async fn select_all<T>(
        &mut self,
        table_name: TableName,
        filter: Option<&str>,
        limit: Option<usize>,
    ) -> surrealdb::Result<Vec<T>>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut sql = format!("SELECT * FROM {}", table_name.as_str());

        if let Some(filter) = filter {
            sql.push_str(" WHERE ");
            sql.push_str(filter);
        }

        if let Some(limit) = limit {
            sql.push_str(" LIMIT ");
            sql.push_str(&limit.to_string());
        }

        let mut ress = self.db.query(sql).await?;

        let vc: Vec<T> = ress.take(0)?;

        Ok(vc)
    }
}
