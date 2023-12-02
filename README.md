# Rust Bytes Blog
A public blog page focused around programming and especially Rust.

## Dependencies:
* [Just](https://just.systems/) - A command runner (comparable to Makefile)
* [Rust](https://rustup.rs) - The Rust programming language

## Development usage
You can get all recipes by doing `just --list`
Any recipe can be ran like this `just <recipe>`, e.g. `just watch`

Here are the useful ones:
- `package` - Creates a standalone binary and copies resources
- `run` - Simply runs the blog in development page
- `watch` - Rebuild the blog in development mode on file changes
- `clean` - Remove build files and downloaded files

## Production usage
- Run `just package`
- Go to the `target/standalone` directory
- Run the `blog` executable
