alias b := build
alias r := run
alias s := setup
alias c := clean

build:
  ./tailwind -i styles.scss -o public/styles.css --minify
  cargo build --release

run:
  ./tailwind -i styles.scss -o public/styles.css
  cargo watch -x run

setup:
  bash ./scripts/tailwind-download.sh
  chmod +x ./tailwind

clean:
  cargo clean
  rm tailwind
  rm public/styles.css
