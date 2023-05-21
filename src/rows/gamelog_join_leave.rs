use chrono::{DateTime, Utc};

/// This is a row from the `gamelog_join_leave` table.
#[derive(Clone, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct GamelogJoinLeaveRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    // #[sqlx(rename = "type")]
    pub event: String,
    pub display_name: String,
    pub location: String,
    pub user_id: String,
    pub time: i64,
}
