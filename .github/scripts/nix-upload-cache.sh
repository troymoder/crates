#!/usr/bin/env bash

set -euo pipefail

if [[ ! -f /tmp/nix-cache-key.pem ]]; then
    echo "Error: /tmp/nix-cache-key.pem not found"
    exit 1
fi

if [[ -z "${NIX_CACHE_URL:-}" ]]; then
    echo "Error: NIX_CACHE_URL not set"
    exit 1
fi

if [[ -s /tmp/nix-built-paths.txt ]]; then
    echo "Uploading $(wc -l < /tmp/nix-built-paths.txt) paths to cache..."
    nix copy \
        --to "${NIX_CACHE_URL}&secret-key=/tmp/nix-cache-key.pem&multipart-upload=true" \
        $(sort -u /tmp/nix-built-paths.txt | tr '\n' ' ') \
        || echo "Warning: nix copy failed, continuing anyway"
else
    echo "No paths were built locally"
fi
