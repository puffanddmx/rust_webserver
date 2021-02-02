#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod model;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::model::{NewUserSession, UsersSessions};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_new_user_session(conn: &MysqlConnection, user_id: i64, token: String) -> UsersSessions {
    use schema::users_sessions;

    let new_user_session = NewUserSession {
        user_id: user_id,
        token: token,
    };

    diesel::insert_into(users_sessions::table).value(&new_user_session).execute(conn).expect("Error saving new session");

    users_sessions::table.order(users_sessions::id.desc()).first(conn).unwrap()
}