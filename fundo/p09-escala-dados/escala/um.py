"""Roda o funil do H2g num único domínio, num processo próprio, e grava tempo e pico de memória.

Uso:  python3 escala/um.py <pasta_dominio> <pasta_dados> <pasta_saida> <semente> [--perfil]
Grava <saida>/<nome>/resultados.json (formato do rodar.py), <saida>/<nome>/medida.json e, com --perfil,
<saida>/<nome>/perfil.pstats e o pico do tracemalloc.
"""

import cProfile
import json
import resource
import sys
import time
import traceback
import tracemalloc
from pathlib import Path

RAIZ = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(RAIZ))

from estocologia.armazem import Armazem  # noqa: E402
from estocologia.barramento import Barramento  # noqa: E402
from estocologia.diario import Diario  # noqa: E402
from estocologia.funil import rodar_dominio  # noqa: E402


def main(pasta: Path, dados: Path, saida: Path, semente: int, perfil: bool):
    nome = pasta.name
    out = saida / nome
    out.mkdir(parents=True, exist_ok=True)
    med = {"dominio": nome, "semente": semente, "perfil": perfil}
    t0, c0 = time.time(), time.process_time()
    prof = cProfile.Profile() if perfil else None
    if perfil:
        tracemalloc.start()
        prof.enable()
    try:
        arm = Armazem(out / "armazem")
        bus = Barramento(out / "barramento.jsonl")
        diario = Diario(out / "diario.json")
        r = rodar_dominio(pasta, dados, arm, bus, diario, semente)
        diario.salvar()
        r["segundos"] = round(time.time() - t0, 2)
        (out / "resultados.json").write_text(json.dumps(r, indent=1, ensure_ascii=False, default=float))
        med["status"] = r["status"]
        med["etapa_final"] = r["status"].split(":")[0]
        med["bytes_tabela"] = r["ingestao"]["bytes_tabela"]
        med["linhas"] = r["ingestao"]["linhas"]
    except Exception as e:  # noqa: BLE001
        med["status"] = "erro"
        med["erro"] = f"{type(e).__name__}: {e}"[:500]
        med["traceback_final"] = traceback.format_exc().strip().splitlines()[-3:]
    if perfil:
        prof.disable()
        _, pico = tracemalloc.get_traced_memory()
        med["tracemalloc_pico_bytes"] = int(pico)
        tracemalloc.stop()
        prof.dump_stats(str(out / "perfil.pstats"))
    med["segundos_parede"] = round(time.time() - t0, 3)
    med["segundos_cpu"] = round(time.process_time() - c0, 3)
    med["rss_pico_kb"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    (out / "medida.json").write_text(json.dumps(med, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    a = sys.argv[1:]
    main(Path(a[0]), Path(a[1]), Path(a[2]), int(a[3]), "--perfil" in a)
