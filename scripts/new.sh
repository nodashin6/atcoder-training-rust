#!/bin/bash
set -euo pipefail

if [ $# -lt 2 ]; then
    echo "Usage: bash scripts/new.sh <contest> <problem>"
    echo "Example: bash scripts/new.sh abc/445 c"
    exit 1
fi

# Parse contest: abc/445 → TYPE=abc, NUMBER=445
IFS='/' read -r TYPE NUMBER <<< "$1"
PROBLEM=$2
TIMESTAMP=$(date +%Y%m%d_%H%M)

# 3-tier directory: 445 → 4XX/44X/445
HUNDRED=$((NUMBER / 100))XX
TEN=$((NUMBER / 10))X
DIR="problem/${TYPE}/${HUNDRED}/${TEN}/${NUMBER}/${PROBLEM}/${TIMESTAMP}"
NAME="${TYPE}${NUMBER}-${PROBLEM}-${TIMESTAMP}"

mkdir -p "${DIR}"

# Cargo.toml
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
sed -e "s/{{NAME}}/${NAME}/g" "${SCRIPT_DIR}/Cargo.toml.template" > "${DIR}/Cargo.toml"

# main.rs
cp "${SCRIPT_DIR}/main.rs" "${DIR}/main.rs"

echo "Created: ${DIR}"
echo "Run:     cargo run -p ${NAME}"
