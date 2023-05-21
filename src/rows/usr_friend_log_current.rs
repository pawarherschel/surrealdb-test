/// This is a row from the `usr_friend_log_current` table.
///
/// Example:
/// ```
/// use surrealdb_test::models::usr_friend_log_current::UsrFriendLogCurrent;
/// use surrealdb_test::zaphkiel::trust_level::TrustLevel;
///
/// let row = UsrFriendLogCurrent {
///    user_id: "usr_12345678-1234-1234-1234-123456789abc".to_string(),
///   display_name: "Some User".to_string(),
///  trust_level: TrustLevel::User,
/// };
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsrFriendLogCurrentRow {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: String,
}
