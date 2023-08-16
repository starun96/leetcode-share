#!/usr/bin/env bash

DIR="$( dirname -- "${BASH_SOURCE[0]}"; )"
DIR="$( realpath -e -- "$DIR"; )"

ASDF="$DIR/asdf"
git submodule update --init --recursive --depth 1

source "$ASDF/asdf.sh"
source "$ASDF/completions/asdf.bash"

# tools that are guaranteed to be instantiated
TOOLS=( shellcheck rust python )

sync_tools() {
    local reshim=0
    for lang in "$@"; do
        if ! asdf current "$lang" &>/dev/null; then
            asdf plugin add "$lang" &>/dev/null
            asdf install "$lang" latest &>/dev/null
            asdf local "$lang" latest &>/dev/null
            reshim=1
        fi
    done

    return "$reshim"
}

asdf reshim

cargo install cargo-quickinstall cargo-binstall
cargo binstall -y sccache
