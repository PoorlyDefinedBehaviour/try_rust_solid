use sqlx::{mysql::MySql, Pool};

use crate::domain::contract::database::Database;

mod users;

pub fn new(pool: Pool<MySql>) -> Database {
  Database {
    users: Box::new(users::UserRepository { pool: pool.clone() }),
  }
}
