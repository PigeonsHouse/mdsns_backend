FROM rust:1.67

RUN mkdir -p /opt/webapi

WORKDIR /opt/webapi
COPY . .

RUN cargo build --release
