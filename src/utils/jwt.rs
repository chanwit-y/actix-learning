use actix_web::HttpResponse;
use chrono::{Duration, Local};
use jwt::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	sub: String,
	company: String,
	exp: i64,
}

pub struct SlimUser {
	pub email: String,
	pub company: String,
}

impl From<Claims> for SlimUser {
	fn from(claims: Claims) -> Self {
		SlimUser {
			email: claims.sub,
			company: claims.company,
		}
	}
}

impl Claims {
	fn with_email(email: &str, company: &str) -> Self {
		let x = Local::now() + Duration::hours(24);
		Claims {
			sub: email.into(),
			company: company.into(),
			exp: x.timestamp(),
		}
	}
}

pub fn create_token(email: &str, company: &str) -> Result<String, HttpResponse> {
	let claims = Claims::with_email(email, company);
	encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret("secret".as_ref()),
	)
	.map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
	decode::<Claims>(
		token,
		&DecodingKey::from_secret("secret".as_ref()),
		&Validation::default(),
	)
	.map(|data| data.claims.into())
	.map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}

// fn get_secret<'a>() -> &'a [u8] {
// 	dotenv!("JWT_SECRET").as_bytes()
// }
