set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SITE_DIR="${SCRIPT_DIR}/.."
# PROJECT_DIR="${SCRIPT_DIR}/../../.."
# cd "$PROJECT_DIR/demos/basic"

cd "$SITE_DIR"

http-server -p 8001 output

