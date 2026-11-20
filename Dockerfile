FROM rust:1.100-bookworm AS builder

WORKDIR /workspace

COPY Cargo.toml Cargo.lock LICENSE README.md ./
COPY runtime ./runtime
COPY macros ./macros

RUN cargo package --locked -p arcadia-macros \
    && cargo package --locked -p arcadia-framework \
    && mkdir /packages \
    && cp target/package/*.crate /packages/

FROM scratch

LABEL org.opencontainers.image.title="Arcadia Framework packages"
LABEL org.opencontainers.image.description="Published Cargo packages for the Arcadia Rust backend framework"
LABEL org.opencontainers.image.source="https://github.com/azomura/arcadia-framework"
LABEL org.opencontainers.image.version="0.1.0"

COPY --from=builder /packages/ /packages/
