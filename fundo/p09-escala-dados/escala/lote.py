"""Roda o funil em lote: um subprocesso por domínio, N em paralelo, com prazo por domínio.

Uso:  python3 escala/lote.py <pasta_dominios> <pasta_dados> <pasta_saida> [--trabalhadores 4] [--prazo 900]
      [--perfil] [--limite N]
A semente do domínio i (ordem alfabética) é 2027 + 100·i, como no rodar.py. Grava <saida>/_lote.json.
"""

import argparse
import json
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

AQUI = Path(__file__).resolve().parent
SEMENTE = 2027


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dominios")
    ap.add_argument("dados")
    ap.add_argument("saida")
    ap.add_argument("--trabalhadores", type=int, default=4)
    ap.add_argument("--prazo", type=int, default=900)
    ap.add_argument("--perfil", action="store_true")
    ap.add_argument("--limite", type=int, default=None)
    a = ap.parse_args()
    pastas = sorted(p for p in Path(a.dominios).iterdir() if (p / "pacote.yaml").exists())
    if a.limite:
        pastas = pastas[: a.limite]
    saida = Path(a.saida)
    saida.mkdir(parents=True, exist_ok=True)
    t0 = time.time()

    def um(ip):
        i, p = ip
        cmd = [sys.executable, str(AQUI / "um.py"), str(p), a.dados, str(saida), str(SEMENTE + 100 * i)]
        if a.perfil:
            cmd.append("--perfil")
        ti = time.time()
        try:
            cp = subprocess.run(cmd, capture_output=True, text=True, timeout=a.prazo)
            rc = cp.returncode
            err = cp.stderr[-800:] if rc else ""
        except subprocess.TimeoutExpired:
            rc, err = "prazo", f"passou de {a.prazo} s"
        med_f = saida / p.name / "medida.json"
        med = json.loads(med_f.read_text()) if med_f.exists() and rc != "prazo" else {"dominio": p.name, "status": "erro"}
        med["retorno"] = rc
        if err:
            med["stderr"] = err
            if rc == "prazo":
                med["erro"] = "prazo"
        med["segundos_externos"] = round(time.time() - ti, 2)
        print(f"{p.name} {med.get('status', '')[:50]} {med['segundos_externos']}s", flush=True)
        return med

    with ThreadPoolExecutor(a.trabalhadores) as ex:
        meds = list(ex.map(um, enumerate(pastas)))
    lote = {"dominios": len(pastas), "trabalhadores": a.trabalhadores, "prazo": a.prazo, "perfil": a.perfil,
            "segundos_parede_total": round(time.time() - t0, 1), "medidas": meds}
    (saida / "_lote.json").write_text(json.dumps(lote, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
