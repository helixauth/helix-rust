# # ------------------------------------------------------------------------------
# # Build Stage
# # ------------------------------------------------------------------------------

FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/helix

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/helix*

COPY . .

RUN cargo build --release

RUN cargo install --path .


# # ------------------------------------------------------------------------------
# # Production Stage
# # ------------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/helix /usr/local/bin/helix

CMD ["helix"]
