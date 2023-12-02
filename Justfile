alias p := package
alias r := run
alias s := setup
alias c := clean

package:
  ./tailwind -i styles.scss -o public/styles.css --minify
  cargo build --release
  ./scripts/package.sh

run:
  ./tailwind -i styles.scss -o public/styles.css --watch &
  cargo watch -x run

setup:
  ./scripts/tailwind-download.sh
  chmod +x ./tailwind

clean:
  cargo clean
  rm tailwind
  rm public/styles.css
