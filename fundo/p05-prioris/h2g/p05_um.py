"""fundo/p05: roda UM domínio do H2g com a mesma semente que ele recebe na varredura completa (SEMENTE + 100 * posição
na lista ordenada de todos os domínios), num armazém, barramento e diário só dele. Permite rodar os domínios em
paralelo sem mudar o resultado de cada um (o orçamento de varredura acumulado da varredura completa nunca chegou
perto do teto, e nenhum outro estado passa de um domínio ao seguinte).

Uso:  python3 p05_um.py <dominio>     (ESTOCOLOGIA_SAIDA/<dominio>/resultados.json; P05_PRIORI como no rodar.py)
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
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", RAIZ.parent / "dados-reais" / "h2b"))
SAIDA = Path(os.environ["ESTOCOLOGIA_SAIDA"])
SEMENTE = 2027

if __name__ == "__main__":
    nome = sys.argv[1]
    todos = sorted(p.name for p in (RAIZ / "dominios").iterdir() if (p / "pacote.yaml").exists())
    i = todos.index(nome)
    tmp = SAIDA / "_trabalho" / nome
    shutil.rmtree(tmp, ignore_errors=True)
    arm = Armazem(tmp / "armazem")
    bus = Barramento(tmp / "barramento.jsonl")
    diario = Diario(tmp / "diario.json")
    t = time.time()
    r = rodar_dominio(RAIZ / "dominios" / nome, DADOS, arm, bus, diario, SEMENTE + 100 * i)
    r["segundos"] = round(time.time() - t, 1)
    (SAIDA / nome).mkdir(parents=True, exist_ok=True)
    (SAIDA / nome / "resultados.json").write_text(json.dumps(r, indent=1, ensure_ascii=False, default=float))
    shutil.rmtree(tmp, ignore_errors=True)
    print(nome, r["status"], r["segundos"], flush=True)
