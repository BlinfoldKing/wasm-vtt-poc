use actix_files::NamedFile;

use actix_web::{get, web, Result};

#[get("/{path:.*}")]
async fn index(params: web::Path<String>) -> Result<NamedFile> {
    let path = params.into_inner();

    match path.split("/").collect::<Vec<&str>>()[0] {
        "_app" => Ok(NamedFile::open(format!("dist/{}", path))?),
        _ => Ok(NamedFile::open("dist/index.html")?),
    }
}
