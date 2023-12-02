alias b := build
alias s := setup
alias c := clean

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
