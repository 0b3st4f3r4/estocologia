"""Roda o ciclo de 6 estágios para os projetos pedidos (padrão: todos) e grava em saida/."""

import json
import os
import shutil
import sys
from pathlib import Path

from estocologia.barramento import Barramento
from estocologia.diario import Diario
from estocologia.orquestrador import rodar

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))


def projetos():
    # H1 em domínio real: os mesmos três projetos, sobre as bases de ../dados-reais
    from projetos.a_real import ProjetoA
    from projetos.b_real import ProjetoB
    from projetos.c_real import ProjetoC
    return {"a": ProjetoA, "b": ProjetoB, "c": ProjetoC}


if __name__ == "__main__":
    pedidos = sys.argv[1:] or list(projetos())
    if SAIDA.exists() and not sys.argv[1:]:
        shutil.rmtree(SAIDA)
    for nome in pedidos:
        shutil.rmtree(SAIDA / nome, ignore_errors=True)
        shutil.rmtree(SAIDA / "loja" / nome, ignore_errors=True)
    bus = Barramento(SAIDA / "barramento.jsonl")
    diario = Diario(SAIDA / "diario.json")
    resumo = {}
    for i, nome in enumerate(pedidos):
        r = rodar(projetos()[nome](), bus, diario, SAIDA, semente=2026 + 100 * i)
        resumo[nome] = {"aprovado": r["aprovado"], "portao": r["portao_m15"], "log": r["log"]}
        print(nome, json.dumps(resumo[nome], ensure_ascii=False))
    diario.salvar()
    (SAIDA / "resumo.json").write_text(json.dumps(resumo, indent=1, ensure_ascii=False))
