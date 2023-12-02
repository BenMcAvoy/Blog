#!/usr/bin/env bash

# Make sure the executable exists
if [ ! -f "./target/release/blog" ]; then
  echo "Executable doesn't exist, compiling now."
  cargo build --release || echo "Failed to compile" || exit 1
fi

echo "Copying executable"
mkdir ./target/standalone/ 2> /dev/null
cp ./target/release/blog ./target/standalone/ -f

echo "Copying resources"
cp ./templates ./target/standalone/ -rf
cp ./public ./target/standalone/ -rf

echo "Package available at ./target/standalone/"
