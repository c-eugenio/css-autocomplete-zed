# Releasing a New Version

## 1. Update version numbers

In **both** files, change the version to the new one (e.g. `0.2.0`):

- `Cargo.toml` (root) → `version = "0.2.0"`
- `extension.toml` → `version = "0.2.0"`

## 2. Commit and tag

```sh
git add Cargo.toml extension.toml
git commit -m "Bump version to 0.2.0"
git tag v0.2.0
git push origin main --follow-tags
```

This triggers GitHub Actions, which builds the binaries for all platforms and creates a GitHub release automatically.

## 3. Update the Zed extensions registry

In your fork of `zed-industries/extensions`, update the submodule to the new commit and bump the version:

```sh
cd extensions/css-class-autocomplete
git pull origin main
cd ../..
git add extensions/css-class-autocomplete
```

Then edit `extensions.toml` and change the version:

```toml
[css-class-autocomplete]
submodule = "extensions/css-class-autocomplete"
version = "0.2.0"
```

Commit and push:

```sh
git add extensions.toml
git commit -m "Update css-class-autocomplete to 0.2.0"
git push origin main
```

Then open a PR to `zed-industries/extensions` from your fork.

---

## If you need to redo a tag

Only do this **before** the PR to `zed-industries/extensions` is submitted or while it is still open. Never move a tag after the PR is merged.

```sh
git tag -d v0.2.0
git push origin --delete v0.2.0

# Make your changes, then:
git tag v0.2.0
git push origin main --follow-tags
```

If the PR is already open, also update the submodule in your fork to the new commit (step 3 above).
