use crate::model;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
    Hash, serde::Serialize, serde::Deserialize)]
pub struct Photo{
    pub id:         surrealdb::sql::Thing,
    pub url:        Option<String>,
    pub created_at: model::DateTimeUtc,
    pub is_paniced: bool,
    pub camera_id: surrealdb::sql::Id,
}