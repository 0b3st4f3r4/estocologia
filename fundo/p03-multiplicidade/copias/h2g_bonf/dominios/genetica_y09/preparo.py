"""genetica_y09: Y = 9º gene sorteado (ver ../_kemmeren/comum.py e ../../DECLARACAO_H2f.md)."""

import importlib.util
from pathlib import Path

_spec = importlib.util.spec_from_file_location("kemmeren_comum", Path(__file__).resolve().parents[1] / "_kemmeren" / "comum.py")
_comum = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_comum)


def preparar(dados: Path):
    return _comum.preparar(dados, 9)
