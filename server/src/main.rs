use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};

#[get("/{filename:.*}")]
async fn index(params: web::Path<String>) -> Result<NamedFile> {
    let filename = params.into_inner();
    let path = format!("dist/{}", filename);

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run();

    println!("server running on :8090");
    server.await
}
