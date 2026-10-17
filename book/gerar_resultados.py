#!/usr/bin/env python3
"""Gera RESULTADOS_RUST.md: o registro dos resultados em Rust, colhidos dos proprios instrumentos.

Uso:
    python3 book/gerar_resultados.py            # escreve RESULTADOS_RUST.md
    python3 book/gerar_resultados.py --check    # confere se o arquivo esta atualizado
"""

from __future__ import annotations

import argparse
import hashlib
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


def manifesto() -> str:
    """O manifesto da edicao: cada declaracao de ponto com o hash, e as contagens da arvore."""
    linhas = [
        "Cada declaracao de ponto, com o hash do arquivo. A cadeia inteira se confere com o `sha256sum -c` do "
        "`DECLARACAO.sha256` de cada pasta, e a lista abaixo e gerada, nao transcrita.",
        "",
        "| ponto | declaracao | SHA-256 |",
        "| --- | --- | --- |",
    ]
    total = 0
    reemitidas = 0
    for pasta in sorted((RAIZ / "fundo").glob("p*")):
        for arq in sorted(pasta.glob("DECLARACAO*.md")):
            linhas.append(f"| {pasta.name} | {arq.name} | {hashlib.sha256(arq.read_bytes()).hexdigest()} |")
            total += 1
            if arq.name != "DECLARACAO.md":
                reemitidas += 1
    fonte = (RAIZ / "crates" / "stochology-book" / "src" / "main.rs").read_text(encoding="utf-8")
    instrumentos = sorted(set(re.findall(r'"(--[a-z0-9]+)"', fonte)))
    capitulos = len(list((RAIZ / "book").glob("cap*.md")))
    pontos = len(list((RAIZ / "fundo").glob("p*")))
    crates = len(list((RAIZ / "crates").glob("stochology-*")))
    livro = RAIZ / "estocologia-livro-didatico.md"
    texto_livro = livro.read_text(encoding="utf-8")
    linhas.append("")
    linhas.append(
        f"Declaracoes de ponto: {total}, das quais {reemitidas} sao declaracoes 2 ou mais (retificacao declarada, "
        f"com os dois valores) | pontos de fundo: {pontos} | instrumentos do `labs`: {len(instrumentos)} "
        f"({chr(44).join(instrumentos)}) | capitulos: {capitulos} | crates: {crates}"
    )
    linhas.append("")
    linhas.append(
        f"Edicao montada: `estocologia-livro-didatico.md` com **{len(texto_livro.splitlines())} linhas** e "
        f"**{len(re.findall(r'\w+', texto_livro))} palavras**, "
        f"sha256 `{hashlib.sha256(livro.read_bytes()).hexdigest()}`. O arquivo tem de ser o texto que "
        f"`book/gerar.py` monta, e `python3 book/gerar.py --check` cobra isso caractere por caractere."
    )
    return "\n".join(linhas) + "\n"


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
        "| BTlabs --geneticaBT | a resolucao da validacao por intervencao: o que a segunda linha muda |\n"
        "| BTlabs --escutaBT | o lacre das sete decisoes que mexem com pessoas: quem foi ouvido |\n"
        "| BTlabs --h3BT | a cadeia do H3: o que se refaz, o que se re-deriva e o que so resta declarar |\n"
        "| BTlabs --ponteBT | a tese da ponte em escala: o piloto re-derivado e o portao do teste justo |\n"
        "| BTlabs --schemaBT | o contrato da ficha: a versao e a cadeia de migracoes em arquivo |\n"
        "| BTlabs --segundaBT | a segunda opiniao dos resultados marcados P |\n"
        "| BTlabs --jevBT | a ponte Jev-LLM: os portoes da ponte e o que so fecha com os pesos |\n"
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
        "A resolucao da validacao por intervencao (genetica de levedura)",
        [str(BIN), "--genetica"],
        "O desenho da replica: 40 mutantes, um deles o mover verdadeiro (o efeito aparece nas duas linhas) e\n"
        "outro o artefato de linha (o efeito aparece numa linha so). O mesmo mundo e validado com uma linha por\n"
        "mutante e com duas, e o que se mede e quantos mutantes passam e qual e o piso do desenho em cada arranjo.",
    ))
    partes.append(bloco(
        "A cadeia do H3, re-executada ate onde a arvore deixa",
        [str(BIN), "--h3"],
        "Le o registro do ponto 14 (BTfundo/p14-h3/registro.tsvBT) com as quatro linhas do H3 e da a classe de cada\n"
        "uma: medida (o dado bruto esta na arvore), derivada (o numero esta num arquivo versionado e a conta e o\n"
        "veredito sao refeitos) e registro (o dado bruto saiu da arvore). Re-deriva o laco performativo do\n"
        "BTresultados.jsonBT do ponto 6 e mostra as provas negativas: um registro em que a razao cai dentro da banda\n"
        "declarada muda o veredito, e um registro em que o campo da razao nao bate com a conta e reprovado.",
    ))
    partes.append(bloco(
        "A ponte Jev-LLM",
        [str(BIN), "--jev"],
        "O portao dos pesos (sem o arquivo ou com o sha256 diferente, a ponte nao roda), a decodificacao\n"
        "restrita ao esquema da ficha com a abstencao declarada, a prova de determinismo na forma do G1 nos\n"
        "dois modelos de ensaio, o custo por chamada e a adequacao do espaco de hipoteses com o criterio de 30%.\n"
        "Os pesos congelados nao estao na arvore, e por isso nenhum numero da ponte e publicado.",
    ))
    partes.append(bloco(
        "A segunda opiniao dos resultados marcados P",
        [str(BIN), "--segunda"],
        "Le o registro do ponto 19 com as catorze linhas marcadas P e da a classe de cada uma: conferida (uma\n"
        "segunda implementacao refez a conta), derivada ou registro. Mede o que da: os limiares alfa/M do p03,\n"
        "o vies do laco em duas fontes independentes, a base do banco recontada em Rust, os pareceres contados e a\n"
        "reproducao do p05, onde a divergencia entre o campo em maquina e a frase publicada fica declarada.",
    ))
    partes.append(bloco(
        "O contrato da ficha: a cadeia de migracoes em arquivo",
        [str(BIN), "--schema"],
        "Le a cadeia declarada em BTcrates/stochology-schema/migracoes.tsvBT, embutida na compilacao, e imprime os dois\n"
        "elos com a nota de cada um. Depois demonstra a migracao de um documento da versao 1.0 ate a versao da ficha:\n"
        "os dois campos que a cadeia acrescenta entram com os valores declarados e o resto do documento fica intacto.",
    ))
    partes.append(bloco(
        "A tese da ponte em escala, com o teste justo por fazer",
        [str(BIN), "--ponte"],
        "Re-deriva o piloto P01 das notas do juiz cego (8 de 8 defeitos nos dois bracos, 0 de 4 alarmes falsos, 0 pares\n"
        "discordantes, empate no teto) e da o veredito do portao: enquanto faltar escolhedor de casos, analistas\n"
        "humanos, juiz humano e os 12 casos, a tese fica nao sustentada e nenhuma linha do registro pode dizer o contrario.",
    ))
    partes.append(bloco(
        "O lacre das sete decisoes que mexem com pessoas",
        [str(BIN), "--escuta"],
        "Le o registro em maquina do ponto 13 (BTfundo/p13-escuta/registro.tsvBT): as sete decisoes, quem paga o erro\n"
        "de cada uma, quem foi ouvido e a cobertura minima declarada. Da o veredito do lacre e mostra, com dois casos\n"
        "sinteticos, que o portao sabe reprovar uma decisao que se diz emitida sem ninguem ouvido e que sabe abrir a\n"
        "caixa quando a cobertura chega ao minimo.",
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
    partes.append("## 5. O manifesto da edicao\n")
    partes.append(manifesto())

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
