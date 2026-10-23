#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo ">>> [1/6] Checking formatting (cargo fmt)..."
cargo fmt --all --check

echo ">>> [2/6] Checking strict lints (cargo clippy -D warnings)..."
cargo clippy --workspace --all-targets -- -D warnings

echo ">>> [3/6] Running unit and integration tests (cargo test)..."
cargo test --workspace

echo ">>> [4/6] Running methodological and empirical laboratories (labs)..."
cargo run --release --bin labs -- --all
cargo run --release --bin labs -- --layers
cargo run --release --bin labs -- --uncertainty
cargo run --release --bin labs -- --lenses
cargo run --release --bin labs -- --invariants
cargo run --release --bin labs -- --siga
cargo run --release --bin labs -- --governance

echo ">>> [5/6] Running epistemological gates (gates) and negative proofs..."
cargo run --release --bin gates

for defect in cromwell phi5 moulton performativity sniedovich clock vault concept_drift chouldechova; do
    if cargo run --release --bin gates -- --defect "$defect" >/dev/null 2>&1; then
        echo "ERROR: negative proof '$defect' should have exited with code 1, but exited with 0!" >&2
        exit 1
    else
        echo "  [OK] Negative proof '$defect' blocked with exit code 1"
    fi
done

echo ">>> [6/6] Verifying absence of git tags..."
if [ -n "$(git tag --list)" ]; then
    echo "ERROR: git tags found; this repository does not use git tags." >&2
    exit 1
fi

echo ">>> FULL VERIFICATION APPROVED (STOCHOLOGY PROTOCOL v1.0) <<<"
