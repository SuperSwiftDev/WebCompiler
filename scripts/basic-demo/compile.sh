set -e

# ./scripts/ssio.sh compile --root sample --template sample/base.html --input "sample/pages/**/*.html" --output output

cargo run --bin web-compiler run --manifest demos/basic/web-compiler.toml
