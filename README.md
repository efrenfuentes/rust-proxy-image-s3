# rust-proxy-image-s3

Rust Proxy Image retrieves image files from a S3 bucket, resizing them based on url parameters and serves them.

## Configuration

The following environment variables are required:

* `AWS_ACCESS_KEY_ID`
* `AWS_SECRET_ACCESS_KEY`
* `AWS_REGION`
* `AWS_BUCKET`

## Usage

Once up and running, you can access image files stored in the S3 bucket like so:

    http://localhost:8080/image/640x640?key=image.jpg

The above will retrieve the image from the S3 bucket, resize it to 640x640 and serve it.

If you want the original image, you can do:

    http://localhost:8080/image/original?key=image.jpg
