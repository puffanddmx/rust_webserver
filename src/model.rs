use crate::schema::*;

#[derive(Queryable)]
pub struct UsersSessions {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
}

#[derive(Insertable)]
#[table_name = "users_sessions"]
pub struct NewUserSession {
    pub user_id: i64, 
    pub token: String,
}
