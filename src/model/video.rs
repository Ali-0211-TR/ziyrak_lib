use crate::model::DateTimeUtc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
    Hash, serde::Serialize, serde::Deserialize)]
pub struct Video{
    pub id:         surrealdb::sql::Thing,
    pub url:        Option<String>,
    pub created_at: DateTimeUtc,
    pub begined_at: DateTimeUtc,
    pub ended_at:   DateTimeUtc,
    pub is_paniced: bool,
    pub camera_id:  surrealdb::sql::Id,
}