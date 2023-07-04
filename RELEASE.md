## Release Process

### Instructions

Requires `Bash`, [sfreleaser](https://github.com/streamingfast/sfreleaser) and [sd](https://github.com/chmln/sd):

```bash
# *Important* Do not forget to replace `0.9.2` below by your real version!
export version="0.9.2"

sd '^version = ".*?"$' "version = \"${version}\"" Cargo.toml
sd 'version = ".*?",' "version = \"${version}\"," Cargo.toml
sd '## Unreleased' "## ${version}" CHANGELOG.md

# Important so that Cargo.lock is updated and you "test
cargo test --target aarch64-apple-darwin # Change 'aarch64-apple-darwin' to fit your own platform!

git add -A . && git commit -m "Preparing release of ${version}"

sfreleaser release "${version}"
```