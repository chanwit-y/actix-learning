use crate::schema::products;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
	pub id: i32,
	pub name: String,
	pub stock: f64,
	pub price: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct {
	pub name: Option<String>,
	pub stock: Option<f64>,
	pub price: Option<i32>,
}

impl ProductList {
	pub fn list() -> Self {
		use crate::db_connection::establish_connetion;
		use crate::schema::products::dsl::*;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		let connection = establish_connetion();

		let result = products
			.limit(10)
			.load::<Product>(&connection)
			.expect("Error loading products");

		ProductList(result)
	}
}
