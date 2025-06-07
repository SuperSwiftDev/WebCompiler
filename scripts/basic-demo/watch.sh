set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/../.."
cd "$PROJECT_DIR"

# watchexec --ignore output/ -w . -e html -- scripts/compile.sh
watchexec --ignore-file .watchexec-ignore -w sample -e html -e css -- scripts/basic-demo/compile.sh
