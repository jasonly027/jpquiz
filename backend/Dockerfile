FROM rust:slim-trixie AS chef

RUN cargo install --locked cargo-chef

WORKDIR /app

FROM chef AS planner

COPY . .

RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --package server --release --bin server

FROM debian:trixie-slim AS runtime

WORKDIR /app

COPY --from=builder /app/configuration ./configuration

COPY --from=builder /app/target/release/server /usr/local/bin/app

EXPOSE 3000

ENV APP_ENV=production
ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/app"]
