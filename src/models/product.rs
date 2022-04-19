use crate::schema::products;
use std::vec::Vec;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
	pub id: i32,
	pub name: String,
	pub stock: f64,
	pub price: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct {
	pub name: Option<String>,
	pub stock: Option<f64>,
	pub price: Option<i32>,
}

impl Product {
	pub fn find(id: &i32) -> Result<Product, diesel::result::Error> {
		use crate::db_connection::establish_connetion;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		let connection = establish_connetion();

		products::table.find(id).first(&connection)
	}

	pub fn destroy(id: &i32) -> Result<(), diesel::result::Error> {
		use crate::db_connection::establish_connetion;
		use crate::schema::products::dsl;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		let connection = establish_connetion();

		diesel::delete(dsl::products.find(&id)).execute(&connection)?;
		Ok(())
	}

	pub fn update(id: &i32, new_product: &NewProduct) -> Result<(), diesel::result::Error> {
		use crate::db_connection::establish_connetion;
		use crate::schema::products::dsl;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		let connection = establish_connetion();

		diesel::update(dsl::products.find(id))
			.set(new_product)
			.execute(&connection)?;
		Ok(())
	}
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

impl NewProduct {
	pub fn create(&self) -> Result<Product, diesel::result::Error> {
		use crate::db_connection::establish_connetion;
		use diesel::RunQueryDsl;

		let connection = establish_connetion();
		diesel::insert_into(products::table)
			.values(self)
			.get_result(&connection)
	}
}
