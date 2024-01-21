use actix_web::{web, App, HttpServer, Responder};

async fn saluer() -> impl Responder {
    format!("Bonjour, monde avec Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(saluer))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

