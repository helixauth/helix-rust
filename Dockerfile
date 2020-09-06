# ------------------------------------------------------------------------------
# Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as build

RUN mkdir -p /usr/src/helix

WORKDIR /usr/src/helix

COPY . /usr/src/helix

RUN cargo build --release

RUN pwd

RUN ls

# # ------------------------------------------------------------------------------
# # Production Stage
# # ------------------------------------------------------------------------------

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/helix /usr/local/bin/helix

CMD ["helix"]
