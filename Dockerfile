FROM docker.io/library/rust:1-slim-bookworm AS builder

WORKDIR /usr/src/myapp

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo install --path .

#docker run --rm -it docker.io/library/debian:bookworm-slim bash
FROM docker.io/library/debian:bookworm-slim

LABEL org.opencontainers.image.authors="Johann Queuniet"
LABEL org.opencontainers.image.source="https://github.com/jqueuniet/ecowitt_exporter"
LABEL org.opencontainers.image.description="Republish metrics sent with the Ecowitt weather station protocol to the prometheus format "
LABEL org.opencontainers.image.licenses="AGPL"

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_CLI_COLORS=true
ENV RUST_LOG=normal

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/ecowitt_exporter /usr/local/bin/

RUN adduser \
    --quiet \
    --system \
    --no-create-home \
    --group \
    --disabled-password \
    exporter

WORKDIR /data

USER exporter

ENTRYPOINT ["ecowitt_exporter"]
