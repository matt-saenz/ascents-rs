#!/usr/bin/env bash
set -e

cargo fmt --check
cargo clippy
