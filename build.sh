#!/bin/sh
unset IFS
set -euf

TARGET="$(uname -m)-unknown-linux-musl"

rustup target add "$TARGET"

cargo build \
    --target "$TARGET" \
    --release

cp "target/$TARGET/release/tuggy" target/release
