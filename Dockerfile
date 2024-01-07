FROM rust:1.67 as builder
ARG APP_NAME
WORKDIR /usr/src/app

RUN cargo init
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm -rf src

COPY ./src ./src

RUN APP_FOLDER=$(echo "${APP_NAME}" | sed 's/-/_/g') && rm target/release/deps/${APP_FOLDER}*

RUN cargo build --release

FROM debian:bullseye-slim
ARG APP_NAME

COPY --from=builder /usr/src/app/target/release/${APP_NAME} /usr/local/bin/app
CMD "app"
