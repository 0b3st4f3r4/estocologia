#!/bin/sh
# Reproduz as quatro reimplementações (~4 min). Grava saida/*.json.
set -e
cd "$(dirname "$0")"
for r in r1_lalonde r2_star r3_herg r4_ri; do python3 -I $r.py > saida/$r.log; done
sha256sum saida/*.json
