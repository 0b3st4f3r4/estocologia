"""Motor da malha contínua do H3: replay de um fluxo real sob relógio, com cofre fora do tempo.

Papéis:
- **F2.** Ingere as tabelas de eventos e serve só a janela (último tick, agora].
- **F7.** O fluxo traz contrato, escopo, decisão e cabeçalho.
- **F4.** O modelo do fluxo ajusta e prevê, e o calibrador corrige.
- **F3.** Sensores de deriva com histerese e reajuste. No corte, congela a especificação num hash de
  pré-registro e abre o cofre uma única vez.
- **F8.** Mantém o disjuntor e a política de ignorância: modelo recusado, reserva oficial e ficha de
  post-mortem.
- **F1.** Mantém o reserva determinístico e o canário de vazamento.
- **M15.** Fecha com o resumo.

Cada previsão vai para o diário colunar com t_emissão, o t_disp máximo do estado no momento da emissão, a
previsão do modelo e a do reserva (as duas, sempre), a origem da previsão oficial e, ao resolver, o t_disp do
rótulo. A auditoria de vazamento sai direto dessas colunas.
"""

from __future__ import annotations

import hashlib
import inspect
import json
import tempfile
import time
from pathlib import Path

import numpy as np
import pandas as pd

from .armazem import Armazem, Relogio
from .barramento import Barramento, Ficha
from .monitor import Disjuntor, SensorDeriva, pinball

INF = 2**62


def sha(obj) -> str:
    return hashlib.sha256(json.dumps(obj, sort_keys=True, default=str, ensure_ascii=False).encode()).hexdigest()


def sha_arquivo(p: Path) -> str:
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


class Diario:
    """Diário de previsões colunar (PLAN §4), com piso de 1% e teto de 99% nas probabilidades."""

    def __init__(self, tipo: str, niveis=None):
        self.tipo, self.niveis = tipo, (np.asarray(niveis) if niveis is not None else None)
        self.linhas: dict[str, dict] = {}

    def registrar(self, id_, agora, t_alvo, estado_tmax, modelo, reserva, origem, canario, extra=None):
        if id_ in self.linhas:
            raise ValueError(f"previsão {id_} já registrada (o pré-registro é imutável)")
        if self.tipo == "binario":
            modelo, reserva = float(np.clip(modelo, 0.01, 0.99)), float(np.clip(reserva, 0.01, 0.99))
        else:
            modelo, reserva = np.sort(np.asarray(modelo, float)), np.sort(np.asarray(reserva, float))
        self.linhas[id_] = {"id": id_, "t_emissao": int(agora), "t_alvo": int(t_alvo), "estado_tmax": int(estado_tmax),
                            "modelo": modelo, "reserva": reserva, "origem": origem, "canario": canario,
                            "y": None, "t_rotulo": None, **(extra or {})}

    def resolver(self, id_, y, t_rotulo) -> dict | None:
        l = self.linhas.get(id_)
        if l is None or l["y"] is not None:
            return None
        l["y"], l["t_rotulo"] = float(y), int(t_rotulo)
        return l

    def perda(self, l, qual: str) -> float:
        p = l[qual]
        if self.tipo == "binario":
            return (p - l["y"]) ** 2
        return pinball(l["y"], p, self.niveis)

    def tabela(self) -> pd.DataFrame:
        df = pd.DataFrame(list(self.linhas.values()))
        if self.tipo == "quantis" and len(df):
            for i, nv in enumerate(self.niveis):
                df[f"modelo_q{nv:g}"] = [m[i] for m in df["modelo"]]
                df[f"reserva_q{nv:g}"] = [r[i] for r in df["reserva"]]
            df = df.drop(columns=["modelo", "reserva"])
        return df


