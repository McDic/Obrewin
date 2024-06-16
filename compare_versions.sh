#!/bin/bash

CURRENT_TAG="$(sed "s/v//g" <<< $(git describe --tags --exact-match --abbrev=0))"

CARGO_TOML_VERSION="$(cat Cargo.toml | grep "^version")"
CARGO_TOML_VERSION=$(sed "s/version = //g" <<< "$CARGO_TOML_VERSION")
CARGO_TOML_VERSION=$(sed "s/\"//g" <<< "$CARGO_TOML_VERSION")

echo CURRENT_TAG = v$CURRENT_TAG, CARGO_TOML_VERSION = v$CARGO_TOML_VERSION

if [[ "$CURRENT_TAG" == "$CARGO_TOML_VERSION" ]]; then
    exit 0
else
    exit 1
fi
