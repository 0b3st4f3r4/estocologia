#!/bin/sh
# Mandatory verification suite of AGENTS.md §V in a single entry point.
#
# Both the governance contract and CI invoke this script directly.
#
# Usage:
#   scripts/verify.sh           # full §V verification
#   scripts/verify.sh --fast    # skip heavy lab runs for inner edit loop
set -eu

cd "$(dirname "$0")/.."

fast=0
if [ "${1:-}" = "--fast" ] || [ "${1:-}" = "--rapido" ]; then
  fast=1
fi

step() { printf '\n== %s\n' "$1"; }

step "format: cargo fmt --check"
cargo fmt --all --check

step "lints: clippy -D warnings"
cargo clippy --workspace --all-targets --locked -- -D warnings

step "compile instruments in release mode"
cargo build --release --workspace --locked

step "test all workspace crates"
cargo test --workspace --locked

step "deterministic gates"
./target/release/gates --horizon all --reproduction

step "negative proofs: gates must know how to reject"
if ./target/release/gates --horizon h3 --defect h3.voos23; then
  echo "FAIL: gates negative proof passed" >&2; exit 1
fi
./target/release/wave --check
if ./target/release/wave --defect mutation; then
  echo "FAIL: seal chain negative proof passed" >&2; exit 1
fi
if ./target/release/wave --defect inversion; then
  echo "FAIL: causal arrow negative proof passed" >&2; exit 1
fi
if ./target/release/labs --unknown-flag; then
  echo "FAIL: labs accepted unknown flag" >&2; exit 1
fi

if [ "$fast" -eq 0 ]; then
  step "all seventeen labs instruments required by §V"
  for front in all bank resolution s1 s2 queue genetics streams h3 second-opinion bridge listening schema jev \
               rehearsal-shelf rehearsal-listening rehearsal-bridge; do
    printf -- "-- labs --%s\n" "$front"
    ./target/release/labs "--$front" > /dev/null
  done
fi

step "the textbook: ramp, disk, and declared counts"
python3 scripts/book/build.py --check

step "every cited output block against its instrument"
python3 scripts/book/check_labs.py

step "parts, chapters, code fences, and order"
python3 scripts/book/check_book.py

step "reimplementation of the laboratories from the text alone"
python3 -I scripts/reimplementation/run.py > /dev/null

step "the generated record docs/RUST_RESULTS.md"
python3 scripts/book/generate_results.py --check

step "all 30 SHA-256 seals"
python3 scripts/check_seals.py

step "prose pass against baseline"
python3 scripts/style/check_pass.py

step "history, not versions: no tags in the repository"
tags=$(git tag --list)
if [ -n "$tags" ]; then
  echo "FAIL: the project keeps history only, and these tags version it (AGENTS.md §VI):" >&2
  printf '%s\n' "$tags" >&2
  exit 1
fi
echo "no tags: the git log and history/ are the only record"

step "no drift: generated and sealed artifacts are committed"
if [ "${CI:-}" = "true" ]; then
  drift=$(git status --porcelain -- book/stochology-textbook.md docs/RUST_RESULTS.md \
          docs/STATE_DECLARATION.sha256)
  if [ -n "$drift" ]; then
    echo "FAIL: generated artifact or seal out of sync with commit:" >&2
    printf '%s\n' "$drift" >&2
    exit 1
  fi
else
  echo "skipped outside CI: generators already enforce --check"
fi

printf '\nverification complete: 0 problems\n'
