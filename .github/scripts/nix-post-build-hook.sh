#!/usr/bin/env bash
set -f # disable globbing
export IFS=' '

echo "${OUT_PATHS}" >> /tmp/nix-built-paths.txt
