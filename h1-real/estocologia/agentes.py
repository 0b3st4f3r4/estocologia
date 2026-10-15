"""Os agentes das 8 faixas. Cada um lê fichas do barramento, faz contas reais e publica uma ficha.

Nenhum agente "faz conta de cabeça": todo número sai de código com semente declarada.
"""

from __future__ import annotations

import cProfile
import hashlib
import io
import json
import pstats
import subprocess
import sys
import time
import tracemalloc
from pathlib import Path

import duckdb
import numpy as np
from scipy import special, stats

from . import estat
from .barramento import Barramento, Ficha
from .diario import Diario, Previsao
from .projeto import Projeto

RAIZ = Path(__file__).resolve().parent.parent


def publicar(bus: Barramento, proj: Projeto, estagio: int, agente: str, tipo: str, payload: dict, pais=()) -> Ficha:
    return bus.publicar(Ficha(proj.nome, estagio, agente, tipo, proj.cabecalho, payload, [p.hash for p in pais]))


# =====================================================================================
# F7 · Transferência de domínio: tradução, SHELF e termo de escopo
# =====================================================================================
def f7_elicitar(elic: dict) -> dict:
    """SHELF v4 (simulado): quartis individuais → consenso (média dos quartis, como no RIO) → Beta."""
    esp = elic["especialistas"]
    q = {k: float(np.mean([e[k] for e in esp])) for k in ("q25", "q50", "q75")}
    ajuste = estat.ajustar_beta_quartis(q["q25"], q["q50"], q["q75"])
    return {"consenso": q, "ajuste": ajuste, "priori": {"a": ajuste["a"], "b": ajuste["b"]},
            "validacao_facial": ajuste["erro_max_quartis"] < 0.01}


def f7(proj: Projeto, bus: Barramento) -> dict:
    trad = publicar(bus, proj, 1, "agent-f7-domain", "ficha_transferencia", proj.traducao())
    elic = proj.elicitacao()
    res = f7_elicitar(elic)
    f_elic = publicar(bus, proj, 1, "agent-f7-domain", "shelf_v4", {**elic, **res}, [trad])
    publicar(bus, proj, 1, "agent-f7-domain", "termo_escopo", proj.termo_escopo(), [trad, f_elic])
    return res["priori"]


