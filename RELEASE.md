# Release process

The release is staged so that crates.io publication can never happen before the tagged GitHub Release is healthy. `cargo release` is configured only to prepare a release commit: it does not publish, tag, or push.

## 1. Preflight

Start from current `main` with no local changes:

```bash
git switch main
git fetch origin main --tags
git pull --ff-only origin main
test -z "$(git status --porcelain)"
test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)"
```

Before preparing a version, confirm that its `v<version>` tag and crates.io versions do not already exist. Run the normal test suite, SemVer checks, docs/MSRV checks, `cargo audit`, and `cargo publish --dry-run` for both public crates. GitHub Dependabot must have no open security alerts.

## 2. Prepare a release PR

Create a branch named `release/<version>`, then preview and execute cargo-release:

```bash
git switch -c release/<version>
cargo release <version> --no-confirm
cargo release <version> --execute --no-confirm
```

The configured release step only:

- bumps the shared workspace version,
- turns the current `CHANGELOG.md` Unreleased section into a dated version section while creating a new empty Unreleased section, and
- creates one local `chore(release): <version>` commit.

It does **not** run `cargo publish`, create a tag, or push anything. Inspect the commit, re-run `cargo audit` and both publish dry-runs, then push the branch and merge it to `main` through a PR.

## 3. Tag and GitHub Release

After the release PR is merged, update local `main` and verify the release commit is exactly `origin/main`. Create and push an annotated tag:

```bash
git switch main
git pull --ff-only origin main
git tag -a v<version> -m "Release <version>"
git push origin v<version>
```

The tag triggers `.github/workflows/release.yml` / cargo-dist. Wait for all release jobs to succeed and verify that the GitHub Release points at exactly the tagged commit. Stop here if cargo-dist fails or the release/tag source does not match `main`.

## 4. crates.io publication

Only after the GitHub Release is healthy:

```bash
cargo publish -p wasi_virt_layer
cargo publish -p wasi_virt_layer-cli
```

Publish the core crate first because the CLI is downstream of it. Verify the intended version of both crates on crates.io after publication.
