use sqlx::{mysql::MySql, Pool};

use crate::domain::contract::database::Database;

pub fn new(_pool: Pool<MySql>) -> Database {
  Database {}
}
