"""Baixa o catálogo do Rdatasets (commit fixo) e os CSV que passam o filtro de metadados da regra R1 (DECLARACAO.md).

Uso:  python3 escala/baixar.py <pasta_destino>
Grava <destino>/rdatasets/catalogo.csv, <destino>/rdatasets/csv/<pacote>/<item>.csv e <destino>/rdatasets/SHA256SUMS.
"""

import hashlib
import sys
import time
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import pandas as pd

COMMIT = "1dcc2bf5f955cc1224a3e1307256e1fe86b68dae"
BASE = f"https://raw.githubusercontent.com/vincentarelbundock/Rdatasets/{COMMIT}"


def filtro_metadados(c: pd.DataFrame) -> pd.DataFrame:
    """R1, passo 1: só metadados do catálogo, sem abrir nenhum dado."""
    m = (c.Rows >= 300) & (c.Rows <= 200_000) & (c.n_numeric >= 3) & ((c.n_factor + c.n_character + c.n_binary) >= 1)
    return c[m].sort_values(["Package", "Item"]).reset_index(drop=True)


def _get(url: str, destino: Path, tentativas: int = 3) -> str:
    for t in range(tentativas):
        try:
            with urllib.request.urlopen(url, timeout=120) as r:
                b = r.read()
            destino.parent.mkdir(parents=True, exist_ok=True)
            destino.write_bytes(b)
            return hashlib.sha256(b).hexdigest()
        except Exception as e:  # noqa: BLE001
            erro = e
            time.sleep(2 * (t + 1))
    return f"ERRO {type(erro).__name__}: {erro}"


def main(dest: Path):
    raiz = dest / "rdatasets"
    h = _get(f"{BASE}/datasets.csv", raiz / "catalogo.csv")
    c = filtro_metadados(pd.read_csv(raiz / "catalogo.csv"))
    print("catálogo", h, "passam metadados:", len(c), flush=True)
    alvos = [(r.Package, r.Item) for r in c.itertuples()]

    def um(pi):
        p, i = pi
        f = raiz / "csv" / p / f"{i}.csv"
        if f.exists():
            return p, i, hashlib.sha256(f.read_bytes()).hexdigest()
        return p, i, _get(f"{BASE}/csv/{p}/{i}.csv", f)

    with ThreadPoolExecutor(8) as ex:
        res = list(ex.map(um, alvos))
    linhas = [f"{h}  catalogo.csv"] + [f"{s}  csv/{p}/{i}.csv" for p, i, s in res if not s.startswith("ERRO")]
    (raiz / "SHA256SUMS").write_text("\n".join(linhas) + "\n")
    erros = [(p, i, s) for p, i, s in res if s.startswith("ERRO")]
    (raiz / "erros_download.txt").write_text("".join(f"{p}/{i}: {s}\n" for p, i, s in erros))
    print("baixados", len(res) - len(erros), "erros", len(erros))


if __name__ == "__main__":
    main(Path(sys.argv[1]))
