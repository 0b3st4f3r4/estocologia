#!/usr/bin/env bash
# Roda as três variantes (original, literatura, fraca) nos três horizontes, a partir das cópias desta pasta.
# Uso: bash rodar.sh <pasta_de_saida>   (padrão: ./saida_rodadas, fora do Git)
set -u
AQUI="$(cd "$(dirname "$0")" && pwd)"
OUT="${1:-$AQUI/saida_rodadas}"
DADOS_REDE="${P05_DADOS:-/mnt/project-files/estocologia/dados-reais}"
mkdir -p "$OUT"
for v in original literatura fraca; do
  for h in h2 h2g h1-real; do
    tab=""; [ "$v" != original ] && tab="$AQUI/prioris/$h/$v.yaml"
    case $h in h2) dados="$DADOS_REDE/h2";; h2g) dados="$DADOS_REDE/h2b";; *) dados="";; esac
    ( cd "$AQUI/$h" && env P05_PRIORI="$tab" ESTOCOLOGIA_SAIDA="$OUT/$h/$v" \
        ${dados:+ESTOCOLOGIA_DADOS="$dados"} python3 rodar.py > "$OUT/$h-$v.log" 2>&1; echo "$h $v saiu $?" >> "$OUT/fim.txt" ) &
  done
done
wait
