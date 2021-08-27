FROM rust
RUN cargo install sqlx-cli --no-default-features --features postgres
ENV DATABASE_URL=postgres://postgres:123@postgres/aerofans
WORKDIR /usr/src/rustserver
COPY . .
RUN cargo build --release
CMD sqlx migrate run; cargo run --release