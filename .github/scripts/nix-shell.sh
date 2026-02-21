#!/usr/bin/env bash

shell="${NIX_SHELL:-default}"
nix develop "${GITHUB_WORKSPACE:-.}#${shell}" --impure --command bash -e "$@"
