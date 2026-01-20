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

# CI: format check, lint, build, and test
ci:
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo clippy --all-targets -- -D warnings
  cargo build
  cargo test --all-targets

# Clean build artifacts
clean:
  cargo clean

# Run E2E tests with mini.test
test-e2e: build
  nvim --headless -u test/e2e_init.lua

# Open sample file in Neovim for manual testing (logs to logs/hjkls.log)
dev-nvim: build
  mkdir -p logs
  nvim -u test/minimal_init.lua test/fixtures/sample.vim

# Open sample file in Vim for manual testing (logs to logs/hjkls.log)
# Note: Using -S instead of -u to preserve Nix's vimrc (which sets packpath for vim-lsp)
dev-vim: build
  mkdir -p logs
  vim -S test/minimal_init.vim test/fixtures/sample.vim

# Show hjkls debug log
log:
  @cat logs/hjkls.log 2>/dev/null || echo "(no log)"

# Clear debug logs
log-clear:
  rm -rf logs/
