#!/bin/bash

VERSION=$(cat Cargo.toml | grep '^version = "' | cut -d' ' -f 3 | tr -d '"')
mkdir artifacts
cp ./target/release/btwn ./artifacts/btwn-linux-x64
cp ./target/x86_64-pc-windows-gnu/release/btwn.exe ./artifacts/btwn-win-x64.exe
gh release create "v$VERSION" ./artifacts/btwn-linux-x64 ./artifacts/btwn-win-x64.exe