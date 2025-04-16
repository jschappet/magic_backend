use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware as mw};
use actix_files as fs;

mod form_data;
use form_data::FormData;
use env_logger::Env;
use serde_json::json;
use std::f32::consts::E;
use std::fs::OpenOptions;
use std::io::{Read, Write};

mod state;
use state::AppState;
use handlebars::Handlebars;
use std::sync::Arc;

async fn index<'hb>(    data: web::Data<AppState<'hb>>) -> impl Responder {
    let hb = &data.hb;
    match hb.render("index",  &json!({})) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }
}

fn save_response_to_file(form: FormData) -> std::io::Result<()> {
    
    
    log::debug!("Parsed form data: {:?}", form);


    let json_data = serde_json::to_string(&form).unwrap();
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("responses.log")
        .expect("Failed to open file");

    if let Err(e) = writeln!(file, "{}", json_data) {
        eprintln!("Failed to write to log file: {}", e);
        return Err(e);
    }
    if let Err(e) = writeln!(file, ",") {
        eprintln!("Failed to write to log file: {}", e);
        return Err(e);
    }
    Ok(())
}



async fn they_said_yes<'hb>(     
    form: web::Form<FormData>,     
    data: web::Data<AppState<'hb>>) -> impl Responder {
    let hb = &data.hb;

    //save_response_to_file(form.into_inner()).expect("Failed to save response");
    match hb.render("yes", &json!({"form": form.into_inner()})) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }
}

async fn they_said_no<'hb>(     
    form: web::Form<FormData>,     
    data: web::Data<AppState<'hb>>) -> impl Responder {
    let hb = &data.hb;
    log::debug!("Parsed form data: {:?}", form);
    save_response_to_file(form.into_inner()).expect("Failed to save response");

    match hb.render("no",  &json!({})) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }
}

async fn delete_responses() -> impl Responder {
    let file = match OpenOptions::new().write(true).truncate(true).open("responses.log") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return HttpResponse::InternalServerError().body("Error deleting responses");
        }
    };

    if let Err(e) = file.set_len(0) {
        eprintln!("Failed to truncate file: {}", e);
        return HttpResponse::InternalServerError().body("Error deleting responses");
    }

    HttpResponse::Ok().body("DONE")
}

async fn read_responses() -> impl Responder {
    let mut file = match OpenOptions::new().read(true).open("responses.log") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return HttpResponse::InternalServerError().body("Error reading responses");
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Failed to read file: {}", e);
        return HttpResponse::InternalServerError().body("Error reading responses");
    }
    // Trim off the last comma and add closing bracket
    log::debug!("Contents: {:?}", contents);
    let contents = contents.trim_end_matches('\n').to_string() ;
    let contents = contents.trim_end_matches(',').to_string();
    let contents = format!("[{}]", contents);
    HttpResponse::Ok().body(contents)
}


async fn submit_form<'hb>(
    form: web::Form<FormData>,     
    data: web::Data<AppState<'hb>>)
        -> impl Responder {
    
    let hb = &data.hb;
    match hb.render("thankyou-message",  &json!({"form": form.into_inner()})) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }

}

async fn completed_form<'hb>(
    form: web::Form<FormData>,     
    data: web::Data<AppState<'hb>>)
        -> impl Responder {
    log::debug!("Parsed form data: {:?}", form);
    let form = form.into_inner();
    save_response_to_file(form.clone()).expect("Failed to save response");
    let hb = &data.hb;
    match hb.render("done",  &json!({"form": form})) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("thankyou-message", "templates/thankyou-message.hbs")
        .expect("Failed to register template");
    handlebars
        .register_template_file("index", "templates/index.hbs")
        .expect("Failed to register template");

        handlebars
        .register_template_file("yes", "templates/yes.hbs")
        .expect("Failed to register template");

        handlebars
        .register_template_file("no", "templates/no.hbs")
        .expect("Failed to register template");

        handlebars
        .register_template_file("done", "templates/done.hbs")
        .expect("Failed to register template");

    let handlebars = Arc::new(handlebars);
    let state = AppState {
        hb: handlebars.clone(),
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    log::info!("Starting server...");
    HttpServer::new(move || {
        App::new()
        .wrap(mw::Logger::default())
        .app_data(web::Data::new(state.clone())) // Share state with all handlers
            .service(
                fs::Files::new("/m/static", "static")
                    .index_file("index.html"), // Serve static files from /static
            )
            //.route("/", web::get().to(index))
            .route("/m/submit", web::post().to(submit_form))
            .route("/m/start", web::get().to(index))
            .route("/m/VybmFtZSI6ImFkbW", web::get().to(read_responses))
            .route("/m/VybmFtZSI6ImFkbW", web::delete().to(delete_responses))
            .route("/m/yes", web::post().to(they_said_yes))
            .route("/m/no", web::post().to(they_said_no))

            .route("/m/done", web::post().to(completed_form))

        })

            .bind("127.0.0.1:8581")?
    .run()
    .await
}



