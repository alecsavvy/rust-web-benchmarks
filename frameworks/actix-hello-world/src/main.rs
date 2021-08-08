use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/", web::get().to(serve)))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

async fn serve(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}
