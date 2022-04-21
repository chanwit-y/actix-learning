use crate::errors::MyStoreError;
use crate::handlers::pg_pool_handler;
use crate::handlers::PgPool;
use crate::models::user::AuthUser;
use crate::utils::jwt::create_token;

use actix_identity::Identity;
use actix_web::{web, HttpResponse};


pub fn login(
	auth_user: web::Json<AuthUser>,
	id: Identity,
	pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
	let pg_pool = pg_pool_handler(pool).unwrap();
	let user = auth_user
		.login(&pg_pool)
		.map_err(|e| match e {
			MyStoreError::DBError(diesel::result::Error::NotFound) => {
				HttpResponse::NotFound().json(e.to_string())
			}
			_ => HttpResponse::InternalServerError().json(e.to_string()),
		})
		.unwrap();

	let token = create_token(&user.email, &user.company)?;

	id.remember(token);

	Ok(HttpResponse::Ok().json(user))
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
	id.forget();
	Ok(HttpResponse::Ok().into())
}
