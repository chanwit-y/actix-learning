use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connetion() -> PgConnection {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	PgConnection::establish(&database_url)
		.expect(&format!("Error connection to {}", database_url))
}
