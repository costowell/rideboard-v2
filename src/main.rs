use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use include_dir::{include_dir, Dir};
use log::info;
use oauth2::basic::BasicClient;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod api;
mod auth;
mod pings;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    google_oauth: BasicClient,
    csh_oauth: BasicClient,
}

// Embed the 'static' directory into the binary
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/frontend/dist");

async fn serve_file(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();
    if let Some(file) = STATIC_DIR.get_file(&file_path) {
        let content = file.contents();
        let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
        HttpResponse::Ok().content_type(mime.as_ref()).body(content)
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

async fn serve_index() -> impl Responder {
    if let Some(file) = STATIC_DIR.get_file("index.html") {
        let content = file.contents();
        let mime = mime_guess::from_path("index.html").first_or_octet_stream();
        HttpResponse::Ok().content_type(mime.as_ref()).body(content)
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let host_inner = host.clone();
    let port: i32 = match &std::env::var("PORT").map(|port| port.parse()) {
        Ok(Ok(p)) => *p,
        Ok(Err(_)) => 8080,
        Err(_) => 8080,
    };

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");

    let session_key = Key::generate();

    info!("Starting server at http://{host}:{port}");
    HttpServer::new(move || {
        let (google_client, csh_client) = auth::get_clients(&host_inner, port);

        App::new()
            .app_data(web::Data::new(AppState {
                db: db_pool.clone(),
                google_oauth: google_client,
                csh_oauth: csh_client,
            }))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_key.clone(),
            ))
            .wrap(Logger::default())
            .service(api::scope())
            .route("/", web::get().to(serve_index))
            .route("/about", web::get().to(serve_index))
            .route("/{filename:.*}", web::get().to(serve_file))
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
