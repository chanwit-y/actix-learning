use actix_web::{web, guard, HttpServer, App, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www"))    
                    .route("", web::to(|| async {HttpResponse::Ok().body("www")}))
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "user"))
                    .route("", web::to(|| async {HttpResponse::Ok().body("user")} ))
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}