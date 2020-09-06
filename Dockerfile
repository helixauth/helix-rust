# ------------------------------------------------------------------------------
# Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as build

RUN mkdir -p /usr/src/helix

WORKDIR /usr/src/helix

COPY . /usr/src/helix

RUN cargo build --release


# # ------------------------------------------------------------------------------
# # Production Stage
# # ------------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/helix/target/release/helix /usr/local/bin/helix

CMD ["helix"]
