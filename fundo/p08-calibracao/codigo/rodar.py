"""Replay contínuo dos fluxos do H3.

Uso:  python3 rodar.py [--fase dev|completo] [--canario-vazado] [fluxo ...]
Sem nomes, roda todos, em paralelo (um processo por fluxo). Saída em $ESTOCOLOGIA_SAIDA (padrão saida/).
"""

from __future__ import annotations

import json
import os
import sys
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

RAIZ = Path(__file__).resolve().parent
DADOS = Path(os.environ.get("ESTOCOLOGIA_DADOS", RAIZ.parent / "dados-reais"))
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))

FLUXOS = {
    "energia": ("fluxos.energia", "Energia"),
    "central": ("fluxos.central", "Central"),
    "bolsa": ("fluxos.bolsa", "Bolsa"),
    "voos": ("fluxos.voos", "Voos"),
    "ciclones": ("fluxos.ciclones", "Ciclones"),
    "prisoes": ("fluxos.prisoes", "Prisoes"),
    "voos23": ("fluxos.voos23", "Voos23"),
    "prisoes20": ("fluxos.prisoes20", "Prisoes20"),
    "prisoes_ri": ("fluxos.prisoes_ri", "PrisoesRI"),
    "voos17": ("fluxos.voos_p08", "Voos17"),
    "voospnw14": ("fluxos.voos_p08", "VoosPNW14"),
    "voos23e1": ("fluxos.voos_p08", "Voos23E1"),
    "voos23e2": ("fluxos.voos_p08", "Voos23E2"),
}


def um(nome: str, fase: str, vazado: bool) -> dict:
    import importlib

    from estocologia.motor import Motor
    mod, cls = FLUXOS[nome]
    f = getattr(importlib.import_module(mod), cls)()
    saida = SAIDA / nome
    r = Motor(f, DADOS, saida, RAIZ, fase=fase, canario_vazado=vazado).rodar()
    if hasattr(f, "estresse_info"):
        r["estresse"] = f.estresse_info
    nome_arq = "resultados_canario_vazado.json" if vazado else ("resultados_dev.json" if fase == "dev" else "resultados.json")
    (saida / nome_arq).write_text(json.dumps(r, indent=1, ensure_ascii=False, default=float))
    return {"fluxo": nome, "segundos": r["segundos"], "placar": r["placar"], "contagens": r["contagens"]}


def main():
    args = sys.argv[1:]
    fase = "dev" if "--fase" in args and args[args.index("--fase") + 1] == "dev" else "completo"
    vazado = "--canario-vazado" in args
    nomes = [a for a in args if a in FLUXOS] or [n for n in FLUXOS if n not in ("voos23", "prisoes20", "prisoes_ri")]   # voos23 (H3b), prisoes20 (H3c) e prisoes_ri (H3d) rodam à parte
    if len(nomes) == 1:
        print(json.dumps(um(nomes[0], fase, vazado), ensure_ascii=False, default=float))
        return
    with ProcessPoolExecutor(max_workers=min(len(nomes), os.cpu_count() or 1)) as ex:
        for r in ex.map(um, nomes, [fase] * len(nomes), [vazado] * len(nomes)):
            print(json.dumps(r, ensure_ascii=False, default=float))


if __name__ == "__main__":
    main()
