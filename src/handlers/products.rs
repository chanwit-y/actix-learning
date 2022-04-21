use crate::handlers::pg_pool_handler;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::db_connection::PgPool;
use crate::models::product::{NewProduct, Product, ProductList};


pub async fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
	let pg_pool = pg_pool_handler(pool).unwrap();
	HttpResponse::Ok().json(ProductList::list(&pg_pool))
}

//pub fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
pub async fn create(new_product: web::Json<NewProduct>, pool: web::Data<PgPool>) -> HttpResponse {
	let pg_pool = pg_pool_handler(pool).unwrap();
	new_product
		.create(&pg_pool)
		.map(|product| HttpResponse::Ok().json(product))
		.unwrap()
	//     .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}

//pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
pub async fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
	let pg_pool = pg_pool_handler(pool).unwrap();
	Product::find(&id, &pg_pool)
		.map(|product| HttpResponse::Ok().json(product))
		.unwrap()
	// .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}

//pub fn destroy(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
pub async fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
	let pg_pool = pg_pool_handler(pool).unwrap();
	Product::destroy(&id, &pg_pool)
		.map(|_| HttpResponse::Ok().json(()))
		.unwrap()
	// .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}

//pub fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
pub async fn update(
	id: web::Path<i32>,
	new_product: web::Json<NewProduct>,
	pool: web::Data<PgPool>,
) -> HttpResponse {
	let pg_pool = pg_pool_handler(pool).unwrap();
	Product::update(&id, &new_product, &pg_pool)
		.map(|_| HttpResponse::Ok().json(()))
		.unwrap()
	// .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}
