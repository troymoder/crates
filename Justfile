default:
    @just --list

fmt:
    dprint fmt
    buf format -w --disable-symlinks --debug
    just --unstable --fmt
    shfmt -w .
    nix fmt .

lint:
    cargo clippy --fix --allow-dirty --allow-staged

deny:
    cargo deny check

test:
    RUSTC_BOOTSTRAP=1 INSTA_FORCE_PASS=1 cargo llvm-cov --no-report nextest
    RUSTC_BOOTSTRAP=1 INSTA_FORCE_PASS=1 cargo llvm-cov --no-report --doc
    RUSTC_BOOTSTRAP=1 INSTA_FORCE_PASS=1 cargo llvm-cov report --doctests --lcov --output-path lcov.info
    cargo insta review

check:
    cargo check --all-targets --all-features

build:
    cargo build --all-targets --all-features

doc:
    cargo doc --all-features --no-deps

doc-serve: doc
    miniserve target/doc

sync-readme:
    cargo run -p cargo-sync-readme2 -- workspace --target-dir target/sync-readme sync

sync-readme-test:
    cargo run -p cargo-sync-readme2 -- workspace --target-dir target/sync-readme test
