#!/usr/bin/env bash

# Initializing build environment


rustup install nightly-2021-03-15-x86_64-unknown-linux-gnu
rustup default nightly-2021-03-15-x86_64-unknown-linux-gnu

rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-15-x86_64-unknown-linux-gnu
