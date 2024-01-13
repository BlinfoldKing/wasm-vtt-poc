use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};

#[get("/")]
async fn index() -> Result<NamedFile> {
    let filename = "index.html";
    let path = format!("dist/{}", filename);

    Ok(NamedFile::open(path)?)
}

#[get("/{filename:.*}")]
async fn dist(params: web::Path<String>) -> Result<NamedFile> {
    let filename = params.into_inner();
    let path = format!("dist/{}", filename);

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var_os("PORT") {
        Some(v) => v.to_str().unwrap().parse().unwrap(),
        None => 8080,
    };
    let server = HttpServer::new(|| App::new().service(index).service(dist))
        .bind(("127.0.0.1", port))?
        .run();

    println!("server running on {}", port);
    server.await
}