# =====================================================================================
# F2 · Dados: contrato, quarentena, ingestão idempotente, leitura colunar com orçamento
# =====================================================================================
class Loja:
    """Armazenamento colunar: Parquet particionado por período, consultado pelo DuckDB."""

    def __init__(self, raiz: Path, proj: Projeto, orcamento_bytes: int = 50_000_000):
        self.raiz = Path(raiz) / proj.nome
        self.proj = proj
        self.orcamento = orcamento_bytes
        self.quarentena: list[dict] = []
        self.log_varredura: list[dict] = []

    def validar_linha(self, r: dict, contrato: dict) -> str | None:
        for nome, spec in contrato["colunas"].items():
            v = r.get(nome)
            if v is None or (isinstance(v, float) and np.isnan(v)):
                if not spec.get("nulo", False):
                    return f"{nome} nulo"
                continue
            if spec["tipo"] == "int" and not isinstance(v, (int, np.integer)):
                return f"{nome} não inteiro"
            if spec["tipo"] == "str" and not isinstance(v, str):
                return f"{nome} não texto"
            if spec["tipo"] == "float" and not isinstance(v, (int, float, np.floating, np.integer)):
                return f"{nome} não numérico"
            if "min" in spec and v < spec["min"]:
                return f"{nome} abaixo do mínimo"
            if "max" in spec and v > spec["max"]:
                return f"{nome} acima do máximo"
            if "valores" in spec and v not in spec["valores"]:
                return f"{nome} fora do domínio"
        return None

    def ingerir(self, linhas: list[dict], periodo: int, contrato: dict) -> dict:
        corpo = json.dumps(linhas, sort_keys=True, default=str).encode()
        h = hashlib.sha256(corpo).hexdigest()[:16]
        destino = self.raiz / f"periodo={periodo}" / f"lote-{h}.parquet"
        if destino.exists():
            return {"lote": h, "periodo": periodo, "idempotente": True, "aceitas": 0, "quarentena": 0}
        chave = contrato["chave"]
        vistos, boas, ruins = set(), [], []
        existentes = self._chaves(periodo, chave)
        for r in linhas:
            motivo = self.validar_linha(r, contrato)
            k = tuple(r.get(c) for c in chave)
            if motivo is None and (k in vistos or k in existentes):
                motivo = "chave primária duplicada"
            if motivo:
                ruins.append({"linha": r, "motivo": motivo, "lote": h})
            else:
                vistos.add(k)
                boas.append(r)
        self.quarentena += ruins
        if boas:
            destino.parent.mkdir(parents=True, exist_ok=True)
            import pyarrow as pa
            import pyarrow.parquet as pq
            tipos = {"int": pa.int64(), "float": pa.float64(), "str": pa.string()}
            esquema = pa.schema([(c, tipos[s["tipo"]]) for c, s in contrato["colunas"].items()])
            tabela = pa.Table.from_pylist(boas, schema=esquema)
            tmp = destino.with_suffix(".tmp")
            pq.write_table(tabela, tmp)
            tmp.rename(destino)  # escrita atômica: um lote interrompido não deixa parquet parcial
        return {"lote": h, "periodo": periodo, "idempotente": False, "aceitas": len(boas), "quarentena": len(ruins),
                "motivos": sorted({x["motivo"] for x in ruins})}

    def _chaves(self, periodo: int, chave: list) -> set:
        arqs = list((self.raiz / f"periodo={periodo}").glob("*.parquet"))
        if not arqs:
            return set()
        rows = duckdb.sql(f"SELECT {', '.join(chave)} FROM read_parquet({[str(a) for a in arqs]})").fetchall()
        return set(rows)

    def ler(self, periodos, colunas, agente: str = "modelagem") -> dict:
        """Projeção explícita e poda de partição; recusa '*' e estouro de orçamento de bytes."""
        if "*" in colunas or not colunas:
            raise PermissionError("consulta sem projeção explícita recusada (F2/MB2)")
        arqs = [a for p in periodos for a in (self.raiz / f"periodo={p}").glob("*.parquet")]
        if not arqs:
            return {c: np.array([]) for c in colunas}
        tamanho = sum(a.stat().st_size for a in arqs)
        frac_cols = len(colunas) / len(self.proj.contrato()["colunas"])
        estimado = int(tamanho * frac_cols)
        if estimado > self.orcamento:
            raise PermissionError(f"orçamento de varredura estourado: {estimado} > {self.orcamento} bytes")
        sel = ", ".join(colunas)
        df = duckdb.sql(f"SELECT {sel} FROM read_parquet({[str(a) for a in arqs]}) ORDER BY {', '.join(self.proj.contrato()['chave'])}").fetchnumpy()
        self.log_varredura.append({"agente": agente, "periodos": list(periodos), "colunas": list(colunas), "arquivos": len(arqs), "bytes_estimados": estimado})
        return {c: np.asarray(df[c]) for c in colunas}


def f2(proj: Projeto, bus: Barramento, loja: Loja, rng) -> dict:
    contrato = proj.contrato()
    f_c = publicar(bus, proj, 2, "agent-f2-data", "datacontract", contrato)
    relat = []
    for p in proj.periodos:
        lote = proj.gerar_lote(p, rng)
        relat.append(loja.ingerir(lote, p, contrato))
    publicar(bus, proj, 2, "agent-f2-data", "ingestao", {"lotes": relat, "quarentena_total": len(loja.quarentena)}, [f_c])
    return {"lotes": relat}


