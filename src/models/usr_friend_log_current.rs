use crate::rows::usr_friend_log_current::UsrFriendLogCurrentRow;
use crate::zaphkiel::trust_level::TrustLevel;

/// This is a row from the `usr_friend_log_current` table.
///
/// Example:
///
/// ```
/// use surrealdb_test::models::usr_friend_log_current::UsrFriendLogCurrent;
/// use surrealdb_test::rows::usr_friend_log_current::UsrFriendLogCurrentRow;
/// use surrealdb_test::zaphkiel::trust_level::TrustLevel;
///
/// let row = UsrFriendLogCurrent::from(
///     UsrFriendLogCurrentRow {
///         user_id: "usr_12345678-1234-1234-1234-123456789abc".to_string(),
///         display_name: "Some User".to_string(),
///         trust_level: "User".to_string(),
///     }
/// );
///
/// assert_eq!(row.user_id, "usr_12345678-1234-1234-1234-123456789abc".to_string());
/// assert_eq!(row.display_name, "Some User".to_string());
/// assert_eq!(row.trust_level, TrustLevel::User);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct UsrFriendLogCurrent {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: TrustLevel,
}

impl UsrFriendLogCurrent {
    /// Get all rows from the `usr_friend_log_current` table by using the default values
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }
}

impl From<UsrFriendLogCurrentRow> for UsrFriendLogCurrent {
    /// Convert a `UsrFriendLogCurrentRow` into a `UsrFriendLogCurrent`
    ///
    /// # What it does
    ///
    /// It converts the `trust_level` field from a `String` into a `TrustLevel` enum.
    /// The `user_id` and `display_name` fields are copied as is.
    fn from(row: UsrFriendLogCurrentRow) -> Self {
        Self {
            user_id: row.user_id,
            display_name: row.display_name,
            trust_level: TrustLevel::from(row.trust_level),
        }
    }
}
