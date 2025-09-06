#!/usr/bin/env bash
set -eux

# shellcheck source=../activate_venv.sh
. ./scripts/activate_venv.sh

python -m ruff format
