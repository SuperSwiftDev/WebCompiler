set -e

# rm -rf demos/basic/output && cargo run --bin web-compiler

cargo run --bin web-compiler -- build demos/basic/web-compiler.toml
