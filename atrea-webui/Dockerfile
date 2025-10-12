# buid it
# https://hub.docker.com/_/rust
FROM docker.io/library/rust:bookworm as build
COPY Cargo.toml Cargo.lock Rocket.toml /mnt/
COPY src /mnt/src/
RUN cd /mnt && cargo build --release --features docker

# compress it for the registry
# https://github.com/GoogleContainerTools/distroless
FROM gcr.io/distroless/cc-debian12
COPY --from=build /mnt/target/release/atrea-webui /
COPY ./plugins /builtin_plugins
COPY Rocket.toml /
WORKDIR "/"
ENTRYPOINT ["/atrea-webui"]
