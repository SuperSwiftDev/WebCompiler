set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/../../.."
cd "$PROJECT_DIR"

# SITE_DIR="${SCRIPT_DIR}/.."

# cd "$PROJECT_DIR/demos/basic"

# CURRENT_DIR="$(pwd)"
# SITE_DIR="/Users/colbyn/Developer/USASuperior"
# DEV_TOOL_DIR="/Users/colbyn/Developer/SuperSwiftDev-Projects/WebCompiler"
# CARGO_MANIFEST_PATH="$DEV_TOOL_DIR/Cargo.toml"

# cargo build --release --manifest-path "$DEV_TOOL_DIR/Cargo.toml"

# cargo run --manifest-path $CARGO_MANIFEST_PATH --bin ssio -- build --manifest ./site.toml

cargo run --bin web-compiler -- build demos/basic/web-compiler.toml
