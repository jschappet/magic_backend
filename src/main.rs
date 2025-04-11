use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files as fs;

mod form_data;
use form_data::FormData;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn submit_form(form: web::Json<FormData>) -> impl Responder {
    HttpResponse::Ok().json(form.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                fs::Files::new("/s", "static")
                    .index_file("index.html"), // Serve static files from /static
            )
            .route("/", web::get().to(index))
            .route("/greet/{name}", web::get().to(greet))
            .route("/submit", web::post().to(submit_form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
