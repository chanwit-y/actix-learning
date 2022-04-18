use actix_web::{HttpRequest, HttpResponse};
use crate::models::product::ProductList;

pub async fn index(_req: HttpRequest) -> HttpResponse {
	HttpResponse::Ok().json(ProductList::list())
}