"""Roda o funil do H2 sobre todos os domínios em dominios/ (ou os pedidos) e grava saida/.

Uso:  python3 rodar.py [dominio ...]
Os dados vêm de ../dados-reais/h2 (ou de $ESTOCOLOGIA_DADOS) e o resultado vai para saida/ (ou $ESTOCOLOGIA_SAIDA).
"""

import json
import os
import shutil
import sys
import time
from pathlib import Path

from estocologia.armazem import Armazem
from estocologia.barramento import Barramento
from estocologia.diario import Diario
from estocologia.funil import rodar_dominio

RAIZ = Path(__file__).resolve().parent
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", RAIZ.parent / "dados-reais" / "h2"))
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
SEMENTE = 2027


def dominios():
    return sorted(p.name for p in (RAIZ / "dominios").iterdir() if (p / "pacote.yaml").exists())


if __name__ == "__main__":
    pedidos = sys.argv[1:] or dominios()
    shutil.rmtree(SAIDA, ignore_errors=True)
    arm = Armazem(SAIDA / "armazem")
    bus = Barramento(SAIDA / "barramento.jsonl")
    diario = Diario(SAIDA / "diario.json")
    resumo = {}
    for i, nome in enumerate(pedidos):
        t = time.time()
        r = rodar_dominio(RAIZ / "dominios" / nome, DADOS, arm, bus, diario, SEMENTE + 100 * i)
        r["segundos"] = round(time.time() - t, 1)
        (SAIDA / nome).mkdir(parents=True, exist_ok=True)
        (SAIDA / nome / "resultados.json").write_text(json.dumps(r, indent=1, ensure_ascii=False, default=float))
        resumo[nome] = {"area": r["area"], "status": r["status"], "segundos": r["segundos"]}
        print(nome, json.dumps(resumo[nome], ensure_ascii=False), flush=True)
    diario.salvar()
    (SAIDA / "registro_varredura.json").write_text(json.dumps({"registro": arm.registro, "gasto_por_agente": arm.gasto,
                                                               "orcamento_por_agente": arm.orcamento}, indent=1, ensure_ascii=False))
    (SAIDA / "resumo.json").write_text(json.dumps(resumo, indent=1, ensure_ascii=False))
