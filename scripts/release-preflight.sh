#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <version>" >&2
  exit 2
fi

VERSION="$1"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "invalid version: $VERSION" >&2
  exit 2
fi

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

require() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "missing required command: $1" >&2
    exit 1
  }
}

for cmd in git cargo rustup gh python3; do
  require "$cmd"
done
cargo audit --version >/dev/null
cargo semver-checks --version >/dev/null
cargo nextest --version >/dev/null
echo "== repository state =="
git fetch origin main --tags
[[ "$(git branch --show-current)" == "main" ]] || {
  echo "release preflight must run on main" >&2
  exit 1
}
[[ -z "$(git status --porcelain)" ]] || {
  echo "working tree is not clean" >&2
  exit 1
}
[[ "$(git rev-parse HEAD)" == "$(git rev-parse origin/main)" ]] || {
  echo "HEAD does not match origin/main" >&2
  exit 1
}

LOCKFILES="$(git ls-files '*Cargo.lock')"
[[ "$LOCKFILES" == "Cargo.lock" ]] || {
  echo "only the root Cargo.lock may be tracked" >&2
  printf '%s\n' "$LOCKFILES" >&2
  exit 1
}

CURRENT_VERSION="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; d=json.load(sys.stdin); print(next(p["version"] for p in d["packages"] if p["name"]=="wasi_virt_layer"))')"
CLI_VERSION="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; d=json.load(sys.stdin); print(next(p["version"] for p in d["packages"] if p["name"]=="wasi_virt_layer-cli"))')"
[[ "$CURRENT_VERSION" == "$CLI_VERSION" ]] || {
  echo "public crate versions differ: core=${CURRENT_VERSION} cli=${CLI_VERSION}" >&2
  exit 1
}
BASELINE_TAG="v${CURRENT_VERSION}"
EXPECTED_VERSION="$(python3 - "$CURRENT_VERSION" <<'PYV'
import sys
major, minor, patch = map(int, sys.argv[1].split('.'))
print(f"{major}.{minor}.{patch + 1}")
PYV
)"
[[ "$VERSION" == "$EXPECTED_VERSION" ]] || {
  echo "target must be the next patch version: ${EXPECTED_VERSION}" >&2
  exit 1
}
echo "current=$CURRENT_VERSION target=$VERSION baseline=$BASELINE_TAG"
echo "== target availability =="
if git rev-parse -q --verify "refs/tags/v${VERSION}" >/dev/null; then
  echo "tag v${VERSION} already exists locally" >&2
  exit 1
fi
if git ls-remote --exit-code --tags origin "refs/tags/v${VERSION}" >/dev/null 2>&1; then
  echo "tag v${VERSION} already exists on origin" >&2
  exit 1
fi

for crate in wasi_virt_layer wasi_virt_layer-cli; do
  info_file="$(mktemp)"
  if cargo info "${crate}@${VERSION}" >"$info_file" 2>&1; then
    rm -f "$info_file"
    echo "${crate} ${VERSION} already exists on crates.io" >&2
    exit 1
  fi
  if ! grep -Fq "could not find \`${crate}@${VERSION}\` in registry" "$info_file"; then
    cat "$info_file" >&2
    rm -f "$info_file"
    echo "could not verify crates.io availability for ${crate}" >&2
    exit 1
  fi
  rm -f "$info_file"
done

echo "== GitHub security alerts =="
REPO="$(gh repo view --json nameWithOwner --jq .nameWithOwner)"
OPEN_ALERTS="$(gh api --paginate -X GET "repos/${REPO}/dependabot/alerts?state=open&per_page=100" --jq '.[].number' | wc -l)"
[[ "$OPEN_ALERTS" -eq 0 ]] || {
  echo "Dependabot has ${OPEN_ALERTS} open alerts" >&2
  exit 1
}

echo "== RustSec audit =="
cargo audit -D unsound
echo "== regression suite =="
cargo nextest run -r --fail-fast --retries 1

echo "== SemVer compatibility =="
git rev-parse -q --verify "refs/tags/${BASELINE_TAG}" >/dev/null || {
  echo "baseline tag ${BASELINE_TAG} does not exist" >&2
  exit 1
}
cargo semver-checks -p wasi_virt_layer \
  --baseline-rev "$BASELINE_TAG" --release-type patch --color never
cargo semver-checks -p wasi_virt_layer-cli \
  --baseline-rev "$BASELINE_TAG" --release-type patch \
  --default-features --color never

echo "== MSRV =="
cargo +1.89.0 check -r -p wasi_virt_layer --all-features --locked
cargo +1.93.0 check -r -p wasi_virt_layer-cli --all-features --locked

echo "== rustdoc =="
RUSTDOCFLAGS='-D rustdoc::bare_urls' cargo doc -p wasi_virt_layer --all-features --no-deps --locked
RUSTDOCFLAGS='-D rustdoc::bare_urls' cargo doc -p wasi_virt_layer-cli --all-features --no-deps --locked
echo "== package verification =="
cargo publish --dry-run -p wasi_virt_layer --locked
cargo publish --dry-run -p wasi_virt_layer-cli --locked

echo "== final state =="
[[ -z "$(git status --porcelain)" ]] || {
  echo "preflight changed the working tree" >&2
  git status --short >&2
  exit 1
}
[[ "$(git rev-parse HEAD)" == "$(git rev-parse origin/main)" ]] || {
  echo "main moved during preflight; rerun against the new HEAD" >&2
  exit 1
}

echo "release preflight passed for ${VERSION}"
