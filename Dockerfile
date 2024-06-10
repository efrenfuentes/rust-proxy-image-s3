FROM rust:1.78
RUN apt-get clean \
  && apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*
RUN mkdir /app
ADD . /app
WORKDIR /app
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/rust-proxy-image-s3"]
