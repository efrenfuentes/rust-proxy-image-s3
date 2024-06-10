mod s3;
mod utils;

use crate::s3::download::download_image;
use actix_files::NamedFile;
use actix_web::{
    error::ErrorInternalServerError, get, middleware::Logger, web, App, HttpServer, Result,
};
use env_logger::Env;
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::Path;
use utils::transform::transform_image;
use utils::validations::{validate_extension, validate_resolution};

#[derive(Deserialize)]
struct QueryInfo {
    key: String,
}

#[derive(Deserialize)]
struct PathInfo {
    resolution: String,
    bucket: String,
}

#[get("/image/{bucket}/{resolution}")]
async fn image(
    path_info: web::Path<PathInfo>,
    query_info: web::Query<QueryInfo>,
) -> Result<NamedFile> {
    let extension = Path::new(query_info.key.as_str())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    if !validate_resolution(&path_info.resolution) {
        return Err(ErrorInternalServerError("Invalid resolution"));
    }

    if !validate_extension(extension) {
        return Err(ErrorInternalServerError("Invalid extension"));
    }

    let output_file = download_image(path_info.bucket.clone(), query_info.key.clone()).await;

    match output_file {
        Ok(output_file) => transform_image(path_info.resolution.clone(), output_file).await,
        Err(e) => Err(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(Logger::default()).service(image))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
