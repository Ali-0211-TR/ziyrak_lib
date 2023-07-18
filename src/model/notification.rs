use super::DateTimeUtc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
    Hash, serde::Serialize, serde::Deserialize)
]
pub struct Notification{
    pub id: surrealdb::sql::Thing,
    pub create_at: DateTimeUtc,
    pub info: String,
    pub photo_id: surrealdb::sql::Thing,
    pub camera_id: surrealdb::sql::Thing,
    pub notification_type: String,
} 