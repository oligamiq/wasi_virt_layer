#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "usage: $0 <version> [github|complete]" >&2
  exit 2
fi

VERSION="$1"
MODE="${2:-github}"
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "invalid version: $VERSION" >&2
  exit 2
fi
if [[ "$MODE" != "github" && "$MODE" != "complete" ]]; then
  echo "invalid mode: $MODE (expected github or complete)" >&2
  exit 2
fi

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
TAG="v${VERSION}"

require() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "missing required command: $1" >&2
    exit 1
  }
}
for cmd in git cargo gh python3 sort grep sha256sum; do
  require "$cmd"
done

echo "== repository and tag =="
git fetch origin main --tags
[[ "$(git branch --show-current)" == "main" ]] || {
  echo "release postflight must run on main" >&2
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
git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null || {
  echo "${TAG} is missing locally" >&2
  exit 1
}
[[ "$(git cat-file -t "refs/tags/${TAG}")" == "tag" ]] || {
  echo "${TAG} is not an annotated tag" >&2
  exit 1
}
TAG_SHA="$(git rev-list -n1 "$TAG")"
[[ "$TAG_SHA" == "$(git rev-parse HEAD)" ]] || {
  echo "${TAG} does not point at current main" >&2
  exit 1
}
REMOTE_TAG_SHA="$(git ls-remote origin "refs/tags/${TAG}^{}" | awk '{print $1}')"
[[ "$REMOTE_TAG_SHA" == "$TAG_SHA" ]] || {
  echo "origin ${TAG} does not peel to current main" >&2
  exit 1
}
echo "== workspace version =="
VERSIONS="$(cargo metadata --no-deps --format-version 1 | python3 -c '
import json,sys
pkgs=json.load(sys.stdin)["packages"]
for name in ("wasi_virt_layer", "wasi_virt_layer-cli"):
    print(next(p["version"] for p in pkgs if p["name"] == name))
')"
CORE_VERSION="$(printf '%s\n' "$VERSIONS" | sed -n '1p')"
CLI_VERSION="$(printf '%s\n' "$VERSIONS" | sed -n '2p')"
[[ "$CORE_VERSION" == "$VERSION" && "$CLI_VERSION" == "$VERSION" ]] || {
  echo "workspace versions do not match ${VERSION}: core=${CORE_VERSION} cli=${CLI_VERSION}" >&2
  exit 1
}

echo "== GitHub Actions release run =="
RUN_JSON="$(gh run list --workflow Release --branch "$TAG" --event push --limit 5 \
  --json headBranch,headSha,status,conclusion,databaseId,url \
  --jq '.[0]')"
[[ "$RUN_JSON" != "null" && -n "$RUN_JSON" ]] || {
  echo "no Release workflow run found for ${TAG}" >&2
  exit 1
}
read -r RUN_SHA RUN_STATUS RUN_CONCLUSION RUN_ID <<<"$(python3 -c '
import json,sys
r=json.load(sys.stdin)
print(r["headSha"], r["status"], r["conclusion"], r["databaseId"])
' <<<"$RUN_JSON")"
[[ "$RUN_SHA" == "$TAG_SHA" ]] || {
  echo "Release workflow SHA does not match tag" >&2
  exit 1
}
[[ "$RUN_STATUS" == "completed" && "$RUN_CONCLUSION" == "success" ]] || {
  echo "Release workflow ${RUN_ID} is not successful: ${RUN_STATUS}/${RUN_CONCLUSION}" >&2
  exit 1
}
echo "== GitHub Release =="
RELEASE_JSON="$(gh release view "$TAG" \
  --json tagName,name,isDraft,isPrerelease,targetCommitish,publishedAt,assets)"
read -r REL_TAG REL_DRAFT REL_PRE REL_TARGET <<<"$(python3 -c '
import json,sys
r=json.load(sys.stdin)
print(r["tagName"], str(r["isDraft"]).lower(), str(r["isPrerelease"]).lower(), r["targetCommitish"])
' <<<"$RELEASE_JSON")"
[[ "$REL_TAG" == "$TAG" ]] || {
  echo "GitHub Release tag mismatch: ${REL_TAG}" >&2
  exit 1
}
[[ "$REL_DRAFT" == "false" && "$REL_PRE" == "false" ]] || {
  echo "GitHub Release is draft or prerelease" >&2
  exit 1
}
[[ "$REL_TARGET" == "$TAG_SHA" ]] || {
  echo "GitHub Release target does not match tag SHA" >&2
  exit 1
}

EXPECTED_ASSETS="$(cat <<EOF
sha256.sum
source.tar.gz
source.tar.gz.sha256
wasi_virt_layer-cli-aarch64-apple-darwin-update
wasi_virt_layer-cli-aarch64-apple-darwin.tar.xz
wasi_virt_layer-cli-aarch64-apple-darwin.tar.xz.sha256
wasi_virt_layer-cli-aarch64-unknown-linux-gnu-update
wasi_virt_layer-cli-aarch64-unknown-linux-gnu.tar.xz
wasi_virt_layer-cli-aarch64-unknown-linux-gnu.tar.xz.sha256
wasi_virt_layer-cli-installer.ps1
wasi_virt_layer-cli-installer.sh
wasi_virt_layer-cli-x86_64-apple-darwin-update
wasi_virt_layer-cli-x86_64-apple-darwin.tar.xz
wasi_virt_layer-cli-x86_64-apple-darwin.tar.xz.sha256
wasi_virt_layer-cli-x86_64-pc-windows-msvc-update
wasi_virt_layer-cli-x86_64-pc-windows-msvc.zip
wasi_virt_layer-cli-x86_64-pc-windows-msvc.zip.sha256
wasi_virt_layer-cli-x86_64-unknown-linux-gnu-update
wasi_virt_layer-cli-x86_64-unknown-linux-gnu.tar.xz
wasi_virt_layer-cli-x86_64-unknown-linux-gnu.tar.xz.sha256
dist-manifest.json
EOF
)"
EXPECTED_ASSETS="$(printf '%s\n' "$EXPECTED_ASSETS" | LC_ALL=C sort)"
ACTUAL_ASSETS="$(python3 -c '
import json,sys
for a in sorted(json.load(sys.stdin)["assets"], key=lambda x: x["name"]):
    print(a["name"])
