#!/usr/bin/env bash
set -eu
cd wrappers/python
if [ ! -d .venv ]; then
  python3 -m uv venv
fi
python3 -m uv sync
# echo "VIRTUAL_ENV=$VIRTUAL_ENV" >> "$GITHUB_ENV"
# echo "PATH=$VIRTUAL_ENV/bin:$PATH" >> "$GITHUB_ENV"