class Motor:
    def __init__(self, fluxo, dados: Path, saida: Path, raiz_h3: Path, fase: str = "completo", canario_vazado: bool = False):
        self.f, self.dados, self.saida = fluxo, Path(dados), Path(saida)
        self.raiz = Path(raiz_h3)
        self.fase = fase
        self.vazado = canario_vazado
        self.saida.mkdir(parents=True, exist_ok=True)
        for velho in ("barramento.jsonl",):
            (self.saida / velho).unlink(missing_ok=True)
        self.bus = Barramento(self.saida / "barramento.jsonl")
        self.diario = Diario(fluxo.tipo, getattr(fluxo, "niveis", None))
        self.eventos_op: list[dict] = []

    # ---------- fichas ----------
    def ficha(self, estagio, agente, tipo, payload, pais=()):
        return self.bus.publicar(Ficha(projeto=self.f.nome, estagio=estagio, agente=agente, tipo=tipo,
                                       cabecalho=self.f.cabecalho, payload=payload, pais=list(pais)))

    def especificacao(self) -> dict:
        mod = inspect.getmodule(type(self.f))
        return {"fluxo": self.f.nome, "modulo_sha256": sha_arquivo(Path(mod.__file__)),
                "motor_sha256": sha_arquivo(Path(__file__)), "monitor_sha256": sha_arquivo(Path(__file__).with_name("monitor.py")),
                "criterios_sha256": sha_arquivo(self.raiz / "CRITERIOS.md"),
                **({"criterios_h3b_sha256": sha_arquivo(self.raiz / "CRITERIOS_H3b.md")} if self.f.nome == "voos23" else {}), "corte": self.f.corte,
                "parametros": self.f.parametros()}

    # ---------- execução ----------
    def rodar(self) -> dict:
        t0 = time.time()
        f = self.f
        ev = f.eventos(self.dados)
        espec = self.especificacao()
        k_hist = self.saida / "especificacoes.jsonl"
        hs = sha(espec)
        vistos = [json.loads(l)["hash"] for l in k_hist.read_text().splitlines()] if k_hist.exists() else []
        if hs not in vistos:
            with k_hist.open("a") as fh:
                fh.write(json.dumps({"hash": hs, "fase": self.fase, "quando": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}) + "\n")
            vistos.append(hs)
        K = len(vistos)

        c0 = self.ficha(1, "agent-f7-domain", "contrato", {**f.contrato, "tabelas": {k: list(v.columns) for k, v in ev.items()},
                                                            "corte_cofre": f.corte, "tipo": f.tipo})
        ticks = np.asarray(f.ticks(ev), dtype=np.int64)
        if self.fase == "dev":
            ticks = ticks[ticks < f.corte]
        rel = Relogio(int(ticks[0]) - 1)
        # o armazém é um cache reconstruído dos CSVs a cada execução; fica em disco local (a pasta do
        # projeto é montada em rede, e o replay lê dezenas de milhares de janelas)
        self._tmp = tempfile.TemporaryDirectory(prefix=f"armazem_{f.nome}_")
        arm = Armazem(Path(self._tmp.name), rel, f.corte)
        ingest = {}
        tabs = dict(ev)
        rotulos = f.tabela_rotulos(ev) if f.tipo == "binario" else None
        if rotulos is not None:
            tabs["canario"] = rotulos.rename(columns={"y": "c"})[["id", "c", "t_disp"]]
        for nome, df in tabs.items():
            ingest[nome] = arm.ingerir(nome, df, f.linhas_por_grupo.get(nome, 256))
        self.ficha(2, "agent-f2-data", "ingestao", {"tabelas": ingest}, [c0.hash])
        canario_verdade = dict(zip(rotulos["id"], rotulos["y"])) if (rotulos is not None and self.vazado) else None

        sensores = {nome: SensorDeriva(nome, **cfg) for nome, cfg in f.sensores.items()}
        disj = Disjuntor(**f.disjuntor)
        ultimo = {t: -INF for t in tabs}
        estado_tmax = -INF
        canario: dict = {}
        fim_calib = f.inicio + (f.corte - f.inicio) // 2
        limiares = None
        prereg = None
        contagens = {"emitidas": 0, "escopo": 0, "disjuntor": 0, "reajustes": 0, "reajustes_por_deriva": 0,
                     "rotulo_ja_conhecido": 0, "estado_do_futuro": 0}
        decisoes = {"alteradas": 0, "total": 0}

        for agora in ticks:
            agora = int(agora)
            if limiares is None and agora >= fim_calib:
                limiares = [s.fixar_limiares() for s in sensores.values()]
                self.ficha(3, "agent-f3-lifecycle", "limiares_deriva", {"sensores": limiares})
            if agora >= f.corte and prereg is None:
                if self.fase == "dev":
                    break
                dev = self._placar(ate=f.corte)
                prereg = self.ficha(3, "agent-f3-lifecycle", "preregistro", {**espec, "hash": hs, "K": K, "placar_desenvolvimento": dev})
                arm.destrancar(hs, "agent-f3-lifecycle")
                self.ficha(3, "agent-f3-lifecycle", "cofre_aberto", {"hash": hs, "t": agora}, [prereg.hash])
            rel.avancar(agora)
            # F2: só a janela nova de cada tabela
            for nome in tabs:
                cols = [c for c in tabs[nome].columns if c != "t_disp"]
                df = arm.ler(nome, cols, "agent-f2-data", ultimo[nome], agora)
                ultimo[nome] = agora
                if not len(df):
                    continue
                estado_tmax = max(estado_tmax, int(df["t_disp"].max()))
                if nome == "canario":
                    canario.update(zip(df["id"], df["c"]))
                    continue
                f.consumir(nome, df, agora)
                for snome, vals in f.sinal_deriva(nome, df).items():
                    for e in sensores[snome].alimentar(vals, agora, calibrando=agora < fim_calib):
                        self.eventos_op.append({**e, "tipo": "deriva"})
                        self.ficha(3, "agent-f3-lifecycle", "alarme_deriva", e)
                        if e["evento"] == "ligou":
                            f.ajustar(agora)
                            contagens["reajustes_por_deriva"] += 1
                            sensores[snome].rebasear()
            # resolução
            for id_, y, t_rot in f.resolver(agora):
                l = self.diario.resolver(id_, y, t_rot)
                if l is None:
                    continue
                if t_rot <= l["t_emissao"]:
                    contagens["rotulo_ja_conhecido"] += 1
                disj.alimentar(self.diario.perda(l, "modelo"), self.diario.perda(l, "reserva"))
            e = disj.verificar(agora)
            if e:
                self.eventos_op.append({**e, "tipo": "disjuntor"})
                self.ficha(6, "agent-f8-decision", "disjuntor_" + e["evento"], {**e, "politica": "política de ignorância: modelo recusado, reserva determinístico oficial, post-mortem aberto" if e["evento"] == "abriu" else "modelo volta a ser oficial: venceu o reserva em sombra"})
            if f.precisa_ajuste(agora):
                f.ajustar(agora)
                contagens["reajustes"] += 1
            # emissão (antes do início da operação o fluxo emite só para formar o próprio histórico de treino)
            emitidas = f.emitir(agora)
            if agora < f.inicio:
                continue
            for p in emitidas:
                if estado_tmax > agora:
                    contagens["estado_do_futuro"] += 1
                if not p["escopo_ok"]:
                    origem = "reserva_escopo"
                    contagens["escopo"] += 1
                elif disj.aberto:
                    origem = "reserva_disjuntor"
                    contagens["disjuntor"] += 1
                else:
                    origem = "modelo"
                cv = canario_verdade.get(p["id"], 0.5) if canario_verdade is not None else canario.get(p["id"], 0.5)
                self.diario.registrar(p["id"], agora, p["t_alvo"], estado_tmax, p["modelo"], p["reserva"], origem, cv, p.get("extra"))
                contagens["emitidas"] += 1
                if hasattr(f, "decidir"):
                    a_m, a_r = f.decidir(p["modelo"]), f.decidir(p["reserva"])
                    decisoes["total"] += 1
                    decisoes["alteradas"] += int(a_m != a_r)
        # fim do replay: resolve o que chegou até o último tick
        resumo = {
            "fluxo": f.nome, "area": f.area, "tipo": f.tipo, "fase": self.fase, "K": K, "hash_especificacao": hs,
            "preregistro": prereg.payload["hash"] if prereg else None, "corte": f.corte,
            "contagens": contagens, "decisoes_C_L": decisoes, "limiares_deriva": limiares,
            "eventos_operacao": self.eventos_op, "varredura": arm.resumo_varredura(),
            "ingestao": ingest, "segundos": round(time.time() - t0, 1),
            "canario_vazado": self.vazado,
        }
        self.diario.tabela().to_parquet(self.saida / ("diario_canario_vazado.parquet" if self.vazado else "diario.parquet"), index=False)
        (self.saida / "registro_varredura.json").write_text(json.dumps(arm.registro[-2000:], default=int))
        resumo["placar"] = {"desenvolvimento": self._placar(ate=f.corte), "cofre": self._placar(desde=f.corte)}
        self.ficha(5, "agent-m15-orchestrator", "resumo", {k: v for k, v in resumo.items() if k not in ("eventos_operacao",)})
        resumo["fichas"] = len(self.bus.fichas)
        return resumo

    def _placar(self, desde=-INF, ate=INF) -> dict:
        ls = [l for l in self.diario.linhas.values() if l["y"] is not None and desde <= l["t_emissao"] < ate]
        if not ls:
            return {"n": 0}
        out = {"n": len(ls)}
        for qual in ("modelo", "reserva"):
            out[f"perda_{qual}"] = float(np.mean([self.diario.perda(l, qual) for l in ls]))
        of = [self.diario.perda(l, "modelo" if l["origem"] == "modelo" else "reserva") for l in ls]
        out["perda_oficial"] = float(np.mean(of))
        out["habilidade_oficial_vs_reserva"] = 1 - out["perda_oficial"] / out["perda_reserva"] if out["perda_reserva"] > 0 else 0.0
        return out
