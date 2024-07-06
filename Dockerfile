FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config

RUN cargo install
RUN cargo install diesel_cli --no-default-features --features "postgres"

RUN cargo build --release

# wait until the database is ready to run the migrations
RUN while ! diesel migration run; do sleep 1; done

