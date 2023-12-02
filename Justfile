run:
  cargo watch -x run

setup:
  bash ./scripts/tailwind-download.sh
  chmod +x ./tailwind

clean:
  cargo clean
  rm tailwind
