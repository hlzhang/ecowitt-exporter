FROM docker.io/library/rust:1-slim-bookworm@sha256:86234e857a413004348b471d11a4ebe1aa92d2eae2b6b143cac68fd1ac3c48be AS builder

ARG DEBIAN_SNAPSHOT=20261118T000000Z

WORKDIR /usr/src/myapp

RUN printf '%s\n' \
      "deb [check-valid-until=no] https://snapshot.debian.org/archive/debian/${DEBIAN_SNAPSHOT} bookworm main" \
      "deb [check-valid-until=no] https://snapshot.debian.org/archive/debian/${DEBIAN_SNAPSHOT} bookworm-updates main" \
      "deb [check-valid-until=no] https://snapshot.debian.org/archive/debian-security/${DEBIAN_SNAPSHOT} bookworm-security main" \
      >/etc/apt/sources.list \
    && rm -f /etc/apt/sources.list.d/debian.sources \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
       pkg-config=1.8.1-1 \
       libssl-dev=3.0.22-1~deb12u2 \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --locked --release

FROM docker.io/library/debian:bookworm-slim@sha256:ad86386827b083b3d71139050b47ffb32bbd9559ea9b1345a739b14fec2d9ecf

ARG DEBIAN_SNAPSHOT=20261118T000000Z
ARG VCS_REF
ARG BUILD_VERSION

LABEL org.opencontainers.image.authors="Johann Queuniet"
LABEL org.opencontainers.image.source="https://github.com/hlzhang/ecowitt-exporter"
LABEL org.opencontainers.image.description="Republish Ecowitt weather-station submissions as Prometheus metrics"
LABEL org.opencontainers.image.licenses="AGPL-3.0"
LABEL org.opencontainers.image.revision="${VCS_REF}"
LABEL org.opencontainers.image.version="${BUILD_VERSION}"

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_CLI_COLORS=true
ENV RUST_LOG=normal

RUN printf '%s\n' \
      "deb [check-valid-until=no] https://snapshot.debian.org/archive/debian/${DEBIAN_SNAPSHOT} bookworm main" \
      "deb [check-valid-until=no] https://snapshot.debian.org/archive/debian/${DEBIAN_SNAPSHOT} bookworm-updates main" \
      "deb [check-valid-until=no] https://snapshot.debian.org/archive/debian-security/${DEBIAN_SNAPSHOT} bookworm-security main" \
      >/etc/apt/sources.list \
    && rm -f /etc/apt/sources.list.d/debian.sources \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates=20250419~deb12u2 \
       libssl3=3.0.22-1~deb12u2 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/myapp/target/release/ecowitt_exporter /usr/local/bin/

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
