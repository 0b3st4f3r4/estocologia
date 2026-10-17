#!/usr/bin/env bash
# H2g domínio a domínio, em paralelo (p05_um.py mantém a semente da varredura completa).
# (b) literatura só roda os 4 domínios com fonte conferida; nos outros 14 a entrada de literatura.yaml tem os
# mesmos quartis da fraca.yaml, então o resultado de (b) é o de (c) e é copiado (comparar.py confere os quartis).
# Uso: bash rodar_h2g.sh <pasta_de_saida> [paralelos]
set -u
AQUI="$(cd "$(dirname "$0")" && pwd)"
OUT="$1"; P="${2:-6}"
DADOS="${P05_DADOS:-/mnt/project-files/estocologia/dados-reais}/h2b"
COM_FONTE="direito_teto_beneficio educacao_star_turmas farmacologia_qt transporte_cinto"
TODOS=$(cd "$AQUI/h2g/dominios" && ls -d */ | tr -d / | grep -v '^_')
mkdir -p "$OUT/h2g"
{
  for d in $TODOS; do echo "fraca $d"; echo "original $d"; done
  for d in $COM_FONTE; do echo "literatura $d"; done
} | xargs -P "$P" -L 1 bash -c '
  v=$0; d=$1; tab=""; [ "$v" != original ] && tab="'"$AQUI"'/prioris/h2g/$v.yaml"
  cd "'"$AQUI"'/h2g" && P05_PRIORI="$tab" ESTOCOLOGIA_SAIDA="'"$OUT"'/h2g/$v" ESTOCOLOGIA_DADOS="'"$DADOS"'" \
    python3 p05_um.py "$d" >> "'"$OUT"'/h2g-$v.log" 2>&1; echo "$v $d saiu $?" >> "'"$OUT"'/fim_h2g.txt"'
for d in $TODOS; do
  case " $COM_FONTE " in *" $d "*) ;; *) mkdir -p "$OUT/h2g/literatura/$d"; cp "$OUT/h2g/fraca/$d/resultados.json" "$OUT/h2g/literatura/$d/";; esac
done
