use crate::models::price::PriceProduct;
use crate::schema::products;
use diesel::PgConnection;
use diesel::BelongingToDsl;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Product {
	pub id: i32,
	pub name: String,
	pub stock: f64,
	pub cost: Option<i32>,
	pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub std::vec::Vec<Product>);

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct {
	pub name: Option<String>,
	pub stock: Option<f64>,
	pub cost: Option<i32>,
	pub user_id: Option<i32>,
}

type ProductColumns = (
	products::id,
	products::name,
	products::stock,
	products::cost,
	products::user_id
    );

const PRODUCT_COLUMNS: ProductColumns = (
	products::id,
	products::name,
	products::stock,
	products::cost,
	products::user_id
    );

impl Product {
	pub fn find(
		id: &i32,
		connection: &PgConnection,
	) -> Result<(Product, Vec<PriceProduct>), diesel::result::Error> {
		// use crate::db_connection::establish_connetion;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;
		use crate::schema;

		// let connection = establish_connetion();

		let product: Product = schema::products::table
			.select(PRODUCT_COLUMNS)
			.find(id)
			.first::<Product>(connection).unwrap();

		let products_with_prices =
			PriceProduct::belonging_to(&product).load::<PriceProduct>(connection)?;

		Ok((product, products_with_prices))
	}

	pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
		// use crate::db_connection::establish_connetion;
		use crate::schema::products::dsl;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		// let connection = establish_connetion();

		diesel::delete(dsl::products.find(&id)).execute(connection)?;
		Ok(())
	}

	pub fn update(
		id: &i32,
		new_product: &NewProduct,
		connection: &PgConnection,
	) -> Result<(), diesel::result::Error> {
		// use crate::db_connection::establish_connetion;
		use crate::schema::products::dsl;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		// let connection = establish_connetion();

		diesel::update(dsl::products.find(id))
			.set(new_product)
			.execute(connection)?;
		Ok(())
	}
}

impl ProductList {
	pub fn list(connection: &PgConnection) -> Self {
		// use crate::db_connection::establish_connetion;
		use crate::schema::products::dsl::*;
		use diesel::QueryDsl;
		use diesel::RunQueryDsl;

		// let connection = establish_connetion();

		let result = products
			.limit(10)
			.load::<Product>(connection)
			.expect("Error loading products");

		ProductList(result)
	}
}

impl NewProduct {
	pub fn create(&self, connection: &PgConnection) -> Result<Product, diesel::result::Error> {
		// use crate::db_connection::establish_connetion;
		use diesel::RunQueryDsl;

		// let connection = establish_connetion();
		diesel::insert_into(products::table)
			.values(self)
			.get_result(connection)
	}
}
