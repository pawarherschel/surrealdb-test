/// This is a row from the `sqlite_master` table.
#[derive(Clone, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct SqliteMaster {
    pub type_: String,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: i32,
    pub sql: String,
}
