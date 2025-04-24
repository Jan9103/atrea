# buid it
# https://hub.docker.com/_/rust
FROM docker.io/library/rust:bookworm as build
COPY Cargo.toml Cargo.lock /mnt/
COPY src /mnt/src/
RUN cd /mnt && cargo build --release --locked

# compress it for the registry
# https://github.com/GoogleContainerTools/distroless
FROM gcr.io/distroless/cc-debian12
LABEL org.opencontainers.image.source=https://github.com/jan9103/atrea
LABEL org.opencontainers.image.licenses=AGPL-3.0-only
COPY --from=build /mnt/target/release/atrea-collector /
ENTRYPOINT ["/atrea-collector"]
