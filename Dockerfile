FROM rust:1.62.0-slim-buster
WORKDIR /leopardybot-src
COPY . /leopardybot-src
RUN apt-get update -y \
    && apt-get install -y libssl-dev pkg-config
RUN cargo build --release --bin leopardybot
ENTRYPOINT ["/leopardybot-src/target/release/leopardybot"]

#FROM rust:1.62.0-slim-buster as planner
#WORKDIR /leopardybot-src
#RUN cargo install cargo-chef
#COPY . /leopardybot-src
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM rust:1.62.0-slim-buster as cacher
#WORKDIR /leopardybot-src
#COPY --from=planner /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
#COPY --from=planner /leopardybot-src/recipe.json recipe.json
#RUN apt-get update -y \
#    && apt-get install -y libssl-dev pkg-config
#RUN cargo chef cook --release --recipe-path recipe.json
#
#FROM rust:1.62.0-slim-buster as builder
#WORKDIR /leopardybot-src
#COPY . /leopardybot-src
#COPY --from=cacher /leopardybot-src/target target
#COPY --from=cacher /usr/local/cargo /usr/local/cargo
#RUN apt-get update -y && apt-get install -y libssl-dev
#RUN cargo build --release --bin leopardybot
#
#FROM debian:buster-slim AS runtime
#WORKDIR /leopardybot-src
#COPY --from=builder /leopardybot-src/target/release/leopardybot leopardybot
#ENTRYPOINT ["/leopardybot-src/leopardybot"]
