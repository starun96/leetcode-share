#!/usr/bin/env bash

DIR="$( dirname -- "${BASH_SOURCE[0]}"; )"
DIR="$( realpath -e -- "$DIR"; )"

git submodule update --init --recursive --depth 1


