FROM rust:1.88

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

CMD ["./target/release/backend"]
