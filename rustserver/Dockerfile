FROM rust:1.58.0
WORKDIR /usr/src/rustserver
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
CMD cargo run --release