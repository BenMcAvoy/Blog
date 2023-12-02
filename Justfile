alias p := package
alias w := watch
alias r := run
alias s := setup
alias c := clean

# Creates a standalone binary and copies resources
package:
  just --justfile {{justfile()}} setup
  ./tailwind -i styles.scss -o public/styles.css --minify
  cargo build --release
  ./scripts/package.sh

# Simply runs the blog in development page
run:
  just --justfile {{justfile()}} setup
  ./tailwind -i styles.scss -o public/styles.css
  cargo run

# Rebuild the blog in development mode on file changes
watch:
  @if ! which cargo-watch > /dev/null; then \
    echo "cargo-watch is not installed"; \
    cargo install cargo-watch; \
  fi

  cargo watch -s "just run"

# Download tools/scripts and set them up.
setup:
  ./scripts/tailwind-download.sh
  chmod +x ./tailwind

# Remove build files and downloaded files
clean:
  cargo clean
  rm tailwind
  rm public/styles.css
