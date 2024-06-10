use actix_files::NamedFile;
use actix_web::{error::ErrorInternalServerError, Result};
use image::imageops::FilterType;
use std::fs;

pub async fn transform_image(resolution: String, filename: String) -> Result<NamedFile> {
    if resolution == "original" {
        return serve_file(filename.clone());
    }

    let output_file = format!("{}_{}", resolution, filename);
    let img = image::open(filename.clone()).unwrap();
    let (width, height) = get_geometry(resolution);

    img.resize_to_fill(width, height, FilterType::Lanczos3)
        .save(output_file.clone())
        .unwrap();

    let result = serve_file(output_file);

    let removed = fs::remove_file(filename);
    match removed {
        Ok(_file) => (),
        Err(_e) => return Err(ErrorInternalServerError("Invalid file")),
    }

    result
}

fn serve_file(filename: String) -> Result<NamedFile> {
    let result = NamedFile::open(filename.clone());
    match result {
        Ok(file) => {
            let removed = fs::remove_file(filename.clone());
            match removed {
                Ok(_file) => (),
                Err(_e) => {
                    return Err(ErrorInternalServerError(format!(
                        "File not found: {}",
                        filename
                    )))
                }
            }
            Ok(file)
        }
        Err(_e) => Err(ErrorInternalServerError(format!(
            "File not found: {}",
            filename
        ))),
    }
}

fn get_geometry(resolution: String) -> (u32, u32) {
    let geometry: Vec<&str> = resolution.split('x').collect();

    let width = geometry[0].parse::<u32>().unwrap();
    let height = geometry[1].parse::<u32>().unwrap();

    (width, height)
}
