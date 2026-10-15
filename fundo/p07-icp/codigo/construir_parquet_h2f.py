"""Constrói dados-reais/h2f/kemmeren2014_obs_int.parquet a partir de Kemmeren.RData.

Converte o objeto `data` (lista) do RData para uma tabela única, exatamente como documentado em
dados-reais/h2f/FONTES.md. Nada é subconjuntado: 262 observacionais + 1479 intervencionais = 1741 linhas,
5 colunas de metadados na ordem sample_id, kind, deleted_gene, deleted_gene_col_index_1based, plate,
seguidas das 6170 colunas de genes nomeadas pelos ORFs. Valores float64, sem transformação.

Uso:  python3 construir_parquet_h2f.py [caminho_do_Kemmeren.RData] [caminho_do_parquet]
"""

from __future__ import annotations

import sys
from pathlib import Path

import numpy as np
import pandas as pd
import rdata

RAIZ = Path(__file__).resolve().parents[3]
RData = Path(sys.argv[1]) if len(sys.argv) > 1 else RAIZ / "dados-reais" / "h2f" / "Kemmeren.RData"
Parquet = Path(sys.argv[2]) if len(sys.argv) > 2 else RAIZ / "dados-reais" / "h2f" / "kemmeren2014_obs_int.parquet"


def construir(origem: Path) -> pd.DataFrame:
    data = rdata.conversion.convert(rdata.parser.parse_file(str(origem)))["data"]
    obs, inter = np.asarray(data["obs"]), np.asarray(data["int"])
    genes = np.asarray(data["genenames"]).astype(str)
    mutantes = np.asarray(data["mutantnames"]).astype(str)
    intpos = np.asarray(data["intpos"]).ravel().astype(np.int64)
    plateobs = np.asarray(data["plateobs"]).ravel().astype(bool)
    plateint = np.asarray(data["plateint"]).ravel().astype(bool)
    idobs = np.asarray(data["obs"].coords[data["obs"].dims[0]]).astype(str)
    idint = np.asarray(data["int"].coords[data["int"].dims[0]]).astype(str)
    assert obs.shape == (262, 6170) and inter.shape == (1479, 6170)
    assert intpos.size == 1479 and plateobs.size == 262 and plateint.size == 1479
    assert np.array_equal(mutantes, genes[intpos - 1])

    n_obs, n_int = obs.shape[0], inter.shape[0]
    fora = pd.DataFrame({
        "sample_id": np.concatenate([idobs, idint]),
        "kind": np.array(["observational"] * n_obs + ["interventional"] * n_int),
        "deleted_gene": np.concatenate([np.full(n_obs, None, dtype=object), mutantes]),
        "deleted_gene_col_index_1based": np.concatenate([np.full(n_obs, 0, dtype=np.int64), intpos]),
        "plate": np.concatenate([plateobs, plateint]),
    })
    # nas linhas observacionais não há gene deletado: a coluna do índice fica NA, como deleted_gene
    fora["deleted_gene_col_index_1based"] = pd.array(
        np.where(np.arange(len(fora)) < n_obs, None, fora["deleted_gene_col_index_1based"]), dtype="Int64")
    df = pd.concat([fora, pd.DataFrame(np.vstack([obs, inter]), columns=genes)], axis=1)
    return df


if __name__ == "__main__":
    df = construir(RData)
    df.to_parquet(Parquet, index=False)
    print(f"{Parquet} {df.shape[0]}x{df.shape[1]}")
