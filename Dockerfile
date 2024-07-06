FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config

RUN cargo build --release

FROM debian:latest as runtime

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config
RUN mkdir -p /usr/src/app/migrations

COPY --from=builder /usr/src/app/target/release/s2t .
COPY --from=builder /usr/src/app/.env .
COPY --from=builder /usr/src/app/migrations/* ./migrations/
COPY --from=builder /usr/src/app/Cargo.toml .


CMD [ "./s2t" ]