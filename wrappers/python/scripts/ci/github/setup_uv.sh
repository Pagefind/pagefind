#!/usr/bin/env bash
set -eu
python3 -m pip install uv

# not using pipx since this is a CI environment that will be reset --
# there's not much risk of uv's zero dependencies conflicting with ours.
