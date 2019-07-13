#[derive(serde::Deserialize, serde::Serialize, Debug)]

pub struct Account {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub user_type: Option<String>,
}