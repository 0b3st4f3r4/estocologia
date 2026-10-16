#!/usr/bin/env python3
"""Confere uma passada de revisao contra a edicao montada de antes.

Uso:
    python3 estilo/conferir-passada.py                  # todos os arquivos do livro
    python3 estilo/conferir-passada.py book/cap07.md

O baseline e a edicao montada tal como esta no Git (`estocologia-livro-didatico.md`
no commit HEAD), que guarda o texto de antes. Para cada arquivo o script compara:

  1. o multissed de numeros: nenhum numero entra, sai ou muda de valor;
  2. as cercas de codigo: cada bloco do texto de antes continua no texto de depois;
  3. os blocos ```text: identicos, porque conferir_labs.py os compara linha a linha;
  4. os marcadores da rampa e o piso de 700 palavras;
  5. a contagem de cada maneirismo, antes e depois, com o alvo da folha de estilo.

Sai com 1 quando acha regressao. A contagem de prosa serve ao indice; o portao e o
resto.
"""

from __future__ import annotations

import re
import subprocess
from collections import Counter
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))
from varredura import PADROES, prosa  # noqa: E402

ALVOS = {
    "travessao_apostivo": 0.20,
    "travessao_fecha_com_e": 0.25,
    "mesmo_mesma": 0.30,
    "o_mesmo": 0.20,
    "o_que_relativo": 0.30,
    "virgula_nao": 0.25,
    "nao_e_X_e_Y": 0.50,
    "dois_pontos": 0.60,
    "proprio": 0.40,
    "exatamente": 0.40,
}

CERCA = re.compile(r"(?:\x60\x60\x60|~~~)([a-zA-Z]*)\n(.*?)(?:\x60\x60\x60|~~~)", re.DOTALL)
NUMERO = re.compile(r"\d+(?:[.,]\d+)*")
CABECALHO = re.compile(r"(?m)^## Capítulo (\d+)\.")
MARCADORES = ["### Degrau 1", "### Degrau 2", "### Degrau 3", "**Exercícios.**", "**Resumo.**"]


def baseline() -> str:
    import os
    if os.environ.get("ESTILO_BASELINE"):
        return Path(os.environ["ESTILO_BASELINE"]).read_text(encoding="utf-8")
    return subprocess.run(
        ["git", "show", "HEAD:estocologia-livro-didatico.md"],
        cwd=RAIZ, capture_output=True, text=True, check=True,
    ).stdout


def secoes(texto: str) -> dict[str, str]:
    """Divide a edicao montada em abertura, capNN e apendices."""
    partes = CABECALHO.split(texto)
    numeros = [int(x) for x in partes[1::2]]
    corpo: dict[str, str] = {"abertura": partes[0]}
    for i, num in enumerate(numeros):
        corpo[f"cap{num:02d}"] = partes[2 * i + 2]
    if numeros and "# Apêndices" in partes[-1]:
        corpo[f"cap{numeros[-1]:02d}"], resto = partes[-1].split("# Apêndices", 1)
        corpo["apendices"] = resto
    return corpo


def secao_do_arquivo(caminho: Path) -> str:
    """Devolve o trecho comparavel: o arquivo, ou a partir do cabecalho do capitulo."""
    texto = caminho.read_text(encoding="utf-8")
    if caminho.name == "apendices.md":
        return texto.split("# Apêndices", 1)[1]
    m = CABECALHO.search(texto)
    return texto[m.end():] if m else texto


def blocos(texto: str) -> list[tuple[str, str]]:
    return [(lang, corpo.strip("\n")) for lang, corpo in CERCA.findall(texto)]


def main() -> int:
    arquivos = [Path(a) for a in sys.argv[1:]] or sorted((RAIZ / "book").glob("*.md"))
    base_secoes = secoes(baseline())

    falhas = 0
    for caminho in arquivos:
        nome = caminho.name
        if nome.startswith("00-"):
            chave = "abertura"
        elif nome == "apendices.md":
            chave = "apendices"
        elif nome.startswith("cap") and nome[3:5].isdigit():
            chave = nome[:5]
        else:
            print(f"{nome}: fora da edicao montada — pulado")
            continue
        antes_txt = base_secoes.get(chave)
        if antes_txt is None:
            print(f"{nome}: sem baseline em {chave!r} — pulado")
            continue
        depois_txt = secao_do_arquivo(caminho)
        linhas_antes = [l for l in antes_txt.splitlines() if not l.startswith("|")]
        linhas_depois = [l for l in depois_txt.splitlines() if not l.startswith("|")]

        problemas: list[str] = []

        n_antes = NUMERO.findall("\n".join(linhas_antes))
        n_depois = NUMERO.findall("\n".join(linhas_depois))
        ca, cd = Counter(n_antes), Counter(n_depois)
        delta = {k: (ca[k], cd[k]) for k in set(ca) | set(cd) if ca[k] != cd[k]}
        if delta:
            detalhe = ", ".join(f"{k!r}: {a}->{d}" for k, (a, d) in sorted(delta.items())[:8])
            onde = ""
            for chave_num in delta:
                achado = next((l.strip() for l in linhas_depois if chave_num in l), "")
                if achado:
                    onde = f" | ex.: {achado[:110]}"
                    break
            problemas.append(f"numeros mudaram: {detalhe}{onde}")

        b_antes = blocos(antes_txt)
        b_depois = blocos(depois_txt)
        if len(b_antes) != len(b_depois):
            problemas.append(f"cercas mudaram de numero: antes {len(b_antes)} depois {len(b_depois)}")
        for lang, corpo in b_antes:
            if (lang, corpo) not in b_depois:
                problemas.append(f"bloco de cerca {lang!r} com {len(corpo.splitlines())} linhas nao esta igual")

        if chave.startswith("cap"):
            for marcador in MARCADORES:
                if marcador not in depois_txt:
                    problemas.append(f"marcador ausente: {marcador}")

        palavras = len(re.findall(r"\w+", prosa(depois_txt)))
        if palavras < 700:
            problemas.append(f"piso de palavras: {palavras}")

        p_antes = prosa(antes_txt)
        p_depois = prosa(depois_txt)
        campos = []
        for padrao, alvo in ALVOS.items():
            a = len(re.findall(PADROES[padrao], p_antes, re.I))
            d = len(re.findall(PADROES[padrao], p_depois, re.I))
            limites = ""
            # Os tetos de capitulo contam so a prosa corrida: celula de tabela e cabecalho
            # carregam o padrao mas nao entram na reescrita.
            if padrao in ("virgula_nao", "travessao_fecha_com_e"):
                corrida = [l for l in prosa(depois_txt).splitlines() if not l.startswith(("|", "#"))]
                d_corrida = len(re.findall(PADROES[padrao], "\n".join(corrida), re.I))
                if d_corrida > 2:
                    problemas.append(f"{padrao} acima do teto do capitulo, em prosa corrida: {d_corrida}")
            limite = int(a * alvo)
            limites = "" if d <= limite else "!"
            campos.append(f"{padrao} {a}->{d}{limites}")
        print(f"{nome}: " + " | ".join(campos))
        if problemas:
            falhas += 1
            for p in problemas:
                print(f"   REGRESSAO: {p}")
    print(f"arquivos conferidos: {len(arquivos)}, com regressao: {falhas}")
    return 1 if falhas else 0


if __name__ == "__main__":
    sys.exit(main())
