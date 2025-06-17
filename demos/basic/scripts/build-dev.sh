set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/../../.."
cd "$PROJECT_DIR"

cargo run --bin web-compiler -- build demos/basic/web-compiler.toml
