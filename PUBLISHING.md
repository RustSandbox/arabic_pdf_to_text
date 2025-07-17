# Publishing Guide

This guide explains how to publish arabic_pdf_to_text to crates.io and create GitHub releases.

## Prerequisites

1. Create a crates.io account at https://crates.io
2. Generate an API token: https://crates.io/settings/tokens
3. Add the token to GitHub secrets as `CARGO_REGISTRY_TOKEN`

## Publishing to crates.io

### Manual Publishing

```bash
# Login to crates.io (first time only)
cargo login

# Dry run to check everything is OK
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

### Automated Publishing (via GitHub Actions)

1. Ensure version in `Cargo.toml` is updated
2. Create and push a version tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. GitHub Actions will automatically:
   - Publish to crates.io
   - Build binaries for multiple platforms
   - Create a GitHub release with binaries

## Version Bumping

Before publishing a new version:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md (if you have one)
3. Commit changes: `git commit -m "Bump version to X.Y.Z"`
4. Tag the release: `git tag vX.Y.Z`
5. Push: `git push && git push --tags`

## Crate Name Availability

The crate name `arabic_pdf_to_text` must be available on crates.io. 
Check availability at: https://crates.io/search?q=arabic_pdf_to_text

## First Time Setup

1. Add your crates.io API token to GitHub repository secrets:
   - Go to Settings → Secrets and variables → Actions
   - Add new secret named `CARGO_REGISTRY_TOKEN`
   - Paste your crates.io API token

2. Ensure all metadata in `Cargo.toml` is complete:
   - description (max 160 chars)
   - license
   - repository
   - keywords (max 5)
   - categories (max 5)

## Troubleshooting

- If publish fails due to missing metadata, check `Cargo.toml`
- If authentication fails, regenerate your crates.io token
- For rate limits, wait and retry later