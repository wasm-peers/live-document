#!/usr/bin/env just --justfile
set dotenv-load := true

list:
    @just --list

help:
    @just list

fmt *FLAGS:
    cargo +nightly fmt {{FLAGS}}

check *FLAGS:
    cargo check # test for trivial errors first
    cargo clippy --tests --examples {{FLAGS}} # thorough check second

build *FLAGS:
    trunk build {{FLAGS}}

run *FLAGS:
    trunk serve {{FLAGS}}

pre-commit:
    @just fmt
    @just check -- -D warnings
    cargo doc --no-deps --all-features
    cargo spellcheck fix

coverage *FLAGS:
    cargo llvm-cov {{FLAGS}} --open

thorough-check:
    cargo +nightly udeps --all-targets
    cargo audit
    cargo upgrades
    @just unused-features

unused-features:
    unused-features analyze
    unused-features build-report --input report.json
    rm report.json
    mv report.html /tmp
    xdg-open /tmp/report.html

init:
    echo # installing git hooks
    pre-commit --version || pip install pre-commit
    pre-commit install || echo "failed to install git hooks!" 1>&2
    echo # installing nightly used by `just fmt` and `cargo udeps`
    rustup install nightly
    echo # installing cargo-binstall for faster setup time
    cargo binstall -V || cargo install cargo-binstall
    echo # things required by `just test`
    cargo binstall cargo-nextest --no-confirm
    echo # things required by `just watch`
    cargo binstall cargo-watch --no-confirm
    echo # things required by `just pre-commit`
    cargo binstall cargo-spellcheck --no-confirm
    echo # things required by `just coverage`
    rustup component add llvm-tools-preview
    cargo binstall cargo-llvm-cov --no-confirm
    echo # things required by `just benchmark`
    cargo binstall cargo-criterion --no-confirm
    echo # things required by `just thorough-check`
    cargo binstall cargo-udeps --no-confirm
    cargo binstall cargo-audit --no-confirm
    cargo binstall cargo-upgrades --no-confirm
    cargo binstall cargo-unused-features --no-confirm
