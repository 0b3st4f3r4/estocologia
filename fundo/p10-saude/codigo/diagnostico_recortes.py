"""Diagnóstico dos recortes do domínio saude_sepse_alerta (medida descritiva, fora do funil).

Percorre os 40 336 arquivos de treino do challenge-2019 e conta em que regra cada internação sai, além do
tamanho e da taxa do desfecho por ambiente. Só mede: nenhuma escolha de desenho se apoia neste script.

Uso:  python3 diagnostico_recortes.py <pasta_dados> [caminho_resultados.json]
"""

from __future__ import annotations

import json
import sys
from collections import Counter
from pathlib import Path

RAIZ = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(RAIZ / "dominios" / "saude_sepse_alerta"))

import math  # noqa: E402

import pandas as pd  # noqa: E402

from preparo import CANDIDATOS, JANELA, RAIZ_DADOS, SISTEMAS, _internacao, preparar  # noqa: E402


def _motivo(caminho: Path, sistema: str) -> str:
    """A regra que tira a internação, na mesma ordem do preparo."""
    linhas = caminho.read_text().splitlines()
    if len(linhas) < 1 + JANELA:
        return "menos de 6 horas registradas"
    indice = {c: i for i, c in enumerate(linhas[0].split("|"))}
    janela = [l.split("|") for l in linhas[1:1 + JANELA]]
    if any(len(r) != len(indice) for r in janela):
        return "linha truncada"
    try:
        if [int(float(r[indice["ICULOS"]])) for r in janela] != list(range(1, JANELA + 1)):
            return "ICULOS da janela fora de 1 a 6"
    except ValueError:
        return "ICULOS ilegível"
    if any(r[indice["SepsisLabel"]].strip() not in ("0", "0.0") for r in janela):
        return "já com alarme na janela"
    r = _internacao(caminho, sistema)
    if r is None:
        return "outro"
    faltando = [c for c in CANDIDATOS if isinstance(r.get(c), float) and math.isnan(r[c])]
    if faltando:
        return "candidato ausente na janela: " + ",".join(sorted(faltando))
    return "entra"


def main() -> int:
    dados = Path(sys.argv[1])
    motivos = Counter()
    for sistema, pasta in SISTEMAS.items():
        for caminho in sorted((dados / RAIZ_DADOS / pasta).iterdir()):
            if caminho.suffix == ".psv":
                motivos[_motivo(caminho, sistema)] += 1
    d = preparar(dados)
    tabela = (d.groupby("ambiente")["y"].agg(["size", "mean"]).rename(columns={"size": "n", "mean": "taxa_y"})
              .join(d.groupby("ambiente")["fr"].median().rename("fr_mediana")))
    print("arquivos:", sum(motivos.values()))
    for k, v in motivos.most_common():
        print(f"  {v:6d}  {k}")
    print("\nlinhas por ambiente (depois dos recortes):")
    print(tabela.to_string())
    print("\ntreino (A) y =", round(d.loc[d.ambiente.str.startswith("A"), "y"].mean(), 5),
          "| cofre (B) y =", round(d.loc[d.ambiente.str.startswith("B"), "y"].mean(), 5))
    saida = RAIZ / "saida" / "diagnostico.json"
    saida.parent.mkdir(parents=True, exist_ok=True)
    saida.write_text(json.dumps({"arquivos": sum(motivos.values()), "motivos": motivos.most_common(),
                                 "por_ambiente": tabela.reset_index().to_dict("records")}, indent=1, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
