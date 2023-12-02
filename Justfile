alias p := package
alias w := watch
alias r := run
alias s := setup
alias c := clean

package:
  just --justfile {{justfile()}} setup
  ./tailwind -i styles.scss -o public/styles.css --minify
  cargo build --release
  ./scripts/package.sh

run:
  just --justfile {{justfile()}} setup
  ./tailwind -i styles.scss -o public/styles.css
  cargo run

watch:
  @if ! which cargo-watch > /dev/null; then \
    echo "cargo-watch is not installed"; \
    cargo install cargo-watch; \
  fi

  cargo watch -s "just run"

setup:
  ./scripts/tailwind-download.sh
  chmod +x ./tailwind

clean:
  cargo clean
  rm tailwind
  rm public/styles.css
