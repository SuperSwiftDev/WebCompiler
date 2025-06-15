set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SITE_DIR="${SCRIPT_DIR}/.."

# echo "$SITE_DIR"
# PROJECT_DIR="${SCRIPT_DIR}/../.."
# cd "$PROJECT_DIR"

watchexec --ignore-file .watchexec-ignore -w . -e html -e css -- "$SCRIPT_DIR/build.sh"