# =====================================================================================
# F5 · Governança: k-anonimato, privacidade diferencial em bits, dossiê
# =====================================================================================
def f5(proj: Projeto, bus: Barramento, linhas: list[dict], dados: dict, rng) -> dict:
    qi = list(proj.quase_ids)
    unicas = {r[proj.unidade]: r for r in linhas}          # k-anonimato conta pessoas, não linhas
    linhas = list(unicas.values())
    k_bruto = estat.k_anonimato(linhas, qi) if qi else None
    gen = [proj.generalizar(r) for r in linhas]
    k_gen = estat.k_anonimato(gen, qi) if qi else None
    k, n, desc = proj.estatistica_publicada(dados)
    dp = []
    for eps in (0.1, 0.5, 1.0, 5.0):
        lib = [estat.laplace_proporcao(k, n, eps, rng) for _ in range(2000)]
        erro = float(np.mean(np.abs(np.array(lib) - k / n)))
        bits = estat.bits_perdidos_laplace(n, k / n, eps, rng, m=40_000)
        dp.append({"epsilon": eps, "erro_absoluto_medio": erro, **bits})
    payload = {
        "k_anonimato": {"quase_identificadores": qi, "k_bruto": k_bruto, "k_generalizado": k_gen, "meta_k": 5,
                         # k-anonimato protege titulares; sem pessoas na base, o k é informativo (sigilo comercial)
                         "cumpre": ((k_gen or 0) >= 5 if qi else True) or not getattr(proj, "tem_titulares", True),
                         "aplicavel_a_titulares": getattr(proj, "tem_titulares", True)},
        "privacidade_diferencial": {"estatistica": desc, "verdadeira": k / n, "n": n, "mecanismo": "Laplace sobre a contagem (sensibilidade 1)", "unidade": proj.unidade, "por_epsilon": dp},
        "lgpd": {"base_legal": "art. 7º, IV (estudo por órgão de pesquisa, dados anonimizados sempre que possível)",
                 "art_20": "decisão automatizada não é tomada sobre titulares: o sistema emite recomendação agregada"},
        "eu_ai_act_anexo_iv": {"descricao_geral": proj.titulo, "processo_de_desenvolvimento": "ciclo de 6 estágios, fichas no barramento",
                                "dados_e_governanca": "datacontract.yaml, quarentena, linhagem por hash de lote",
                                "monitoramento": "deriva KS/Wasserstein (F3), circuit breaker (F8)",
                                "supervisao_humana": "responsável humano nomeado assina o parecer"},
        "trilha_auditoria": {"cadeia_integra": not bus.verificar(), "fichas": len(bus.fichas)},
        "responsavel_humano": {"nome": "sil", "assinatura": "PENDENTE"},
        "dados_simulados": getattr(proj, "dados_simulados", True),
        "origem_dos_dados": getattr(proj, "origem_dos_dados", "mundo simulado"),
    }
    if not getattr(proj, "tem_titulares", True):
        payload["lgpd"] = {"aplicavel": False, "motivo": "a base não tem titulares de dados pessoais (séries agregadas ou de sinistros)"}
    publicar(bus, proj, 2, "agent-f5-governance", "dossie_governanca", payload)
    return payload


# =====================================================================================
# F6 · Justiça e poder
# =====================================================================================
def f6(proj: Projeto, bus: Barramento, ctx: dict) -> dict:
    payload = proj.justica(ctx)
    publicar(bus, proj, 2, "agent-f6-fairness", "auditoria_justica", payload)
    return payload


# =====================================================================================
# F1 · Código: processo gerador, modelo nulo, invariantes, perfil
# =====================================================================================
def f1(proj: Projeto, bus: Barramento, dados: dict, rng) -> dict:
    nulo = proj.nulo(dados, rng)
    t = subprocess.run([sys.executable, "-m", "pytest", "-q", f"tests/test_{proj.nome}.py"], cwd=RAIZ, capture_output=True, text=True)
    pr = cProfile.Profile()
    tracemalloc.start()
    t0 = time.perf_counter()
    pr.enable()
    proj.gerar_lote(proj.periodos[0], np.random.default_rng(0))
    pr.disable()
    dt = time.perf_counter() - t0
    _, pico = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    s = io.StringIO()
    pstats.Stats(pr, stream=s).sort_stats("cumulative").print_stats(3)
    payload = {"modelo_nulo": nulo, "hypothesis": {"ok": t.returncode == 0, "saida": t.stdout.strip().splitlines()[-1] if t.stdout.strip() else t.stderr[-300:]},
               "perfil_lote": {"segundos": dt, "pico_memoria_mb": pico / 2**20}, "semente_declarada": True}
    publicar(bus, proj, 3, "agent-f1-code", "processo_gerador_e_nulo", payload)
    return payload


