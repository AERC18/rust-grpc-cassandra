FROM rust:1.41.0 AS builder
WORKDIR /usr/src/
RUN USER=root cargo new myapp
WORKDIR /usr/src/myapp
COPY proto ./proto
COPY certs ./certs
COPY build.rs Cargo.toml Cargo.lock ./
RUN ls -la && pwd && ls -la proto/
RUN rustup component add rustfmt 
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN  cargo build --release
COPY src src
#RUN rustup component add rustfmt --toolchain 1.39.0-x86_64-unknown-linux-gnu
RUN rustc --version && cargo build --release && pwd && ls -la && ls -la target/release/

FROM debian:buster
ENV CASSANDRA_URI="cassandra.us-east-2.amazonaws.com:9142"
ENV CASSANDRA_SSL_CERT_PATH="/usr/src/AmazonRootCA1.pem"
ENV CASSANDRA_USER="YOUR CASSANDRA USER"                                                                        
ENV CASSANDRA_PASSWORD="YOUR CASSANDRA PASSWORD"
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY --from=builder /usr/src/myapp/certs/AmazonRootCA1.pem /usr/src/
COPY --from=builder /usr/src/myapp/target/release/blocking-service /bin/
RUN ls -la /bin/
CMD ["blocking-service"] 
