use chrono::{DateTime, Utc};

/// This is a row from the `gamelog_location` table.
#[derive(Clone, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct GamelogLocationRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub location: String,
    pub world_id: String,
    pub world_name: String,
    pub time: i64,
    pub group_name: String,
}
