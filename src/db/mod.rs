pub mod schema;
pub mod alert;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://user:password@localhost/crypto_alerts");
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
