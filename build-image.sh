#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
cd "$repo_root"

test -z "$(git status --porcelain)" || {
    echo 'FAIL build-image reason=dirty_worktree' >&2
    exit 1
}

SOURCE_REVISION=$(git rev-parse HEAD)
BUILD_VERSION=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
DEBIAN_SNAPSHOT=${DEBIAN_SNAPSHOT:-20261118T000000Z}
case "$SOURCE_REVISION" in [0-9a-f][0-9a-f]*) ;; *) echo 'FAIL build-image reason=invalid_revision' >&2; exit 1 ;; esac
case "$BUILD_VERSION" in '' | *[!0-9A-Za-z._+-]*) echo 'FAIL build-image reason=invalid_version' >&2; exit 1 ;; esac
case "$DEBIAN_SNAPSHOT" in [0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]T[0-9][0-9][0-9][0-9][0-9][0-9]Z) ;; *) echo 'FAIL build-image reason=invalid_snapshot' >&2; exit 1 ;; esac

export SOURCE_REVISION BUILD_VERSION DEBIAN_SNAPSHOT
docker compose build --pull ecowitt_exporter

image="ecowitt-exporter:$SOURCE_REVISION"
test "$(docker image inspect --format '{{index .Config.Labels "org.opencontainers.image.revision"}}' "$image")" = "$SOURCE_REVISION"
test "$(docker image inspect --format '{{index .Config.Labels "org.opencontainers.image.version"}}' "$image")" = "$BUILD_VERSION"
docker run --rm --entrypoint /usr/bin/test "$image" -x /usr/local/bin/ecowitt_exporter
echo "PASS image=$image revision=$SOURCE_REVISION version=$BUILD_VERSION snapshot=$DEBIAN_SNAPSHOT"

