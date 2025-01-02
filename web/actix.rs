use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "¡Hola desde Actix-web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello)) // Ruta que responde con "¡Hola desde Actix-web!"
    })
    .bind("127.0.0.1:8080")? // Iniciar el servidor en el puerto 8080
    .run()
    .await
}
