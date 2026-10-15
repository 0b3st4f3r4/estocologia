"""Esteira de verificação do H1: os 5 portões do AGENTS.md §V.2, aplicados ao que rodar.py gravou.

Uso:  python3 portoes.py [--reproducao]     (sai com 1 se algum portão automático falhar)

--reproducao refaz o ciclo inteiro num diretório temporário e compara os resultados (máquina limpa).
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

import duckdb
import yaml

RAIZ = Path(__file__).resolve().parent
SAIDA = Path(os.environ.get("ESTOCOLOGIA_SAIDA", RAIZ / "saida"))
VOLATEIS = {"perfil_lote", "saida", "detalhe", "segundos"}


def carregar():
    return {p.parent.name: json.loads(p.read_text()) for p in sorted(SAIDA.glob("*/resultados.json"))}


def sem_volateis(o):
    if isinstance(o, dict):
        return {k: sem_volateis(v) for k, v in o.items() if k not in VOLATEIS}
    if isinstance(o, list):
        return [sem_volateis(v) for v in o]
    if isinstance(o, float):
        return round(o, 10)
    return o


def portao1(res, reproducao: bool):
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", "tests"], cwd=RAIZ, capture_output=True, text=True,
                       env={**os.environ, "HYPOTHESIS_PROFILE": "dev"})
    ok = t.returncode == 0
    det = "não verificada (rode com --reproducao)"
    if reproducao:
        with tempfile.TemporaryDirectory() as tmp:
            subprocess.run([sys.executable, "rodar.py", *res], cwd=RAIZ, env={**os.environ, "ESTOCOLOGIA_SAIDA": tmp}, check=True, capture_output=True)
            outra = {p.parent.name: json.loads(p.read_text()) for p in Path(tmp).glob("*/resultados.json")}
        iguais = {k: sem_volateis(res[k]) == sem_volateis(outra.get(k)) for k in res}
        ok = ok and all(iguais.values())
        det = f"resultados idênticos numa segunda execução limpa: {iguais}"
    return ok, f"pytest (Hypothesis): {(t.stdout.strip().splitlines() or ['?'])[-1]}; reprodução: {det}"


MECANISMOS_DA_AUSENCIA = ("MCAR", "MAR", "MNAR", "NAO_DECLARADO")


def checar_ausencia(contrato: dict) -> list[str]:
    """F2/M4: o contrato tem de classificar o mecanismo da ausência, e não só limitar a nulidade.

    Declarado em DECLARACAO_F2_AUSENCIA.md (K = 2 do H1 real), antes de escrever esta função. Recusa: contrato sem
    o campo; mecanismo fora de {MCAR, MAR, MNAR, NAO_DECLARADO}; colunas que não sejam lista de nomes do próprio
    contrato; sensibilidade que não seja {feito: bool, metodos: [...]} ou incoerente; e MAR/MNAR sem análise de
    sensibilidade declarada."""
    aus = contrato.get("ausencia")
    if not isinstance(aus, dict):
        return ["contrato sem o campo ausencia (mecanismo da ausência não declarado)"]
    falhas = []
    mec = aus.get("mecanismo")
    if mec not in MECANISMOS_DA_AUSENCIA:
        falhas.append(f"ausencia.mecanismo inválido: {mec!r}")
    cols = aus.get("colunas")
    if not isinstance(cols, list) or not all(isinstance(c, str) for c in cols):
        falhas.append("ausencia.colunas tem de ser lista de nomes de coluna")
    else:
        fora = [c for c in cols if c not in contrato.get("colunas", {})]
        if fora:
            falhas.append(f"ausencia.colunas cita coluna fora do contrato: {fora}")
    s = aus.get("sensibilidade")
    if not isinstance(s, dict) or not isinstance(s.get("feito"), bool) or not isinstance(s.get("metodos"), list):
        falhas.append("ausencia.sensibilidade tem de ser {feito: bool, metodos: [...]}")
    else:
        if s["feito"] and not s["metodos"]:
            falhas.append("ausencia.sensibilidade.feito = True sem nenhum método declarado")
        if not s["feito"] and s["metodos"]:
            falhas.append("ausencia.sensibilidade.metodos preenchido com feito = False")
        if mec in ("MAR", "MNAR") and not s["feito"]:
            falhas.append(f"ausencia.mecanismo {mec} exige análise de sensibilidade (sensibilidade.feito = True)")
    return falhas


def portao2(res):
    falhas = []
    for nome, r in res.items():
        contrato = yaml.safe_load((SAIDA / nome / "datacontract.yaml").read_text())
        falhas.extend(f"{nome}: {f}" for f in checar_ausencia(contrato))
        arqs = sorted((SAIDA / "loja" / nome).glob("periodo=*/*.parquet"))
        rel = duckdb.sql(f"SELECT * FROM read_parquet({[str(a) for a in arqs]})")
        cols = rel.columns
        if set(cols) != set(contrato["colunas"]):
            falhas.append(f"{nome}: colunas divergem do contrato")
        chave = ", ".join(contrato["chave"])
        dup = duckdb.sql(f"SELECT count(*) - count(DISTINCT ({chave})) FROM read_parquet({[str(a) for a in arqs]})").fetchone()[0]
        if dup:
            falhas.append(f"{nome}: {dup} chaves duplicadas")
        for c, s in contrato["colunas"].items():
            q = f"SELECT count(*) FROM read_parquet({[str(a) for a in arqs]}) WHERE {c} IS NULL" if not s.get("nulo") else None
            if q and duckdb.sql(q).fetchone()[0]:
                falhas.append(f"{nome}: {c} com nulos")
            if "min" in s and duckdb.sql(f"SELECT count(*) FROM read_parquet({[str(a) for a in arqs]}) WHERE {c} < {s['min']}").fetchone()[0]:
                falhas.append(f"{nome}: {c} abaixo do mínimo")
            if "max" in s and duckdb.sql(f"SELECT count(*) FROM read_parquet({[str(a) for a in arqs]}) WHERE {c} > {s['max']}").fetchone()[0]:
                falhas.append(f"{nome}: {c} acima do máximo")
        if not r["banca"]["MB2_lote_corrompido"]["passou"]:
            falhas.append(f"{nome}: quarentena/idempotência falhou na banca")
        if any("*" in v["colunas"] for v in r["varredura"]):
            falhas.append(f"{nome}: consulta sem projeção")
    return not falhas, "contratos (com o mecanismo da ausência), chaves, faixas, quarentena e projeção ok" if not falhas else "; ".join(falhas)


def portao3(res):
    det, ok = [], True
    for nome, r in res.items():
        d, s, c = r["f4"]["diagnostico"], r["f4"]["sbc"], r["f3"]["cobertura_conforme_out_of_time"]
        a = r["f4"]["conforme"]["alpha"]
        o = d["rhat_max"] < 1.01 and d["ess_min"] > 400 and s["p_uniformidade"] > 0.01 and c >= 1 - a - 0.05
        ok &= o
        det.append(f"{nome}: R-hat {d['rhat_max']:.4f}, ESS {d['ess_min']:.0f}, SBC p {s['p_uniformidade']:.2f}, cobertura conforme {c:.1%} (meta {1 - a:.0%})")
    return ok, "; ".join(det)


def portao4(res):
    det, ok = [], True
    for nome, r in res.items():
        cofre = r["f3"]["cofre"]["periodo"]
        leituras = r["varredura"]
        # a leitura do cofre só pode acontecer pelo F3 (depois do pré-registro) e pela banca
        antes_f3 = [v for v in leituras if v.get("agente") not in ("agent-f3-lifecycle", "banca")]
        vaz = [v for v in antes_f3 if cofre in v["periodos"]]
        m = r["lentes"].get("Morfonomia", {}); inv = next((m[k] for k in ("invariante", "invariante_entre_coortes", "invariante_entre_particoes") if k in m), None)
        hab = r["f3"]["placar"].get("agent-f4-frontier", {}).get("habilidade", -1)
        o = not vaz and inv is not False and hab > 0
        ok &= o
        det.append(f"{nome}: leituras do cofre fora do F3 = {len(vaz)}; invariância entre ambientes = {inv}; habilidade fora do tempo = {hab:+.3f}")
    return ok, "; ".join(det) + "; insensibilidade a Δt: não se aplica (tempo discreto nos três projetos)"


def portao5(res):
    det, ok, pend = [], True, []
    for nome, r in res.items():
        f5, f6, f8 = r["f5"], r["f6"], r.get("f8", {})
        anexo = f5["eu_ai_act_anexo_iv"]
        o = (all(anexo.values()) and f5["trilha_auditoria"]["cadeia_integra"] and f5["k_anonimato"]["cumpre"]
             and bool(f6.get("inversao_metrica")) and all(k in f8 for k in ("circuit_breaker", "arvore_CL", "frequencias_naturais")))
        ok &= o
        if f5["responsavel_humano"]["assinatura"] != "ASSINADO":
            pend.append(f"{nome}: parecer de risco sem assinatura de {f5['responsavel_humano']['nome']}")
        det.append(f"{nome}: Anexo IV completo, trilha íntegra, k-anonimato {f5['k_anonimato']['k_bruto']}→{f5['k_anonimato']['k_generalizado']}, "
                   f"inversão de métrica e cláusula de saída presentes" if o else f"{nome}: incompleto")
    return ok, "; ".join(det), pend


def main():
    res = carregar()
    if not res:
        print("nada em", SAIDA, "— rode rodar.py antes")
        return 1
    for nome in res:  # exporta o contrato de cada projeto como datacontract.yaml
        from rodar import projetos
        (SAIDA / nome / "datacontract.yaml").write_text(yaml.safe_dump(projetos()[nome]().contrato(), allow_unicode=True, sort_keys=False))
    rep = "--reproducao" in sys.argv
    out = {}
    out["Portão 1 · F1/M0 invariantes e reprodução"] = portao1(res, rep)
    out["Portão 2 · F2/M4 contrato e dados"] = portao2(res)
    out["Portão 3 · F4 convergência, SBC e conforme"] = portao3(res)
    out["Portão 4 · F3/M15 corte temporal e invariância"] = portao4(res)
    ok5, d5, pend = portao5(res)
    out["Portão 5 · F5/F6/F8 governança e decisão"] = (ok5, d5)
    for k, (o, d) in out.items():
        print(f"{'PASSA' if o else 'FALHA'}  {k}: {d}")
    for p in pend:
        print(f"PENDENTE (humano)  {p}")
    (SAIDA / "portoes.json").write_text(json.dumps({"portoes": {k: {"ok": o, "detalhe": d} for k, (o, d) in out.items()}, "pendencias_humanas": pend},
                                                   indent=1, ensure_ascii=False))
    return 0 if all(o for o, _ in out.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