' <<<"$RELEASE_JSON")"
[[ "$ACTUAL_ASSETS" == "$EXPECTED_ASSETS" ]] || {
  echo "GitHub Release asset set differs from expected cargo-dist 0.31.0 output" >&2
  diff -u <(printf '%s\n' "$EXPECTED_ASSETS") <(printf '%s\n' "$ACTUAL_ASSETS") >&2 || true
  exit 1
}

echo "== release checksums and installers =="
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
gh release download "$TAG" \
  -p sha256.sum \
  -p source.tar.gz \
  -p 'wasi_virt_layer-cli-aarch64-apple-darwin.tar.xz' \
  -p 'wasi_virt_layer-cli-aarch64-unknown-linux-gnu.tar.xz' \
  -p 'wasi_virt_layer-cli-x86_64-apple-darwin.tar.xz' \
  -p 'wasi_virt_layer-cli-x86_64-pc-windows-msvc.zip' \
  -p 'wasi_virt_layer-cli-x86_64-unknown-linux-gnu.tar.xz' \
  -p 'wasi_virt_layer-cli-installer.sh' \
  -p 'wasi_virt_layer-cli-installer.ps1' \
  -D "$TMP"
(
  cd "$TMP"
  sha256sum -c sha256.sum
)
grep -Fq "$TAG" "$TMP/wasi_virt_layer-cli-installer.sh" || {
  echo "shell installer does not reference ${TAG}" >&2
  exit 1
}
grep -Fq "$TAG" "$TMP/wasi_virt_layer-cli-installer.ps1" || {
  echo "PowerShell installer does not reference ${TAG}" >&2
  exit 1
}

if [[ "$MODE" == "complete" ]]; then
  echo "== crates.io publication =="
  for crate in wasi_virt_layer wasi_virt_layer-cli; do
    INFO="$(cargo info --registry crates-io "${crate}@${VERSION}")"
    grep -Fxq "version: ${VERSION}" <<<"$INFO" || {
      echo "${crate} ${VERSION} is not visible on crates.io" >&2
      exit 1
    }
  done
fi
echo "== final state =="
[[ -z "$(git status --porcelain)" ]] || {
  echo "postflight changed the working tree" >&2
  git status --short >&2
  exit 1
}
[[ "$(git rev-parse HEAD)" == "$(git rev-parse origin/main)" ]] || {
  echo "main moved during postflight; rerun against the new HEAD" >&2
  exit 1
}

echo "release postflight (${MODE}) passed for ${VERSION}"