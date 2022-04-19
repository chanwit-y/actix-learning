use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	Pool::builder().build(manager)
}

pub fn establish_connetion() -> PgPool {
	dotenv().ok();
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	init_pool(&database_url).expect("Failed to create pool")
}

// pub fn establish_connetion() -> PgConnection {
// 	dotenv().ok();

// 	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

// 	// PgConnection::establish(&database_url)
// 	// 	.expect(&format!("Error connection to {}", database_url))
// }
