#!/usr/bin/env bash

# get script dir
DIR="$( dirname -- "${BASH_SOURCE[0]}"; )"
DIR="$( realpath -e -- "$DIR"; )"

# install asdf to start
ASDF="$DIR/asdf"
git submodule update --init --recursive --depth 1 "$ASDF"
source "$ASDF/asdf.sh"
source "$ASDF/completions/asdf.bash"

# asdf installs the tooling
TOOLS=( shellcheck rust python direnv )
reshim=0
for lang in "${TOOLS[@]}"; do
    if ! asdf current "$lang" &>/dev/null; then
        asdf plugin add "$lang" &>/dev/null
        asdf install "$lang" latest &>/dev/null
        asdf local "$lang" latest &>/dev/null
        reshim=1
    fi
done

# reshim if any deltas need to be picked up
asdf reshim

eval "$(direnv stdlib)"
eval "$(direnv hook bash)"

# basic cargo tools to speed up compiles and the like
cargo install cargo-quickinstall cargo-binstall
cargo binstall -y sccache cargo-prefetch
cargo-prefetch --top-downloads=384

asdf reshim
