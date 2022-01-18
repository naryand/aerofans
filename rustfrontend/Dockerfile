FROM rust:1.58.0
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/src/rustfrontend
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./index.html ./index.html
RUN trunk build --release
CMD trunk serve --release