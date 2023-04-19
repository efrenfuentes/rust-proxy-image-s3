mod utils;
mod s3;

use actix_web::{get, middleware::Logger, web, App, HttpServer, Result, Error, error::ErrorInternalServerError};
use actix_files::NamedFile;
use serde::Deserialize;
use env_logger::Env;
use std::path::Path;
use std::ffi::OsStr;

use utils::validations::{validate_extension, validate_resolution};
use utils::transform::transform_image;
use crate::s3::download::download_image;

#[derive(Deserialize)]
struct QueryInfo {
    key: String,
}

#[derive(Deserialize)]
struct PathInfo {
    resolution: String,
}

#[get("/image/{resolution}")]
async fn image(path_info: web::Path<PathInfo>, query_info: web::Query<QueryInfo>) -> Result<NamedFile> {
    let extension = Path::new(query_info.key.as_str()).extension().and_then(OsStr::to_str).unwrap_or("");

    if !validate_resolution(&path_info.resolution) {
        return Err(Error::from(ErrorInternalServerError("Invalid resolution")));
    }

    if !validate_extension(extension) {
        return Err(Error::from(ErrorInternalServerError("Invalid extension")));
    }

    let output_file = download_image(query_info.key.clone()).await;

    match output_file {
        Ok(output_file) => transform_image(path_info.resolution.clone(), output_file).await,
        Err(e) => Err(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
