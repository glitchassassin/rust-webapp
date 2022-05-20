FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo install perseus-cli wasm-pack
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cd /app/wasm-client && perseus deploy

FROM scratch
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/server /app/server
COPY --from=builder /app/wasm-client/pkg /app/client
EXPOSE 8000
ENTRYPOINT ["./entrypoint.sh"]