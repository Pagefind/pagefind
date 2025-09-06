#!/usr/bin/env bash
set -eu

cd wrappers/python

VIRTUAL_ENV="$PWD/.venv"
echo "VIRTUAL_ENV=$VIRTUAL_ENV" >> "$GITHUB_ENV"

if ! [ -d "$VIRTUAL_ENV" ]; then
  echo "No virtualenv found at $VIRTUAL_ENV"
  exit 127
fi

# Ensure binaries from the virtualenv are available at the start of $PATH
# see https://docs.python.org/3/library/venv.html#creating-virtual-environments
# on unix systems, virtualenv puts executables in .venv/bin
echo "$VIRTUAL_ENV/bin" >> "$GITHUB_PATH"
  # on windows, virtualenv places executables in .venv/Scripts
echo "$VIRTUAL_ENV/Scripts" >> "$GITHUB_PATH"
# see https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#adding-a-system-path
