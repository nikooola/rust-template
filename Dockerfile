# might wanna change this
FROM rust:latest AS build

ARG RUNTIME_DEPS="libpq-dev"

WORKDIR /usr/src/myapp

COPY . .

RUN cargo clean && cargo build --release


FROM debian:bookworm

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3333


WORKDIR /usr/src/myapp
COPY --from=build /usr/src/myapp/target/release/rust-api .
EXPOSE 3333

CMD ["./rust-api"]