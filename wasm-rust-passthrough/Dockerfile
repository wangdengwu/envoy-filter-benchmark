FROM rust:1.71.1 as builder
WORKDIR /usr/src/proxy-wasm-example-rust
COPY . .
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release


FROM  envoyproxy/envoy:contrib-v1.26.4
COPY --from=builder /usr/src/proxy-wasm-example-rust/target/wasm32-wasi/release/rust.wasm /etc/envoy/rust.wasm
RUN  apt update && apt install -y curl