# =====================================================================================
# F4 · Fronteira: MCMC com diagnósticos, SBC e predição conforme
# =====================================================================================
def amostrar(proj: Projeto, suf: dict, priori: dict, rng, cadeias=4, n=3000, aquec=1000) -> dict:
    x0 = proj.x_inicial(suf)
    lp = lambda x: proj.logpost(x, suf, priori)
    # passo calibrado pela curvatura numérica em x0 (aprox. 2.4/√d desvios da posterior)
    h = 1e-3
    passo = np.empty(proj.dim)
    for j in range(proj.dim):
        e = np.zeros(proj.dim); e[j] = h
        c = -(lp(x0 + e) - 2 * lp(x0) + lp(x0 - e)) / h**2
        passo[j] = 2.4 / np.sqrt(proj.dim) / np.sqrt(max(c, 1e-6))
    ch = np.stack([estat.metropolis(lp, x0 + 0.5 * passo * rng.standard_normal(proj.dim), n + aquec, passo, rng)[aquec:] for _ in range(cadeias)])
    q = np.array([[proj.quantidade(x) for x in c] for c in ch])
    return {"cadeias": ch, "q": q, "rhat_q": estat.rhat(q), "ess_q": estat.ess(q),
            "rhat_max": max(estat.rhat(ch[:, :, j]) for j in range(proj.dim)),
            "ess_min": min(estat.ess(ch[:, :, j]) for j in range(proj.dim))}


