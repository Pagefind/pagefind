#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -lt 1 ]]; then
  echo "Usage: $0 <binary-path> [<binary-path> ...]" >&2
  exit 2
fi

for cmd in objdump qemu-x86_64; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Missing required verification tool: $cmd" >&2
    exit 2
  fi
done

tmp_site="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_site"
}
trap cleanup EXIT

cat > "$tmp_site/index.html" <<'HTML'
<!doctype html>
<html>
  <body>
    <main>
      <h1>Baseline CPU Smoke Test</h1>
      <p>Verifying pagefind runs on pre-BMI2 x86-64 CPUs.</p>
    </main>
  </body>
</html>
HTML

bmi2_regex='^[[:space:]]*[0-9a-f]+:.*[[:space:]](shlx|shrx|sarx|andn|mulx|pdep|pext|bzhi|rorx)[qlwb]?([[:space:]]|$)'

for bin in "$@"; do
  if [[ ! -x "$bin" ]]; then
    echo "Binary is missing or not executable: $bin" >&2
    exit 1
  fi

  echo "Checking $bin for BMI2 instructions..."
  matches="$(objdump -d "$bin" | grep -E -m 20 "$bmi2_regex" || true)"
  if [[ -n "$matches" ]]; then
    echo "Error: found BMI2 instructions in $bin (breaks pre-BMI2 x86-64 CPUs):" >&2
    echo "$matches" >&2
    exit 1
  fi

  echo "Running $bin under qemu-x86_64 -cpu Nehalem..."
  qemu-x86_64 -cpu Nehalem "$bin" --version
  rm -rf "$tmp_site/pagefind"
  qemu-x86_64 -cpu Nehalem "$bin" --site "$tmp_site" >/dev/null
done
