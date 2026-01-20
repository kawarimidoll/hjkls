[private]
default:
  @just --list

# Build debug binary
build:
  cargo build

# Build release binary
release:
  cargo build --release

# Run clippy and check
check:
  cargo check --all-targets
  cargo clippy --all-targets -- -D warnings

# Format code (Rust, Markdown, YAML, Nix)
fmt:
  cargo fmt --all
  dprint fmt
  nixfmt *.nix

# Run tests
test:
  cargo test --all-targets

# Verify (fmt + check + test)
verify: fmt check test

# Clean build artifacts
clean:
  cargo clean

# Open sample file in Neovim for manual testing (logs to logs/hjkls.log)
dev: build
  mkdir -p logs
  nvim -u test/minimal_init.lua test/fixtures/sample.vim

# Show hjkls debug log
log:
  @cat logs/hjkls.log 2>/dev/null || echo "(no log)"

# Clear debug logs
log-clear:
  rm -rf logs/
