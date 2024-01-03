#!/usr/bin/env just --justfile

lint:
  cargo clippy

release-macos-arm:
  cargo build --release --target aarch64-apple-darwin

release-macos-x86:
  cargo build --release --target x86_64-apple-darwin

release-macos: release-macos-arm release-macos-x86
  mkdir -p build
  lipo -create -output build/installer-macos \
    target/aarch64-apple-darwin/release/ravn-mobile-cicd-installer \
    target/x86_64-apple-darwin/release/ravn-mobile-cicd-installer

# For local build
install-cross:
  cargo install cross --git https://github.com/cross-rs/cross

[macos]
release-linux: install-cross
  cross build --release --target x86_64-unknown-linux-musl
  mkdir -p build
  cp target/x86_64-unknown-linux-musl/release/ravn-mobile-cicd-installer build/installer-linux

[linux]
release-linux:
  cargo build --release --target x86_64-unknown-linux-musl
  mkdir -p build
  cp target/x86_64-unknown-linux-musl/release/ravn-mobile-cicd-installer build/installer-linux


