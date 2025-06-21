FROM rust:latest AS builder
WORKDIR /app
COPY . .

RUN cargo build --release


FROM debian:latest

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/auth_api /app/

CMD ["/app/auth_api"]