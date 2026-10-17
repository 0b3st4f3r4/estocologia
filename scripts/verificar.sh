#!/bin/sh
# A verificacao obrigatoria de AGENTS.md §V, em um so lugar.
#
# O contrato e o CI leem daqui: enquanto a lista morava em dois arquivos, os dois divergiam
# (29 comandos no contrato, 11 no CI em 2026-10-17). Uma lista, dois consumidores.
#
# Uso:
#   scripts/verificar.sh            # tudo (§V inteiro)
#   scripts/verificar.sh --rapido   # sem os laboratorios pesados, para o laco de edicao
set -eu

cd "$(dirname "$0")/.."

rapido=0
[ "${1:-}" = "--rapido" ] && rapido=1

passo() { printf '\n== %s\n' "$1"; }

passo "formato: cargo fmt --check"
cargo fmt --all --check

passo "lints: clippy -D warnings"
cargo clippy --workspace --all-targets --locked -- -D warnings

passo "compilar instrumentos em release"
cargo build --release --workspace --locked

passo "testes dos dezesseis crates"
cargo test --workspace --locked

passo "portoes deterministicos"
./target/release/portoes --horizonte todos --reproducao

passo "provas negativas: um portao precisa saber reprovar"
if ./target/release/portoes --horizonte h3 --defeito h3.voos23; then
  echo "FALHA: a prova negativa dos portoes passou" >&2; exit 1
fi
./target/release/onda --check
if ./target/release/onda --defeito mutacao; then
  echo "FALHA: a prova negativa da cadeia de selos passou" >&2; exit 1
fi
if ./target/release/onda --defeito inversao; then
  echo "FALHA: a prova negativa da seta causal passou" >&2; exit 1
fi
if ./target/release/labs --flag-desconhecida; then
  echo "FALHA: labs aceitou flag desconhecida" >&2; exit 1
fi

if [ "$rapido" -eq 0 ]; then
  passo "os dezesseis instrumentos de labs que §V cobra"
  for frente in todos banco resolucao s1 s2 fila genetica h3 segunda ponte escuta schema jev \
                ensaio-shelf ensaio-escuta ensaio-ponte; do
    printf -- "-- labs --%s\n" "$frente"
    ./target/release/labs "--$frente" > /dev/null
  done
fi

passo "o livro: a rampa e o disco"
python3 book/gerar.py --check

passo "cada bloco citado contra o instrumento"
python3 book/conferir_labs.py

passo "partes, capitulos, cercas e ordem"
python3 book/conferir_livro.py

passo "o registro RESULTADOS_RUST.md"
python3 book/gerar_resultados.py --check

passo "os 28 lacres SHA-256"
python3 conferir_lacres.py

passo "a passada de prosa contra HEAD:book/"
python3 estilo/conferir-passada.py

passo "sem deriva: o gerado e o selado estao commitados"
# So faz sentido onde o commit existe: no CI a arvore e o commit, e o passo pega o artefato
# que foi gerado e esquecido. Na maquina do revisor o --check de cada gerador ja cobra o mesmo.
if [ "${CI:-}" = "true" ]; then
  deriva=$(git status --porcelain -- estocologia-livro-didatico.md RESULTADOS_RUST.md \
          DECLARACAO_DO_ESTADO.sha256)
  if [ -n "$deriva" ]; then
    echo "FALHA: artefato gerado ou selo fora do commit:" >&2
    printf '%s\n' "$deriva" >&2
    exit 1
  fi
else
  echo "pulado fora do CI: os geradores ja cobram o --check"
fi

printf '\nverificacao completa: 0 problemas\n'