def f4(proj: Projeto, bus: Barramento, suf: dict, priori: dict, calib: np.ndarray, rng, alpha=0.1) -> dict:
    a = amostrar(proj, suf, priori, rng)
    q = a["q"].ravel()

    def post_rapida(s, rr, L):
        b = amostrar(proj, s, priori, rr, cadeias=1, n=L * 20, aquec=300)
        return b["q"].ravel()[::20][:L]

    # SBC sobre a quantidade de interesse
    postos = []
    for _ in range(150):
        x = proj.amostra_priori(priori, rng)
        s = proj.simular_suf(x, rng)
        post = post_rapida(s, rng, 39)
        postos.append(int(np.sum(post < proj.quantidade(x))))
    caixas = np.bincount(np.array(postos) * 8 // 40, minlength=8)
    chi2, p_sbc = stats.chisquare(caixas)
    lo, hi = estat.conforme_intervalo(calib, alpha)
    payload = {
        "posterior": {"media": float(q.mean()), "q05": float(np.quantile(q, 0.05)), "q50": float(np.median(q)), "q95": float(np.quantile(q, 0.95))},
        "diagnostico": {"rhat_quantidade": a["rhat_q"], "ess_bulk_quantidade": a["ess_q"], "rhat_max": a["rhat_max"], "ess_min": a["ess_min"],
                        "limiar_rhat": 1.01, "cadeias": int(a["cadeias"].shape[0]), "amostras_por_cadeia": int(a["cadeias"].shape[1])},
        "sbc": {"n_sim": 150, "L": 39, "caixas": caixas.tolist(), "chi2": float(chi2), "p_uniformidade": float(p_sbc)},
        "conforme": {"alpha": alpha, "intervalo": [lo, hi], "n_calibracao": int(calib.size), "garantia": f"cobertura marginal ≥ {1 - alpha:.0%} sob trocabilidade"},
    }
    publicar(bus, proj, 3, "agent-f4-frontier", "fronteira", payload)
    return {**payload, "_amostras": a["cadeias"].reshape(-1, proj.dim), "_q": q}


# =====================================================================================
# F3 · Ciclo de vida: cofre out-of-time, diário de previsões, deriva, política de ignorância
# =====================================================================================
def f3(proj: Projeto, bus: Barramento, loja: Loja, treino: dict, amostras_x: np.ndarray, conforme: list, diario: Diario, K: int) -> dict:
    cofre_periodo = proj.periodos[-1]
    cofre = loja.ler([cofre_periodo], list(proj.contrato()["colunas"]), agente="agent-f3-lifecycle")   # só o F3 abre o cofre
    entradas = proj.entradas_cofre(cofre)
    prevs = proj.previsoes(treino, entradas, amostras_x)
    for pv in prevs:   # pré-registro: tudo entra no diário antes de qualquer desfecho
        for prev, p in pv["probs"].items():
            diario.registrar(Previsao(id=f"{proj.nome}:{pv['id']}:{prev}", projeto=proj.nome, previsor=prev, evento=pv["evento"],
                                      prob=float(np.clip(p, 0.01, 0.99)), taxa_base=pv["taxa_base"], **pv["meta"]))
    h_pre = hashlib.sha256(json.dumps(sorted((k, v.prob) for k, v in diario.itens.items() if v.projeto == proj.nome)).encode()).hexdigest()
    desf = proj.desfechos(cofre)           # o cofre é aberto uma única vez
    for pv in prevs:
        for prev in pv["probs"]:
            diario.resolver(f"{proj.nome}:{pv['id']}:{prev}", desf[pv["id"]])
    # o placar do portão é o do projeto: o mesmo previsor (agent-f4-frontier) prevê nos três projetos
    placar = diario.placar(projeto=proj.nome)
    res_cofre = proj.resultado_unidade(cofre)
    cobertura = float(np.mean((res_cofre >= conforme[0]) & (res_cofre <= conforme[1])))
    ref = proj.entradas_cofre(loja.ler([proj.periodos[-2]], list(proj.contrato()["colunas"]), agente="agent-f3-lifecycle"))["deriva"]
    der = estat.deriva(ref, entradas["deriva"])
    payload = {"cofre": {"periodo": cofre_periodo, "aberto_uma_vez": True, "hash_pre_registro": h_pre, "previsoes": len(prevs)},
               "placar": placar, "cobertura_conforme_out_of_time": cobertura,
               "deriva": {**der, "alarme": der["p_ks"] < 0.01},
               "politica_ignorancia": {"fora_do_dominio": "recusar", "campo_ausente": "fallback determinístico (ação conservadora)", "confianca_baixa": "devolver intervalo conforme"},
               "tentativas_de_especificacao": K, "vazamento_temporal": "nenhum: modelagem leu só períodos " + str(list(proj.periodos[:-1]))}
    publicar(bus, proj, 3, "agent-f3-lifecycle", "ciclo_de_vida", payload)
    return payload


# =====================================================================================
# F8 · Decisão: EVSI, árvore C/L, frequências naturais, HOPs, cláusula de saída
# =====================================================================================
def hops_gif(amostras: dict, caminho: Path, titulo: str, rotulo: str, quadros=24, semente=0):
    """Hypothetical Outcome Plot: cada quadro mostra um desfecho sorteado por opção."""
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib import animation

    rr = np.random.default_rng(semente)
    nomes = list(amostras)
    fig, ax = plt.subplots(figsize=(6, 3.2))
    todos = np.concatenate([np.asarray(v) for v in amostras.values()])
    lo, hi = np.quantile(todos, [0.01, 0.99])

    def quadro(i):
        ax.clear()
        for j, n in enumerate(nomes):
            v = rr.choice(np.asarray(amostras[n]))
            ax.barh(j, v, color=f"C{j}")
            ax.text(v, j, f" {v:.2f}", va="center", fontsize=9)
        ax.set_yticks(range(len(nomes)), nomes, fontsize=9)
        ax.set_xlim(min(lo, 0), hi)
        ax.set_xlabel(rotulo)
        ax.set_title(f"{titulo} (desfecho hipotético {i + 1})", fontsize=10)
        fig.tight_layout()

    anim = animation.FuncAnimation(fig, quadro, frames=quadros)
    anim.save(caminho, writer=animation.PillowWriter(fps=2))
    plt.close(fig)


def f8(proj: Projeto, bus: Barramento, ctx: dict, saida: Path) -> dict:
    d = proj.decisao(ctx)
    q = ctx["f4"]["_q"]
    vi = estat.evpi(q[:: max(1, q.size // 2000)], proj.utilidade, proj.acoes())
    saida.mkdir(parents=True, exist_ok=True)
    gif = saida / f"hops-{proj.nome}.gif"
    hops_gif(d.pop("_hops"), gif, d["hops_titulo"], d["hops_rotulo"])
    payload = {**d, "evpi": vi, "hops": gif.name}
    publicar(bus, proj, 6, "agent-f8-decision", "decisao_executiva", payload)
    return payload
