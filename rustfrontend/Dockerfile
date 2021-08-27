FROM rust
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/src/rustfrontend
COPY . .
RUN trunk build --release
WORKDIR /usr/src/rustfrontend/dist
CMD python3 -m http.server 8080