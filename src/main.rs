use actix_files as fs;
use actix_web::{web, App, HttpServer, Result};
use fs::NamedFile;
use std::path::PathBuf;

async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./templates/index.html".parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on localhost:8080...");
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", ".").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
