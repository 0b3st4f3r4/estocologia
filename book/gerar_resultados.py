#!/usr/bin/env python3
"""Gera RESULTADOS_RUST.md: o registro dos resultados em Rust, colhidos dos proprios instrumentos.

Uso:
    python3 book/gerar_resultados.py            # escreve RESULTADOS_RUST.md
    python3 book/gerar_resultados.py --check    # confere se o arquivo esta atualizado
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

RAIZ = Path(__file__).resolve().parent.parent
DESTINO = RAIZ / "RESULTADOS_RUST.md"
BIN = RAIZ / "target" / "release" / "labs"
PORTAS = RAIZ / "target" / "release" / "portoes"


def normalizar(texto: str) -> str:
    """Tira o que muda entre execucoes (tempo de teste, linhas do cargo) para o registro ser conferivel."""
    texto = re.sub(r"finished in [0-9.]+s", "finished in Xs", texto)
    for padrao in (r"^\s*Compiling .*$", r"^\s*Finished .*$", r"^\s*Running .*$", r"^\s*Doc-tests .*$"):
        texto = re.sub(padrao, "", texto, flags=re.M)
    # a ordem dos testes muda entre execucoes: ordena cada bloco contiguo de linhas de teste
    linhas = texto.splitlines()
    saida: list[str] = []
    bloco: list[str] = []
    for linha in linhas:
        if linha.startswith("test "):
            bloco.append(linha)
            continue
        if bloco:
            saida.extend(sorted(bloco))
            bloco = []
        saida.append(linha)
    if bloco:
        saida.extend(sorted(bloco))
    texto = "\n".join(saida)
    return re.sub(r"\n{3,}", "\n\n", texto).strip()


def rodar(comando: list[str]) -> str:
    proc = subprocess.run(comando, capture_output=True, text=True, cwd=RAIZ)
    saida = normalizar((proc.stdout + proc.stderr).rstrip())
    return saida if saida else "(sem saida)"


def bloco(titulo: str, comando: list[str], nota: str) -> str:
    texto = rodar(comando)
    cmd = " ".join(comando)
    return f"### {titulo}\n\n{nota}\n\n~~~bash\n{cmd}\n~~~\n\n~~~text\n{texto}\n~~~\n"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()

    if not BIN.exists() or not PORTAS.exists():
        print("instrumentos ausentes: rode cargo build --release primeiro")
        return 1

    partes = [
        "# Resultados em Rust\n",
        "Registro colhido direto dos instrumentos, em 2026-10-16. Nada aqui e transcricao: cada bloco e a\n"
        "saida literal do comando acima dele. Quem duvidar de um numero roda o comando e compara.\n",
    ]
    partes.append(
        "| Instrumento | O que ele mede |\n| --- | --- |\n"
        "| BTlabs --cap NBT | o laboratorio do capitulo N do livro, com semente fixa |\n"
        "| BTlabs --todosBT | os 21 laboratorios em sequencia |\n"
        "| BTlabs --bancoBT | o H0 real: a base de 2,65 milhoes de chamadas, escala de Erlang C, unidade independente e nulo de metades |\n"
        "| BTlabs --resolucaoBT | o horizonte R: quanto custa perguntar grosso, com o mesmo recurso total |\n"
        "| BTlabs --s1BT | a triagem deterministica do Sistema 1 |\n"
        "| BTlabs --s2BT | o Sistema 2: a busca sobre o hipergrafo, a crenca da banca, a inferencia e o eixo R |\n"
        "| BTlabs --pretesteBT | o nulo do pre-teste de calibracao do painel SHELF |\n"
        "| BTlabs --filaBT | a fila com desistencia e as quatro regras de escala (R0 a R3) |\n"
        "| BTportoesBT | os sete portoes deterministicos e a banca adversarial A1-A8 |\n"
        "| BTcargo test --workspaceBT | os testes dos quinze crates |\n"
    )
    partes.append("## 1. O H0 real, em Rust\n")
    partes.append(bloco(
        "A central de chamadas do banco (Capitulo 16)",
        [str(BIN), "--banco"],
        "Le BTdados-reais/bank_calls.csvBT (a base publica de 5 em 5 minutos), monta a escala de Erlang C sobre o\n"
        "perfil medio do treino e mede o que a mesma escala faz nos dias do cofre. Declaracao de divergencia contra\n"
        "o README do H0 real: a BTVar/mediaBT de 273 que o README publica sai de 272,6 no treino (este instrumento\n"
        "mede 246,7 no cofre e 261,1 na serie toda); o nulo de metades por dia e 0,0441 contra 0,0474 (esquema de\n"
        "reparticao diferente); e o nulo por chamada e 0,0006 pela binomial, contra os 0,0033 publicados, que\n"
        "usavam outro desenho de correlacao intradiaria.",
    ))
    partes.append(bloco(
        "A fila com desistencia e as quatro regras de escala (R0 a R3)",
        [str(BIN), "--fila"],
        "Le a base do banco, reconstroi a fila com desistencia por dois motores independentes (o exato sobre as\n"
        "chegadas observadas e o de eventos com chegadas de Poisson) e mede as quatro regras de escala. Os\n"
        "agente-horas batem com o registro; os niveis e as falhas por dia divergem para baixo e para cima, e a\n"
        "divergencia fica declarada no Capitulo 16.",
    ))
    partes.append(bloco(
        "O Sistema 1: triagem deterministica de fichas",
        [str(BIN), "--s1"],
        "Vocabulario fechado de 6 categorias e 35 temas, arvore de sufixos de 136 entradas, probabilidade presa em\n"
        "[0,01; 0,99] e soma conferida a cada rodada. Sem casamento, a resposta e a priori declarada de outros.",
    ))
    partes.append(bloco(
        "O pre-teste de calibracao do painel SHELF",
        [str(BIN), "--preteste"],
        "O nulo declarado: um especialista perfeitamente calibrado, com a priori uniforme, no desenho do\n"
        "protocolo. Mede o que o limiar de ECE de 0,10 aprova e mostra os dois limites do desenho (o criterio 3\n"
        "vacuoso com 20 itens em 10 caixas e o criterio 4 inerte como escrito).",
    ))
    partes.append(bloco(
        "O Sistema 2 deliberativo",
        [str(BIN), "--s2"],
        "LATS contra o ganancioso no grafo declarado da amostra, a crenca da banca medida em ataques ate o\n"
        "primeiro reprovado, a posterior da Beta(4, 8) contra a forma fechada (1/3) e a resolucao escolhida\n"
        "pelo proprio instrumento do eixo R. O no que nenhuma lente cobre e a frente declarada e nao\n"
        "implementada (f1.sintese-de-codigo).",
    ))
    partes.append(bloco(
        "O horizonte R: a resolucao da pergunta",
        [str(BIN), "--resolucao"],
        "Bracos pareados, mesmo recurso total. A perda de resolucao mede quanto custa perguntar pelo dia medio em\n"
        "vez de pelo dia que existe.",
    ))
    partes.append("## 2. Os 21 laboratorios do livro\n")
    partes.append(bloco("Todos os laboratorios", [str(BIN), "--todos"], "Uma execucao, semente fixa, sem rede."))
    partes.append("## 3. Os portoes deterministicos\n")
    partes.append(bloco(
        "Sete portoes e a banca adversarial",
        [str(PORTAS), "--horizonte", "todos", "--reproducao"],
        "Sete portoes (G1 a G7), reproducao identica e raiz de Merkle da trilha. O portao G6 e a banca A1-A8.",
    ))
    partes.append(bloco(
        "Prova negativa: o portao sabe reprovar",
        [str(PORTAS), "--horizonte", "h3", "--defeito", "h3.voos23"],
        "Com um defeito plantado, o G6 reprova e o binario sai com codigo 1.",
    ))
    partes.append("## 4. Os testes\n")
    partes.append(bloco("Os quinze crates", ["cargo", "test", "--workspace"], "Cada crate com o seu bloco de testes."))

    texto = "\n".join(partes).replace("BT", chr(96))
    if args.check:
        atual = DESTINO.read_text(encoding="utf-8") if DESTINO.exists() else ""
        if atual.strip() != texto.strip():
            print("RESULTADOS_RUST.md desatualizado: rode python3 book/gerar_resultados.py")
            return 1
        print("RESULTADOS_RUST.md atualizado")
        return 0

    DESTINO.write_text(texto, encoding="utf-8")
    print(f"escrito {DESTINO.name}: {len(texto.splitlines())} linhas")
    return 0


if __name__ == "__main__":
    sys.exit(main())
