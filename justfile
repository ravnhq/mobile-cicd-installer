#!/usr/bin/env just --justfile

lint:
  cargo clippy

release-macos-arm:
  cargo build --release --target aarch64-apple-darwin

release-macos-x86:
  cargo build --release --target x86_64-apple-darwin

release-macos: release-macos-arm release-macos-x86
  mkdir -p build
  lipo -create -output build/installer \
    target/aarch64-apple-darwin/release/ravn-mobile-cicd-installer \
    target/x86_64-apple-darwin/release/ravn-mobile-cicd-installer
