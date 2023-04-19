use s3::bucket::Bucket;
use s3::creds::Credentials;
use awsregion::Region;
use std::fs::File;
use std::env;
use std::io::Write;
use actix_web::{Error, Result, error::ErrorInternalServerError};

pub async fn download_image(key: String) -> Result<String> {
  let filename = key.replace("/", "_");

  let bucket_name = get_env_key("AWS_BUCKET");
  let region = get_env_key("AWS_REGION");

  if bucket_name.is_none() || region.is_none() {
    return Err(Error::from(ErrorInternalServerError("Invalid environment variables")));
  }

  let aws_region: Region = match region.unwrap().parse() {
    Ok(region) => region,
    Err(_e) => return Err(Error::from(ErrorInternalServerError("Invalid region"))),
  };

  let credentials = Credentials::default();
  let bucket = Bucket::new(bucket_name.unwrap().as_str(), aws_region, credentials.unwrap());

  match bucket {
    Ok(bucket) => download_from_bucket(bucket, key, filename).await,
    Err(_e) => Err(Error::from(ErrorInternalServerError("Invalid bucket"))),
  }

}


async fn download_from_bucket(bucket: Bucket, key: String, filename: String) -> Result<String> {
  let response_data = bucket.get_object(key.as_str()).await;

  match response_data {
    Ok(response_data) => write_file(response_data, filename).await,
    Err(_e) => Err(Error::from(ErrorInternalServerError("Invalid response"))),
  }
}

async fn write_file(response_data: s3::request_trait::ResponseData, filename: String) -> Result<String> {
  let mut file = File::create(filename.clone())?;
  file.write_all(response_data.bytes())?;

  return Ok(filename);
}

fn get_env_key(key: &str) -> Option<String> {
  match env::var(key) {
    Ok(val) => Some(val),
    Err(_e) => None,
  }
}