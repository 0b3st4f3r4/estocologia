"""PhysioNet/CinC 2019 (sepse precoce): um arquivo .psv = uma internação; a linha da tabela é a internação.

Regra declarada em DECLARACAO.md (Parte 2), antes de rodar:
- janela = as 6 primeiras horas registradas (ICULOS 1 a 6), que é onde o gatilho é medido;
- entra a internação que tem as 6 horas na janela e está sem alarme nela (SepsisLabel = 0 em ICULOS 1 a 6);
- y = 1 se SepsisLabel = 1 em alguma hora depois da janela (ICULOS >= 7);
- chave fr = maior Resp da janela; os demais candidatos são o maior/menor valor declarado da janela;
- sexo (Gender), idade (Age) e tempo_hospital_uti (HospAdmTime) são constantes por internação: vale o primeiro
  valor não ausente da janela;
- ambiente = sítio = sistema (pasta) x tipo de UTI: MICU se Unit1 = 1, SICU se Unit2 = 1, "outros" caso contrário
  (flag ausente ou 0), pelo primeiro valor não ausente da janela; nível com menos de 200 linhas vira "outros" do
  seu sistema (regra mecânica declarada). Cofre = os três níveis do sistema B;
- faltante em qualquer candidato (ou no y) tira a internação da amostra.

Leitura direta e leve: nenhum arquivo é carregado inteiro em DataFrame; só o cabeçalho, as 6 linhas da janela e
uma varredura da coluna do rótulo depois da janela.
"""

from __future__ import annotations

import math
from pathlib import Path

import pandas as pd

JANELA = 6
FLOOR_NIVEL = 200
RAIZ_DADOS = Path("challenge-2019") / "1.0.0" / "training"
SISTEMAS = {"A": "training_setA", "B": "training_setB"}
VITAIS = {"fr": ("Resp", max), "temp": ("Temp", max), "pas": ("SBP", min),
          "pad": ("DBP", min), "spo2": ("O2Sat", min), "fc": ("HR", max)}
CONSTANTES = (("idade", "Age"), ("sexo", "Gender"), ("tempo_hospital_uti", "HospAdmTime"))
CANDIDATOS = ["fr", "temp", "pas", "pad", "spo2", "fc", "idade", "sexo", "tempo_hospital_uti"]


def _num(s: str) -> float:
    return float(s) if s not in ("", "NaN", "nan", "NA") else math.nan


def _internacao(caminho: Path, sistema: str) -> dict | None:
    linhas = caminho.read_text().splitlines()
    if len(linhas) < 1 + JANELA:
        return None
    indice = {c: i for i, c in enumerate(linhas[0].split("|"))}
    if not {"SepsisLabel", "ICULOS", "Resp", "Temp", "SBP", "DBP", "O2Sat", "HR"} <= set(indice):
        return None
    janela = [l.split("|") for l in linhas[1:1 + JANELA]]
    if any(len(r) != len(indice) for r in janela):
        return None
    try:
        if [int(float(r[indice["ICULOS"]])) for r in janela] != list(range(1, JANELA + 1)):
            return None
    except ValueError:
        return None
    if any(r[indice["SepsisLabel"]].strip() not in ("0", "0.0") for r in janela):
        return None                                  # já havia alarme dentro da janela: fora
    linha = {"y": 1.0 if any(l.endswith("|1") for l in linhas[1 + JANELA:]) else 0.0,
             "unidade": f"{sistema}:{caminho.stem}", "sistema": sistema}
    for nome, (coluna, agregacao) in VITAIS.items():
        vals = [v for v in (_num(r[indice[coluna]]) for r in janela) if not math.isnan(v)]
        linha[nome] = agregacao(vals) if vals else math.nan
    for nome, coluna in CONSTANTES:
        valor = next((v for v in (_num(r[indice[coluna]]) for r in janela) if not math.isnan(v)), math.nan)
        linha[nome] = valor
    u1 = next((r[indice["Unit1"]].strip() for r in janela if r[indice["Unit1"]].strip() not in ("", "NaN")), "")
    u2 = next((r[indice["Unit2"]].strip() for r in janela if r[indice["Unit2"]].strip() not in ("", "NaN")), "")
    linha["tipo"] = "MICU" if u1 == "1" else ("SICU" if u2 == "1" else "outros")
    return linha


def preparar(dados: Path) -> pd.DataFrame:
    base = Path(dados) / RAIZ_DADOS
    registros = []
    for sistema, pasta in SISTEMAS.items():
        for caminho in sorted((base / pasta).iterdir()):
            if caminho.suffix == ".psv":
                r = _internacao(caminho, sistema)
                if r is not None:
                    registros.append(r)
    d = pd.DataFrame(registros)
    if d.empty:
        raise ValueError(f"nenhuma internação lida em {base}")
    d = d.dropna(subset=["y", *CANDIDATOS]).copy()
    d["ambiente"] = d["sistema"] + "_" + d["tipo"]
    contagem = d["ambiente"].value_counts()
    pequenos = [a for a, n in contagem.items() if n < FLOOR_NIVEL]
    if pequenos:
        d.loc[d["ambiente"].isin(pequenos), "ambiente"] = d.loc[d["ambiente"].isin(pequenos), "sistema"] + "_outros"
    d["unidade"] = d["unidade"].astype(str)
    return d[["y", "ambiente", "unidade", *CANDIDATOS]].reset_index(drop=True)
