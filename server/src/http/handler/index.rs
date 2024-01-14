use actix_files::NamedFile;

use actix_web::{get, web, Result};

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
