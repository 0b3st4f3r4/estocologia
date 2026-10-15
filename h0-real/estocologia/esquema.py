"""Contrato mínimo do hipergrafo (AGENTS.md, seção II): valida nós, arestas e o cabeçalho da ficha."""

from __future__ import annotations

from pathlib import Path

CAMADAS = {"L1", "L2", "L3", "raiz", "core"}
INCERTEZAS = {"Aleatoria", "Epistemica", "Knightiana_Especificacao", "nao_se_aplica"}
CAMPOS_NO = ("id", "camada", "tipo_incerteza", "definicao_formal", "artefato_simulavel")
TIPOS_ARESTA = {"tree", "ponte", "causal", "patologia"}
LENTES = {"Fluxonomia", "Crononomia", "Escalonomia", "Morfonomia", "Crisonomia"}
CABECALHO = ("processo_gerador", "nao_linearidade", "unidade_e_margem", "fisica_do_ruido", "escada_causal")


def validar(ficha: dict, raiz: Path) -> list[str]:
    """Devolve a lista de violações (vazia se a ficha cumpre o contrato)."""
    erros: list[str] = []
    for k in CABECALHO:
        if not str(ficha.get("cabecalho", {}).get(k, "")).strip():
            erros.append(f"cabeçalho sem resposta: {k}")
    ids = set()
    for no in ficha.get("nos", []):
        for c in CAMPOS_NO:
            if c not in no:
                erros.append(f"nó {no.get('id', '?')} sem campo {c}")
        if no.get("camada") not in CAMADAS:
            erros.append(f"nó {no.get('id')}: camada inválida {no.get('camada')}")
        if no.get("tipo_incerteza") not in INCERTEZAS:
            erros.append(f"nó {no.get('id')}: tipo_incerteza inválido {no.get('tipo_incerteza')}")
        art = no.get("artefato_simulavel")
        if art and not (raiz / art.split("::")[0]).exists():
            erros.append(f"nó {no.get('id')}: artefato inexistente {art}")
        if no.get("id") in ids:
            erros.append(f"id repetido: {no.get('id')}")
        ids.add(no.get("id"))
    for a in ficha.get("arestas", []):
        if a.get("tipo") not in TIPOS_ARESTA:
            erros.append(f"aresta {a}: tipo inválido")
        for ponta in (a.get("de"), a.get("para")):
            if ponta not in ids:
                erros.append(f"aresta aponta para nó inexistente: {ponta}")
    for h in ficha.get("hiperarestas", []):
        if h.get("lente") not in LENTES:
            erros.append(f"hiperaresta com lente inválida: {h.get('lente')}")
        for n in h.get("nos", []):
            if n not in ids:
                erros.append(f"hiperaresta {h.get('lente')} cita nó inexistente: {n}")
    return erros
