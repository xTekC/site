#!/bin/just

_default:
  clear && just --list --unsorted

# Web
w:
  @cargo b
  @just _fl && just _s

# Serve
_s:
    clear
    @printf "\nhttp://127.0.0.1:8080/#dev\n\n"
    @dx serve --hot-reload

# Format and Lint
_fl:
    clear
    @cargo fmt --all
    @cargo clippy --locked --all-targets

# Test
t:
    clear
    @cargo test

# Clean
c:
    clear
    @rm -rf dist/
    @rm -rf target/
    @rm -f Cargo.lock
