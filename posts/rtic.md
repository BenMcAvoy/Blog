<div class="title">

# RTIC
RTIC, the hardware accelerated Rust RTOS.

</div>

## Requirements:
- [Rust](https://rust-lang.org)
<br>

## Installing dependencies:
```bash
cargo install flip-link # Zero-cost stack-overflow protection
cargo install probe-rs -F cli # Used for interfacing with the board
cargo install cargo-generate # Used to make a new project from a template
```
<br>

## Creating a new project:
```bash
cargo generate --git https://github.com/rtic-rs/app-template
```

This will ask you for a project name and then will create a new directory with the project in it.