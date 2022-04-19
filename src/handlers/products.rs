use crate::models::product::{NewProduct, Product, ProductList};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn index(_req: HttpRequest) -> HttpResponse {
	HttpResponse::Ok().json(ProductList::list())
}

//pub fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
pub async fn create(new_product: web::Json<NewProduct>) -> HttpResponse {
	new_product
		.create()
		.map(|product| HttpResponse::Ok().json(product))
		.unwrap()
	//     .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}

//pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
pub async fn show(id: web::Path<i32>) -> HttpResponse {
	Product::find(&id)
		.map(|product| HttpResponse::Ok().json(product))
		.unwrap()
	// .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}

//pub fn destroy(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
pub async fn destroy(id: web::Path<i32>) -> HttpResponse {
	Product::destroy(&id)
		.map(|_| HttpResponse::Ok().json(()))
		.unwrap()
	// .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}

//pub fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
pub async fn update(id: web::Path<i32>, new_product: web::Json<NewProduct>) -> HttpResponse {
	Product::update(&id, &new_product)
		.map(|_| HttpResponse::Ok().json(()))
		.unwrap()
	// .map_err(|e| {
	// 	HttpResponse::InternalServerError().json(e.to_string())
	//     })
}
