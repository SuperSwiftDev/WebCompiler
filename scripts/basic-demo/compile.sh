set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/../.."
cd $PROJECT_DIR

# ./scripts/ssio.sh compile --root sample --template sample/base.html --input "sample/pages/**/*.html" --output output

cargo run --bin web-compiler run --manifest demos/basic/web-compiler.toml
