#!/usr/bin/env bash
# set up the .venv
if [ -z "${VIRTUAL_ENV:-}" ]; then
  export VIRTUAL_ENV="$PWD/.venv"
fi

if [ ! -d "$VIRTUAL_ENV" ]; then
    echo "missing .venv @ $VIRTUAL_ENV" >&2
    exit 127
fi
export PATH="$VIRTUAL_ENV/bin:$VIRTUAL_ENV/Scripts:$PATH"
