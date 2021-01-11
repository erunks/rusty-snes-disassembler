#!/bin/bash

DIR="$(dirname "$0")"

if cargo "$@"; then
    [ -d "$DIR/target/debug" ] && cp -r "$DIR/roms" "$DIR/target/debug/roms"
    [ -d "$DIR/target/debug" ] && cp -r "$DIR/opcodes" "$DIR/target/debug/opcodes"
    [ -d "$DIR/target/release" ] && cp -r "$DIR/roms" "$DIR/target/release/roms"
    [ -d "$DIR/target/release" ] && cp -r "$DIR/opcodes" "$DIR/target/release/opcodes"
fi