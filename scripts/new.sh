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
cat > "${DIR}/Cargo.toml" <<EOF
[package]
name = "${NAME}"
edition = "2021"

[[bin]]
name = "${NAME}"
path = "main.rs"

[dependencies]
proconio.workspace = true
num-bigint.workspace = true
num-traits.workspace = true
ac-lib.workspace = true
EOF

# main.rs
cat > "${DIR}/main.rs" <<EOF
use proconio::input;

fn main() {
    input! {
    }
}
EOF

echo "Created: ${DIR}"
echo "Run:     cargo run -p ${NAME}"
