use actix_web::{ Error, Result, error::ErrorInternalServerError };
use image::imageops::FilterType;
use actix_files::NamedFile;
use std::fs;

pub async fn transform_image(resolution: String, filename: String) -> Result<NamedFile> {
  if resolution == "original" {
    let result = NamedFile::open(filename.clone());
    let removed = fs::remove_file(filename);
    match removed {
      Ok(_file) => (),
      Err(_e) => return Err(Error::from(ErrorInternalServerError("Invalid file"))),
    }
    match result {
      Ok(file) => return Ok(file),
      Err(_e) => return Err(Error::from(ErrorInternalServerError("Invalid file"))),
    }
  }

  let output_file = format!("{}_{}", resolution, filename);
  let img = image::open(filename.clone()).unwrap();

  let (width, height) = get_geometry(resolution);

  img.resize_to_fill(width, height, FilterType::Lanczos3)
    .save(output_file.clone())
    .unwrap();

  let result = NamedFile::open(output_file.clone());

  let mut removed = fs::remove_file(filename);
  match removed {
    Ok(_file) => (),
    Err(_e) => return Err(Error::from(ErrorInternalServerError("Invalid file"))),
  }

  removed = fs::remove_file(output_file);
  match removed {
    Ok(_file) => (),
    Err(_e) => return Err(Error::from(ErrorInternalServerError("Invalid file"))),
  }

  match result {
    Ok(file) => return Ok(file),
    Err(_e) => return Err(Error::from(ErrorInternalServerError("Invalid file"))),
  }
}

fn get_geometry(resolution: String) -> (u32, u32) {
  let geometry: Vec<&str> = resolution.split("x").collect();

  let width = geometry[0].parse::<u32>().unwrap();
  let height = geometry[1].parse::<u32>().unwrap();

  return (width, height);
}
