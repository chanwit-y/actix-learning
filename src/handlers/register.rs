use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
use crate::models::user::{RegisterUser, User};
use actix_web::{web, HttpResponse};

pub async fn register(new_user: web::Json<RegisterUser>, pool: web::Data<PgPool>) -> HttpResponse {
	let pg_pool = pg_pool_handler(pool).unwrap();
	let register_user = new_user
		.into_inner()
		.validates()
		.map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
		.unwrap();

	User::create(register_user, &pg_pool)
		.map(|user| HttpResponse::Ok().json(user))
		.map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
		.unwrap()
}
