#!/bin/bash
set -euo pipefail

# Usage:
#   bash scripts/run.sh                # 最新の問題を実行
#   bash scripts/run.sh abc/445 a      # abc445-a の最新版を実行

if [ $# -ge 2 ]; then
    IFS='/' read -r TYPE NUMBER <<< "$1"
    PROBLEM=$2
    HUNDRED=$((NUMBER / 100))XX
    TEN=$((NUMBER / 10))X
    DIR="problem/${TYPE}/${HUNDRED}/${TEN}/${NUMBER}/${PROBLEM}"
    LATEST=$(ls -1d "${DIR}"/*/ 2>/dev/null | sort | tail -1)
    if [ -z "${LATEST}" ]; then
        echo "Error: No submissions found for ${TYPE}${NUMBER}-${PROBLEM}"
        exit 1
    fi
else
    LATEST=$(find problem -name "Cargo.toml" -printf '%T@ %h\n' 2>/dev/null | sort -n | tail -1 | cut -d' ' -f2)
    if [ -z "${LATEST}" ]; then
        echo "Error: No problems found"
        exit 1
    fi
fi

NAME=$(grep -oP 'name = "\K[^"]+' "${LATEST}/Cargo.toml" | head -1)
echo "Running: ${NAME}"
cargo run -p "${NAME}" < input.txt 2>&1 | tee output.txt
