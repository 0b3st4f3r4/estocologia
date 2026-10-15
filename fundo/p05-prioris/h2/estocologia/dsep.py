"""d-separação e critério da porta dos fundos (M13), para o Filtro 2 do funil.

O DAG vem do agent-f7-domain (literatura e especialistas), não dos dados: é uma premissa declarada,
e o que se verifica aqui é se o conjunto de ajuste proposto é coerente com ela.
"""

from __future__ import annotations


def _pais(arestas):
    p = {}
    for a, b in arestas:
        p.setdefault(b, set()).add(a)
        p.setdefault(a, set())
    return p


def _filhos(arestas):
    f = {}
    for a, b in arestas:
        f.setdefault(a, set()).add(b)
        f.setdefault(b, set())
    return f


def descendentes(arestas, x) -> set:
    f, vist, pilha = _filhos(arestas), set(), [x]
    while pilha:
        v = pilha.pop()
        for w in f.get(v, ()):
            if w not in vist:
                vist.add(w)
                pilha.append(w)
    return vist


def d_separados(arestas, x, y, Z) -> bool:
    """x ⫫ y | Z no DAG? Algoritmo de alcançabilidade (Koller & Friedman 2009, alg. 3.1)."""
    Z = set(Z)
    pais, filhos = _pais(arestas), _filhos(arestas)
    for v in (x, y, *Z):
        pais.setdefault(v, set())
        filhos.setdefault(v, set())
    # ancestrais de Z (um colisor se abre se ele ou um descendente está em Z)
    anc, pilha = set(), list(Z)
    while pilha:
        v = pilha.pop()
        if v not in anc:
            anc.add(v)
            pilha.extend(pais[v])
    visitados, fila = set(), [(x, "cima")]
    while fila:
        v, dirc = fila.pop()
        if (v, dirc) in visitados:
            continue
        visitados.add((v, dirc))
        if v not in Z and v == y:
            return False
        if dirc == "cima" and v not in Z:
            fila += [(p, "cima") for p in pais[v]] + [(c, "baixo") for c in filhos[v]]
        elif dirc == "baixo":
            if v not in Z:
                fila += [(c, "baixo") for c in filhos[v]]
            if v in anc:
                fila += [(p, "cima") for p in pais[v]]
    return True


def porta_dos_fundos(arestas, x, y, Z) -> dict:
    """Z satisfaz o critério da porta dos fundos para o efeito de x em y?"""
    desc = descendentes(arestas, x)
    ruins = sorted(set(Z) & desc)
    sem_saida = [(a, b) for a, b in arestas if a != x]
    sep = d_separados(sem_saida, x, y, Z)
    return {"valido": not ruins and sep, "descendentes_no_ajuste": ruins, "d_separa_sem_arestas_de_x": sep}
