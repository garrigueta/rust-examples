FROM rust:latest
RUN rustup target wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
EXPOSE 8080
VOLUME [ "/code" ]
WORKDIR /code
CMD [ "trunk", "serve" ]