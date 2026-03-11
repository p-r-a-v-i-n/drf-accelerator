# Releasing `drf-accelerator`

This repo is set up to publish to PyPI automatically from GitHub Actions when a version tag is pushed.

## One-time Setup (Recommended: Trusted Publishing)
1. Create the project on PyPI (`drf-accelerator`) if it doesn't exist.
2. In PyPI, enable "Trusted Publishing" for this project and add this GitHub repo as a publisher.
3. Ensure the GitHub Actions workflow has `id-token: write` permission (see `.github/workflows/release.yml`).

With trusted publishing, no PyPI API token secret is required.

## Release Steps
1. Update version in `pyproject.toml` (`[project].version`).
2. Commit the version bump.
3. Tag and push:
   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```
4. GitHub Actions builds wheels + sdist and publishes to PyPI automatically.

## Notes
- The workflow verifies that the pushed tag `vX.Y.Z` matches `pyproject.toml`'s version.
- Wheels are built for Linux/macOS/Windows across CPython 3.10–3.13.

