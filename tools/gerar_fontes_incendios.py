#!/usr/bin/env python3
"""Gera as duas fontes do item 13 a partir de um catalogo de casos de incendio.

Duas etapas desacopladas, para o pre-registro ficar de pe:

1. o catalogo de casos (casos_incendios.tsv) e montado fora daqui, com desfecho observado e fonte publica
   auditavel na linha; este script nao inventa caso, nao sorteia desfecho e nao preenche fonte;
2. a inferencia congelada roda caso a caso, com temperatura 0 e semente fixa, e a resposta e validada
   contra o esquema canonico da ficha antes de ser serializada com a quebra escapada como barra-n.

O que o catalogo tem de ter, por linha (TSV com cabecalho):

    caso_id  bioma  temp_c  umid_rel  vento_kmh  dias_sem_chuva  desfecho_observado  fonte

desfecho_observado e 0 ou 1 (1 = foco detectado na janela de 24 h, 0 = ausencia) e fonte nomeia o registro
publico de onde o desfecho saiu: a consulta ao BDQueimadas/INPE com a celula de grade e a data, por exemplo.
Sem essa coluna preenchida, a linha e recusada; procedencia vazia e afirmacao sem lastro.

Uso:

    gerar_fontes_incendios.py exemplo --saida dados_brutos/casos_exemplo.tsv
    gerar_fontes_incendios.py conferir-casos --casos dados_brutos/casos_incendios.tsv
    gerar_fontes_incendios.py inferir --casos CASOS --runner llama-cli --modelo M.gguf \
        --saida-inferencias dados_brutos/inferencias_reais.tsv \
        --saida-desfechos dados_brutos/desfechos_auditados.tsv

Runners: llama-cli (subprocesso), vllm (HTTP /v1/completions), api (HTTP /v1/chat/completions, chave em
JEV_API_KEY), arquivo (respostas ja colhidas) e demo
(ficha deterministica, so para atestar o encanamento; nunca e o modelo).
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import urllib.request
from pathlib import Path

CABECALHO_CASOS = [
    "caso_id",
    "bioma",
    "temp_c",
    "umid_rel",
    "vento_kmh",
    "dias_sem_chuva",
    "desfecho_observado",
    "fonte",
]

# A ficha e a do schema do repositorio: campo = valor, uma linha por campo. Resposta fora disso abstem na
# ponte, e por isso o prompt pede exatamente esta forma.
CAMPOS_OBRIGATORIOS = ("versao", "id", "definicao", "margem")

MODELO_DE_FICHA = """versao = 1.2
acao_do_pacote = abster quando a saida sai do esquema
artefato_simulavel = crates/stochology-core#jev.incendio.{indice}
camada = L2
clausula_de_saida = parar-se-ECE>0.10
definicao = {definicao}
deriva_declarada = true
escopo_ecologico = true
id = jev.incendio.{indice}
incerteza = epistemica
margem = {margem}
modelo_nulo = media-historica
patologias_cobertas = true
pergunta_do_decisor = o foco aparece nas proximas 24 horas?
priori_elicitada = true
quem_paga_pelo_erro = brigada-local
resolucao = R1-dia
semente = 2026
subgrupos_auditados = true
teste_de_propriedade = true
trata_ausentes = true
trilha_integra = true
unidade_independente = dia
"""


def construir_prompt(bioma: str, temp: float, umidade: int, vento: int, dias_secos: int) -> str:
    """O pedido exato: parametros observados e o esquema da ficha, nada de preambulo."""
    return (
        "Preencha a ficha canonica de calibracao com uma linha por campo, no formato 'campo = valor', "
        "sem preambulo, sem comentario e sem cercas de codigo. "
        f"Bioma: {bioma}. Temperatura: {temp} C. Umidade relativa: {umidade}%. "
        f"Vento: {vento} km/h. Dias sem precipitacao mensuravel: {dias_secos}. "
        "A pergunta do decisor e se havera foco de incendio nas proximas 24 horas. "
        "O campo margem e a probabilidade declarada, obrigatoriamente entre 0.01 e 0.99."
    )


def escapar(texto: str) -> str:
    """Uma linha por registro: a quebra interna vira a sequencia de dois caracteres barra-n."""
    return texto.replace("\r", "").replace("\n", "\\n")


def validar_ficha(texto: str) -> float:
    """Valida o esquema antes de serializar e devolve a probabilidade declarada."""
    campos: dict[str, str] = {}
    for linha in texto.replace("\\n", "\n").splitlines():
        if "=" not in linha:
            continue
        chave, valor = linha.split("=", 1)
        campos[chave.strip()] = valor.strip()
    faltando = [c for c in CAMPOS_OBRIGATORIOS if c not in campos]
    if faltando:
        raise ValueError(f"resposta fora do esquema: faltam {faltando}")
    bruto = campos["margem"]
    try:
        p = float(bruto)
    except ValueError:
        raise ValueError(f"margem nao numerica: {bruto!r}") from None
    if not (0.0 < p < 1.0):
        raise ValueError(f"extremo dogmatico: margem = {p}; a ficha exige 0 < p < 1")
    return p


def ler_casos(caminho: Path) -> list[dict[str, str]]:
    linhas = [l for l in caminho.read_text(encoding="utf-8").splitlines() if l.strip() and not l.startswith("#")]
    if not linhas:
        raise SystemExit(f"{caminho}: catalogo vazio")
    cabecalho = linhas[0].split("\t")
    if cabecalho != CABECALHO_CASOS:
        raise SystemExit(f"{caminho}: cabecalho inesperado\n  achei: {cabecalho}\n  esperado: {CABECALHO_CASOS}")
    casos = []
    for i, linha in enumerate(linhas[1:], start=2):
        campos = linha.split("\t")
        if len(campos) != len(CABECALHO_CASOS):
            raise SystemExit(f"{caminho}:{i}: {len(campos)} campos, esperado {len(CABECALHO_CASOS)}")
        caso = dict(zip(CABECALHO_CASOS, campos))
        if caso["desfecho_observado"] not in ("0", "1"):
            raise SystemExit(f"{caminho}:{i}: desfecho {caso['desfecho_observado']!r} nao e 0 nem 1")
        if not caso["fonte"].strip():
            raise SystemExit(f"{caminho}:{i}: fonte vazia; procedencia vazia nao entra")
        if not caso["caso_id"].strip():
            raise SystemExit(f"{caminho}:{i}: caso_id vazio")
        casos.append(caso)
    return casos


def runner_llama_cli(prompt: str, modelo: str, binario: str, seed: int, max_tokens: int) -> str:
    """Subprocesso do llama.cpp: temperatura 0, semente fixa, sem eco do prompt."""
    comando = [
        binario,
        "-m", modelo,
        "-p", prompt,
        "-n", str(max_tokens),
        "--temp", "0",
        "--seed", str(seed),
        "--no-display-prompt",
        "--simple-io",
    ]
    saida = subprocess.run(comando, capture_output=True, text=True, timeout=600)
    if saida.returncode != 0:
        raise RuntimeError(f"{binario} saiu com {saida.returncode}: {saida.stderr.strip()[:400]}")
    return saida.stdout.strip()


def runner_vllm(prompt: str, url: str, modelo: str, seed: int, max_tokens: int) -> str:
    """Endpoint compativel com OpenAI (vllm serve): temperatura 0 e semente fixa."""
    corpo = {
        "model": modelo,
        "prompt": prompt,
        "temperature": 0,
        "seed": seed,
        "max_tokens": max_tokens,
        "stop": ["\n\n"],
    }
    dados = json.dumps(corpo).encode("utf-8")
    pedido = urllib.request.Request(url, data=dados, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(pedido, timeout=600) as resposta:
        carga = json.loads(resposta.read().decode("utf-8"))
    return carga["choices"][0]["text"].strip()


def runner_api(
    prompt: str,
    url: str,
    modelo: str,
    chave: str,
    seed: int,
    max_tokens: int,
    temperatura: float = 0.0,
) -> str:
    """Endpoint de chat compativel com OpenAI, com chave no cabecalho.

    Serve para provedor remoto e para vllm serve. A chave vem do ambiente, nunca do repositorio.
    """
    corpo = {
        "model": modelo,
        "messages": [
            {
                "role": "system",
                "content": (
                    "Voce responde apenas com o documento da ficha, uma linha por campo no formato "
                    "'campo = valor'. Sem preambulo, sem comentario, sem cercas de codigo."
                ),
            },
            {"role": "user", "content": prompt},
        ],
        "temperature": temperatura,
        "seed": seed,
        "max_tokens": max_tokens,
    }
    dados = json.dumps(corpo).encode("utf-8")
    cabecalhos = {"Content-Type": "application/json"}
    if chave:
        cabecalhos["Authorization"] = f"Bearer {chave}"
    pedido = urllib.request.Request(url, data=dados, headers=cabecalhos)
    with urllib.request.urlopen(pedido, timeout=600) as resposta:
        carga = json.loads(resposta.read().decode("utf-8"))
    return carga["choices"][0]["message"]["content"].strip()


def runner_demo(caso: dict[str, str], indice: int) -> str:
    """Ficha deterministica a partir dos numeros do proprio caso: atesta o encanamento, nao e o modelo."""
    temp = float(caso["temp_c"])
    umidade = float(caso["umid_rel"])
    dias = float(caso["dias_sem_chuva"])
    # Regra declarada, monotona no que a fisica do fogo pede: calor e seca sobem, umidade desce.
    bruto = 0.10 + 0.75 * (temp / 45.0) - 0.40 * (umidade / 100.0) + 0.15 * min(dias, 60.0) / 60.0
    margem = min(max(bruto, 0.02), 0.98)
    definicao = (
        f"risco de foco em 24h: {caso['bioma']}, {caso['temp_c']} C, {caso['umid_rel']}% UR, "
        f"{caso['vento_kmh']} km/h, {caso['dias_sem_chuva']} dias secos"
    )
    return MODELO_DE_FICHA.format(indice=indice, definicao=definicao, margem=f"{margem:.4f}")


def ler_respostas_colhidas(caminho: Path) -> dict[str, str]:
    respostas = {}
    for linha in caminho.read_text(encoding="utf-8").splitlines():
        if not linha.strip() or linha.startswith("#"):
            continue
        caso_id, resposta = linha.split("\t", 1)
        respostas[caso_id] = resposta.replace("\\n", "\n")
    return respostas


def escrever_exemplo(saida: Path, n: int) -> None:
    """Catalogo sintetico, marcado como sintetico: serve para atestar o encanamento, nao para calibrar."""
    saida.parent.mkdir(parents=True, exist_ok=True)
    linhas = ["\t".join(CABECALHO_CASOS)]
    biomas = ["Cerrado", "Pantanal", "Amazonia", "Mata Atlantica", "Caatinga", "Pampa"]
    for i in range(1, n + 1):
        bioma = biomas[i % len(biomas)]
        temp = 18.0 + (i * 7) % 22
        umidade = 10 + (i * 13) % 80
        vento = 8 + (i * 5) % 28
        dias = (i * 11) % 60
        desfecho = 1 if (i * 13) % 5 < 3 else 0
        linhas.append(
            f"SINTETICO-INC-{i:03}\t{bioma}\t{temp:.1f}\t{umidade}\t{vento}\t{dias}\t{desfecho}\tsintetico-demo"
        )
    saida.write_text("\n".join(linhas) + "\n", encoding="utf-8")
    print(f"-> {saida} gravado com {n} casos SINTETICOS (fonte sintetico-demo). Nao e dado do INPE.")


def main() -> int:
    parser = argparse.ArgumentParser(description="Gera as fontes do item 13 a partir de um catalogo de casos.")
    sub = parser.add_subparsers(dest="comando", required=True)

    p_ex = sub.add_parser("exemplo", help="grava um catalogo sintetico, marcado como sintetico")
    p_ex.add_argument("--saida", required=True)
    p_ex.add_argument("--n", type=int, default=32)

    p_cf = sub.add_parser("conferir-casos", help="valida o catalogo sem chamar modelo nenhum")
    p_cf.add_argument("--casos", required=True)

    p_in = sub.add_parser("inferir", help="roda a inferencia congelada e grava as duas fontes")
    p_in.add_argument("--casos", required=True)
    p_in.add_argument("--runner", choices=["llama-cli", "vllm", "api", "arquivo", "demo"], required=True)
    p_in.add_argument("--saida-inferencias", required=True)
    p_in.add_argument("--saida-desfechos", required=True)
    p_in.add_argument("--modelo", default="")
    p_in.add_argument("--binario", default="llama-cli")
    p_in.add_argument("--url", default="http://127.0.0.1:8000/v1/completions")
    p_in.add_argument("--url-api", default="https://api.openai.com/v1/chat/completions")
    p_in.add_argument("--chave-env", default="JEV_API_KEY")
    p_in.add_argument("--repetir", type=int, default=1, help="repeticoes por caso, para medir determinismo")
    p_in.add_argument("--respostas", default="", help="arquivo caso_id<TAB>resposta, para o runner arquivo")
    p_in.add_argument("--seed", type=int, default=2026)
    p_in.add_argument("--max-tokens", type=int, default=400)

    args = parser.parse_args()

    if args.comando == "exemplo":
        escrever_exemplo(Path(args.saida), args.n)
        return 0

    casos = ler_casos(Path(args.casos))
    if args.comando == "conferir-casos":
        positivos = sum(1 for c in casos if c["desfecho_observado"] == "1")
        print(f"catalogo ok: {len(casos)} casos, {positivos} com desfecho 1 e {len(casos) - positivos} com 0")
        if len(casos) < 30:
            print("volume abaixo de 30: o escore nao sera publicado pelo instrumento", file=sys.stderr)
            return 1
        return 0

    colhidas = ler_respostas_colhidas(Path(args.respostas)) if args.runner == "arquivo" and args.respostas else {}
    if args.runner in ("llama-cli", "vllm", "api") and not args.modelo:
        print(f"runner {args.runner} exige --modelo", file=sys.stderr)
        return 1
    if args.runner == "arquivo" and not colhidas:
        print("runner arquivo exige --respostas com ao menos uma linha", file=sys.stderr)
        return 1

    linhas_inf: list[str] = []
    linhas_desf: list[str] = [
        "\t".join(["caso_id", "probabilidade_declarada", "desfecho_observado", "fonte_desfecho"])
    ]
    for indice, caso in enumerate(casos, start=1):
        prompt = construir_prompt(
            caso["bioma"],
            float(caso["temp_c"]),
            int(caso["umid_rel"]),
            int(caso["vento_kmh"]),
            int(caso["dias_sem_chuva"]),
        )
        if args.runner == "demo":
            resposta = runner_demo(caso, indice)
        elif args.runner == "arquivo":
            if caso["caso_id"] not in colhidas:
                raise SystemExit(f"caso {caso['caso_id']} sem resposta no arquivo colhido")
            resposta = colhidas[caso["caso_id"]]
        elif args.runner == "llama-cli":
            resposta = runner_llama_cli(prompt, args.modelo, args.binario, args.seed, args.max_tokens)
        elif args.runner == "vllm":
            resposta = runner_vllm(prompt, args.url, args.modelo, args.seed, args.max_tokens)
        else:
            chave = os.environ.get(args.chave_env, "")
            if not chave:
                print(
                    f"runner api exige a chave em {args.chave_env}; nenhuma credencial e gravada na arvore",
                    file=sys.stderr,
                )
                return 1
            resposta = runner_api(
                prompt, args.url_api, args.modelo, chave, args.seed, args.max_tokens
            )

        try:
            prob = validar_ficha(resposta)
        except ValueError as erro:
            raise SystemExit(f"{caso['caso_id']}: {erro}") from None

        linhas_inf.append(f"{escapar(prompt)}\t{escapar(resposta)}")
        linhas_desf.append(
            f"{caso['caso_id']}\t{prob:.4f}\t{caso['desfecho_observado']}\t{caso['fonte']}"
        )

    if args.repetir > 1:
        # A prova do G1 no provedor: mesma pergunta, duas chamadas, resposta byte a byte.
        if args.runner in ("demo", "arquivo"):
            print(f"  determinismo: runner {args.runner} e deterministico por construcao, nada a repetir")
        else:
            caso = casos[0]
            prompt = construir_prompt(
                caso["bioma"],
                float(caso["temp_c"]),
                int(caso["umid_rel"]),
                int(caso["vento_kmh"]),
                int(caso["dias_sem_chuva"]),
            )
            iguais = 0
            primeira = None
            for _ in range(args.repetir):
                if args.runner == "llama-cli":
                    r = runner_llama_cli(prompt, args.modelo, args.binario, args.seed, args.max_tokens)
                elif args.runner == "vllm":
                    r = runner_vllm(prompt, args.url, args.modelo, args.seed, args.max_tokens)
                else:
                    r = runner_api(
                        prompt,
                        args.url_api,
                        args.modelo,
                        os.environ.get(args.chave_env, ""),
                        args.seed,
                        args.max_tokens,
                    )
                if primeira is None:
                    primeira = r
                elif r == primeira:
                    iguais += 1
            print(
                f"  determinismo: {iguais} de {args.repetir - 1} repeticoes identicas a primeira"
                + (" | fecha o G1" if iguais == args.repetir - 1 else " | NAO fecha o G1")
            )

    saida_inf = Path(args.saida_inferencias)
    saida_desf = Path(args.saida_desfechos)
    saida_inf.parent.mkdir(parents=True, exist_ok=True)
    saida_desf.parent.mkdir(parents=True, exist_ok=True)
    saida_inf.write_text("\n".join(linhas_inf) + "\n", encoding="utf-8")
    saida_desf.write_text("\n".join(linhas_desf) + "\n", encoding="utf-8")
    print(f"-> {saida_inf} ({len(linhas_inf)} inferencias)")
    print(f"-> {saida_desf} ({len(linhas_desf) - 1} pares rotulados)")
    print("proximo passo: cargo run --release --bin ingestar-jev -- --conferir <inf> <desf>")
    return 0


if __name__ == "__main__":
    sys.exit(main())
