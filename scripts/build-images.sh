#!/usr/bin/env bash
# Build (and optionally push) every phoxal platform runtime image.
#
# Usage:
#   ./scripts/build-images.sh                        # local build, single arch, no push
#   VERSION=0.1.0 PUSH=1 ./scripts/build-images.sh   # multi-arch, push to GHCR

set -euo pipefail

VERSION="${VERSION:-0.0.0-dev}"
PUSH="${PUSH:-0}"
REGISTRY="${REGISTRY:-ghcr.io/phoxal}"
DOCKERFILE="${DOCKERFILE:-Dockerfile.runtime}"

RUNTIMES=(
  asset drive explore follow frame joint localize map mission motion
  odometry perception plan power presence router safety video
)

if [[ "$PUSH" == "1" ]]; then
  PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"
  OUTPUT_FLAG="--push"
else
  ARCH="$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/')"
  PLATFORMS="linux/${ARCH}"
  OUTPUT_FLAG="--load"
fi

docker buildx inspect phoxal-builder >/dev/null 2>&1 \
  || docker buildx create --name phoxal-builder --use

REVISION="${GITHUB_SHA:-$(git rev-parse HEAD)}"

for runtime in "${RUNTIMES[@]}"; do
  echo "Building ${REGISTRY}/runtime-${runtime}:${VERSION}"
  docker buildx build \
    --file "$DOCKERFILE" \
    --platform "$PLATFORMS" \
    --build-arg "RUNTIME_NAME=${runtime}" \
    --tag "${REGISTRY}/runtime-${runtime}:${VERSION}" \
    --tag "${REGISTRY}/runtime-${runtime}:latest" \
    --label "org.opencontainers.image.source=https://github.com/phoxal/framework" \
    --label "org.opencontainers.image.version=${VERSION}" \
    --label "org.opencontainers.image.revision=${REVISION}" \
    --label "org.opencontainers.image.title=phoxal-runtime-${runtime}" \
    --label "org.opencontainers.image.licenses=AGPL-3.0-only" \
    --builder phoxal-builder \
    ${OUTPUT_FLAG} \
    .
done

echo "Built ${#RUNTIMES[@]} runtime images at version ${VERSION}"
