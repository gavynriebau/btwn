#!/bin/bash

VERSION=$(cat Cargo.toml | grep '^version = "' | cut -d' ' -f 3 | tr -d '"')
gh release create "v$VERSION" ./btwn-linux-x64 ./btwn-win-x64.exe