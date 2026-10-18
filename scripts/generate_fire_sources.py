#!/usr/bin/env python3
"""Build the two item-13 sources from a catalog of fire cases.

Two decoupled stages, so that pre-registration stands:

1. the case catalog (casos_incendios.tsv) is assembled outside this script, with the observed outcome and an
   auditable public source on the row; this script invents no case, draws no outcome, and fills no source;
2. the frozen inference runs case by case, at temperature 0 and fixed seed, and the answer is validated
   against the canonical sheet schema before being serialized with the line break escaped as barra-n.

What the catalog must carry, per row (TSV with a header):

    caso_id  bioma  temp_c  umid_rel  vento_kmh  dias_sem_chuva  desfecho_observado  fonte

desfecho_observado is 0 or 1 (1 = hotspot detected inside the 24 h window, 0 = absence) and fonte names the public
record the outcome came from: the BDQueimadas/INPE query with grid cell and date, for instance. Without that
column filled, the row is refused; empty provenance is a claim without backing.

Usage:

    generate_fire_sources.py example --output dados_brutos/casos_exemplo.tsv
    generate_fire_sources.py check-cases --cases dados_brutos/casos_incendios.tsv
    generate_fire_sources.py infer --cases CASES --runner llama-cli --model M.gguf \
        --inferences-output dados_brutos/inferencias_reais.tsv \
        --outcomes-output dados_brutos/desfechos_auditados.tsv

Runners: llama-cli (subprocess), vllm (HTTP /v1/completions), api (HTTP /v1/chat/completions, key in
JEV_API_KEY), file (answers already collected) and demo
(deterministic sheet, only to attest the plumbing; it is never the model).

Every subcommand and every option keeps the Portuguese spelling it had as an alias (exemplo, conferir-casos,
inferir, arquivo, --saida, --casos, --modelo, --binario, --url-api, --chave-env, --repetir, --duas-etapas,
--gramatica, --respostas, --saida-inferencias, --saida-desfechos), so a command line written before the
translation still runs unchanged.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.request
from pathlib import Path

# Column names of the catalog and of the labeled-outcome source: this is the wire format that the Rust ramp
# reads, so the identifiers stay exactly as they are.
CASE_HEADER = [
    "caso_id",
    "bioma",
    "temp_c",
    "umid_rel",
    "vento_kmh",
    "dias_sem_chuva",
    "desfecho_observado",
    "fonte",
]

# The sheet is the repository schema: field = value, one line per field. A response outside that abstains at the
# bridge, and that is why the prompt asks for exactly this form.
REQUIRED_FIELDS = ("versao", "id", "definicao", "margem")

SHEET_TEMPLATE = """versao = 1.2
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


SHEET_EXAMPLE = """versao = 1.2
acao_do_pacote = abster e pedir vistoria
artefato_simulavel = crates/stochology-core#jev.incendio.1
camada = L2
clausula_de_saida = parar-se-ECE>0.10
definicao = caso de exemplo: Cerrado com 30 C, 40% de umidade, 12 km/h de vento e 10 dias secos
deriva_declarada = true
escopo_ecologico = true
id = jev.incendio.1
incerteza = epistemica
margem = 0.42
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
unidade_independente = dia"""


# Portuguese subcommand and runner spellings accepted as aliases of the canonical English ones.
COMMAND_ALIASES = {"exemplo": "example", "conferir-casos": "check-cases", "inferir": "infer"}
RUNNER_ALIASES = {"arquivo": "file"}


def build_prompt(biome: str, temp: float, humidity: int, wind: int, dry_days: int) -> str:
    """The exact request: one whole filled document, and the case to fill in the same form.

    The example is complete on purpose. A small model copies form, not abstract instruction: with empty or
    marked fields it invents; with a whole document in front of it, it swaps the values and keeps the names.
    No number from the example is reused — the margin that counts is the one it declares for the case.
    """
    return (
        "Voce preenche a ficha canonica de calibracao. Responda com o documento e nada mais: "
        "uma linha por campo, no formato 'campo = valor', sem preambulo, sem comentario, sem cercas de codigo.\n\n"
        f"{SHEET_EXAMPLE}\n\n"
        "Agora responda para este caso com o DOCUMENTO COMPLETO: as 24 linhas, na mesma ordem do exemplo, "
        "incluindo os campos que nao dependem do caso. Troque so os valores de id, definicao, acao_do_pacote "
        "e margem; repita os outros como estao.\n"
        f"Bioma: {biome}. Temperatura: {temp} C. Umidade relativa: {humidity}%. "
        f"Vento: {wind} km/h. Dias sem precipitacao mensuravel: {dry_days}.\n"
        "margem e a sua probabilidade declarada de foco nas proximas 24 horas, entre 0.01 e 0.99."
    )


def build_number_prompt(biome: str, temp: float, humidity: int, wind: int, dry_days: int) -> str:
    """Stage 1 of the two-stage protocol: the probability alone, so the model does not copy the example's."""
    return (
        "Responda apenas com um numero entre 0.0100 e 0.9900, com quatro casas decimais, representando a "
        "probabilidade de haver foco de incendio nas proximas 24 horas. Nada de texto antes ou depois. "
        f"Bioma: {biome}. Temperatura: {temp} C. Umidade relativa: {humidity}%. "
        f"Vento: {wind} km/h. Dias sem precipitacao mensuravel: {dry_days}."
    )


def build_document_prompt(
    biome: str, temp: float, humidity: int, wind: int, dry_days: int, margin: float, index: int
) -> str:
    """Stage 2: the complete document, with the stage-1 margin already inside, so the model fills only the rest."""
    example = SHEET_EXAMPLE.replace("margem = 0.42", f"margem = {margin:.4f}").replace(
        "id = jev.incendio.1", f"id = jev.incendio.{index}"
    )
    return (
        "Responda com o DOCUMENTO COMPLETO, as 24 linhas na mesma ordem, sem preambulo, sem comentario e sem "
        "cercas de codigo. Troque so definicao e acao_do_pacote; repita todos os outros campos exatamente "
        "como estao.\n\n"
        f"{example}\n\n"
        "Caso a descrever: "
        f"bioma {biome}, temperatura {temp} C, umidade relativa {humidity}%, vento {wind} km/h, "
        f"{dry_days} dias sem precipitacao mensuravel."
    )


def extract_number(text: str) -> float:
    """Read the stage-1 probability and refuse the dogmatic extreme."""
    found = re.findall(r"0?[.,][0-9]+|1[.,]0+|0", text)
    for raw in found:
        try:
            p = float(raw.replace(",", "."))
        except ValueError:
            continue
        if 0.0 < p < 1.0:
            return p
    raise ValueError(f"stage 1 returned no probability in (0, 1): {text[:120]!r}")


def escape_newlines(text: str) -> str:
    """One line per record: the inner break becomes the two-character sequence backslash-n."""
    return text.replace("\r", "").replace("\n", "\\n")


def validate_sheet(text: str) -> float:
    """Validate the schema before serializing and return the declared probability."""
    fields: dict[str, str] = {}
    lines = text.replace("\\n", "\n").splitlines()
    # A small model sometimes opens with a sentence before the document. The parser starts at the first canonical
    # field and ignores whatever comes before; whatever comes after is also ignored when it lacks the field = value form.
    if any(l.strip().startswith("versao =") for l in lines):
        lines = lines[next(i for i, l in enumerate(lines) if l.strip().startswith("versao =")):]
    for line in lines:
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        fields[key.strip()] = value.strip()
    missing = [f for f in REQUIRED_FIELDS if f not in fields]
    if missing:
        raise ValueError(f"response outside the schema: missing {missing}")
    raw = fields["margem"]
    try:
        p = float(raw)
    except ValueError:
        raise ValueError(f"non-numeric margem: {raw!r}") from None
    if not (0.0 < p < 1.0):
        raise ValueError(f"dogmatic extreme: margem = {p}; the sheet requires 0 < p < 1")
    return p


def read_cases(path: Path) -> list[dict[str, str]]:
    lines = [l for l in path.read_text(encoding="utf-8").splitlines() if l.strip() and not l.startswith("#")]
    if not lines:
        raise SystemExit(f"{path}: empty catalog")
    header = lines[0].split("\t")
    if header != CASE_HEADER:
        raise SystemExit(f"{path}: unexpected header\n  found: {header}\n  expected: {CASE_HEADER}")
    cases = []
    for i, line in enumerate(lines[1:], start=2):
        columns = line.split("\t")
        if len(columns) != len(CASE_HEADER):
            raise SystemExit(f"{path}:{i}: {len(columns)} fields, expected {len(CASE_HEADER)}")
        case = dict(zip(CASE_HEADER, columns))
        if case["desfecho_observado"] not in ("0", "1"):
            raise SystemExit(f"{path}:{i}: outcome {case['desfecho_observado']!r} is neither 0 nor 1")
        if not case["fonte"].strip():
            raise SystemExit(f"{path}:{i}: empty fonte; empty provenance does not enter")
        if not case["caso_id"].strip():
            raise SystemExit(f"{path}:{i}: empty caso_id")
        cases.append(case)
    return cases


def runner_llama_cli(
    prompt: str, model: str, binary: str, seed: int, max_tokens: int, grammar: str = ""
) -> str:
    """llama.cpp subprocess: temperature 0, fixed seed, no prompt echo."""
    command = [
        binary,
        "-m", model,
        "-p", prompt,
        "-n", str(max_tokens),
        "--temp", "0",
        "--seed", str(seed),
        "--no-display-prompt",
        "--simple-io",
    ]
    if grammar:
        command += ["--grammar-file", grammar]
    result = subprocess.run(command, capture_output=True, text=True, timeout=600)
    if result.returncode != 0:
        raise RuntimeError(f"{binary} exited with {result.returncode}: {result.stderr.strip()[:400]}")
    return result.stdout.strip()


def runner_vllm(
    prompt: str, url: str, model: str, seed: int, max_tokens: int, grammar: str = ""
) -> str:
    """OpenAI-compatible endpoint (vllm serve): temperature 0 and fixed seed."""
    payload = {
        "model": model,
        "prompt": prompt,
        "temperature": 0,
        "seed": seed,
        "max_tokens": max_tokens,
    }
    if grammar:
        payload["grammar"] = grammar
    data = json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=600) as answer:
        loaded = json.loads(answer.read().decode("utf-8"))
    return loaded["choices"][0]["text"].strip()


def runner_api(
    prompt: str,
    url: str,
    model: str,
    key: str,
    seed: int,
    max_tokens: int,
    temperature: float = 0.0,
    grammar: str = "",
) -> str:
    """OpenAI-compatible chat endpoint, with the key in the header.

    Serves a remote provider and vllm serve alike. The key comes from the environment, never from the repository.
    """
    payload = {
        "model": model,
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
        "temperature": temperature,
        "seed": seed,
        "max_tokens": max_tokens,
    }
    if grammar:
        payload["grammar"] = grammar
    data = json.dumps(payload).encode("utf-8")
    headers = {"Content-Type": "application/json"}
    if key:
        headers["Authorization"] = f"Bearer {key}"
    request = urllib.request.Request(url, data=data, headers=headers)
    with urllib.request.urlopen(request, timeout=600) as answer:
        loaded = json.loads(answer.read().decode("utf-8"))
    return loaded["choices"][0]["message"]["content"].strip()


def runner_demo(case: dict[str, str], index: int) -> str:
    """Deterministic sheet from the case's own numbers: it attests the plumbing, it is not the model."""
    temp = float(case["temp_c"])
    humidity = float(case["umid_rel"])
    days = float(case["dias_sem_chuva"])
    # Declared rule, monotone in what fire physics asks for: heat and drought rise, humidity falls.
    raw = 0.10 + 0.75 * (temp / 45.0) - 0.40 * (humidity / 100.0) + 0.15 * min(days, 60.0) / 60.0
    margin = min(max(raw, 0.02), 0.98)
    definition = (
        f"risco de foco em 24h: {case['bioma']}, {case['temp_c']} C, {case['umid_rel']}% UR, "
        f"{case['vento_kmh']} km/h, {case['dias_sem_chuva']} dias secos"
    )
    return SHEET_TEMPLATE.format(indice=index, definicao=definition, margem=f"{margin:.4f}")


def read_collected_answers(path: Path) -> dict[str, str]:
    answers = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        case_id, answer = line.split("\t", 1)
        answers[case_id] = answer.replace("\\n", "\n")
    return answers


def write_example(output: Path, n: int) -> None:
    """Synthetic catalog, labeled as synthetic: it attests the plumbing, it does not calibrate."""
    output.parent.mkdir(parents=True, exist_ok=True)
    lines = ["\t".join(CASE_HEADER)]
    biomes = ["Cerrado", "Pantanal", "Amazonia", "Mata Atlantica", "Caatinga", "Pampa"]
    for i in range(1, n + 1):
        biome = biomes[i % len(biomes)]
        temp = 18.0 + (i * 7) % 22
        humidity = 10 + (i * 13) % 80
        wind = 8 + (i * 5) % 28
        days = (i * 11) % 60
        outcome = 1 if (i * 13) % 5 < 3 else 0
        lines.append(
            f"SINTETICO-INC-{i:03}\t{biome}\t{temp:.1f}\t{humidity}\t{wind}\t{days}\t{outcome}\tsintetico-demo"
        )
    output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"-> {output} written with {n} SYNTHETIC cases (source sintetico-demo). Not INPE data.")


def main() -> int:
    parser = argparse.ArgumentParser(description="Build the item-13 sources from a case catalog.")
    sub = parser.add_subparsers(dest="command", required=True)

    p_ex = sub.add_parser(
        "example", aliases=["exemplo"], help="writes a synthetic catalog, labeled as synthetic"
    )
    p_ex.add_argument("--output", "--saida", required=True, dest="output")
    p_ex.add_argument("--n", type=int, default=32)

    p_cf = sub.add_parser(
        "check-cases", aliases=["conferir-casos"], help="validates the catalog without calling any model"
    )
    p_cf.add_argument("--cases", "--casos", required=True, dest="cases")

    p_in = sub.add_parser(
        "infer", aliases=["inferir"], help="runs the frozen inference and writes the two sources"
    )
    p_in.add_argument("--cases", "--casos", required=True, dest="cases")
    p_in.add_argument(
        "--runner",
        choices=["llama-cli", "vllm", "api", "file", "arquivo", "demo"],
        required=True,
    )
    p_in.add_argument(
        "--inferences-output", "--saida-inferencias", required=True, dest="inferences_output"
    )
    p_in.add_argument(
        "--outcomes-output", "--saida-desfechos", required=True, dest="outcomes_output"
    )
    p_in.add_argument("--model", "--modelo", default="", dest="model")
    p_in.add_argument("--binary", "--binario", default="llama-cli", dest="binary")
    p_in.add_argument("--url", default="http://127.0.0.1:8000/v1/completions")
    p_in.add_argument(
        "--api-url", "--url-api", default="https://api.openai.com/v1/chat/completions", dest="api_url"
    )
    p_in.add_argument("--key-env", "--chave-env", default="JEV_API_KEY", dest="key_env")
    p_in.add_argument(
        "--repeat", "--repetir", type=int, default=1, dest="repeat",
        help="repetitions per case, to measure determinism",
    )
    p_in.add_argument(
        "--two-stage", "--duas-etapas", action="store_true", dest="two_stage",
        help="elicit the number before the document",
    )
    p_in.add_argument(
        "--grammar", "--gramatica", default="", dest="grammar",
        help="GBNF file that constrains the output",
    )
    p_in.add_argument(
        "--answers", "--respostas", default="", dest="answers",
        help="caso_id<TAB>answer file, for the file runner",
    )
    p_in.add_argument("--seed", type=int, default=2026)
    p_in.add_argument("--max-tokens", type=int, default=400)

    args = parser.parse_args()
    # argparse echoes the spelling that was typed, so the legacy alias is mapped back to the canonical name here.
    args.command = COMMAND_ALIASES.get(args.command, args.command)
    if getattr(args, "runner", None):
        args.runner = RUNNER_ALIASES.get(args.runner, args.runner)

    if args.command == "example":
        write_example(Path(args.output), args.n)
        return 0

    cases = read_cases(Path(args.cases))
    if args.command == "check-cases":
        positives = sum(1 for c in cases if c["desfecho_observado"] == "1")
        print(f"catalog ok: {len(cases)} cases, {positives} with outcome 1 and {len(cases) - positives} with 0")
        if len(cases) < 30:
            print("volume below 30: the instrument will not publish the score", file=sys.stderr)
            return 1
        return 0

    collected = read_collected_answers(Path(args.answers)) if args.runner == "file" and args.answers else {}
    if args.runner in ("llama-cli", "vllm", "api") and not args.model:
        print(f"runner {args.runner} requires --model", file=sys.stderr)
        return 1
    if args.runner == "file" and not collected:
        print("runner file requires --answers with at least one row", file=sys.stderr)
        return 1

    grammar = ""
    if args.grammar:
        grammar = Path(args.grammar).read_text(encoding="utf-8")
        print(f"  grammar: {args.grammar} ({len(grammar)} bytes) — form is no longer measured, it is guaranteed")

    inference_lines: list[str] = []
    outcome_lines: list[str] = [
        "\t".join(["caso_id", "probabilidade_declarada", "desfecho_observado", "fonte_desfecho"])
    ]
    for index, case in enumerate(cases, start=1):
        prompt = build_prompt(
            case["bioma"],
            float(case["temp_c"]),
            int(case["umid_rel"]),
            int(case["vento_kmh"]),
            int(case["dias_sem_chuva"]),
        )
        if args.two_stage:
            # Stage 1: the number. Stage 2: the document, with the stage-1 margin inside. Without it the small
            # model copies the example's margin and every forecast comes out the same — constant calibration
            # measures nothing.
            number_prompt = build_number_prompt(
                case["bioma"],
                float(case["temp_c"]),
                int(case["umid_rel"]),
                int(case["vento_kmh"]),
                int(case["dias_sem_chuva"]),
            )
            if args.runner in ("llama-cli",):
                raw = runner_llama_cli(number_prompt, args.model, args.binary, args.seed, 32, "")
            elif args.runner == "vllm":
                raw = runner_vllm(number_prompt, args.url, args.model, args.seed, 32, "")
            elif args.runner == "api":
                key = os.environ.get(args.key_env, "")
                if not key:
                    print(f"runner api requires the key in {args.key_env}", file=sys.stderr)
                    return 1
                raw = runner_api(number_prompt, args.api_url, args.model, key, args.seed, 32)
            else:
                print("--two-stage does not apply to demo or file", file=sys.stderr)
                return 1
            try:
                margin = extract_number(raw)
            except ValueError as err:
                raise SystemExit(f"{case['caso_id']}: {err}") from None
            prompt = build_document_prompt(
                case["bioma"],
                float(case["temp_c"]),
                int(case["umid_rel"]),
                int(case["vento_kmh"]),
                int(case["dias_sem_chuva"]),
                margin,
                index,
            )
            if args.runner == "llama-cli":
                response = runner_llama_cli(prompt, args.model, args.binary, args.seed, args.max_tokens, grammar)
            elif args.runner == "vllm":
                response = runner_vllm(prompt, args.url, args.model, args.seed, args.max_tokens, grammar)
            else:
                response = runner_api(
                    prompt,
                    args.api_url,
                    args.model,
                    os.environ.get(args.key_env, ""),
                    args.seed,
                    args.max_tokens,
                    grammar=grammar,
                )
        elif args.runner == "demo":
            response = runner_demo(case, index)
        elif args.runner == "file":
            if case["caso_id"] not in collected:
                raise SystemExit(f"case {case['caso_id']} has no answer in the collected file")
            response = collected[case["caso_id"]]
        elif args.runner == "llama-cli":
            response = runner_llama_cli(prompt, args.model, args.binary, args.seed, args.max_tokens)
        elif args.runner == "vllm":
            response = runner_vllm(prompt, args.url, args.model, args.seed, args.max_tokens)
        else:
            key = os.environ.get(args.key_env, "")
            if not key:
                print(
                    f"runner api requires the key in {args.key_env}; no credential is written into the tree",
                    file=sys.stderr,
                )
                return 1
            response = runner_api(
                prompt, args.api_url, args.model, key, args.seed, args.max_tokens
            )

        try:
            prob = validate_sheet(response)
        except ValueError as err:
            raise SystemExit(f"{case['caso_id']}: {err}") from None

        inference_lines.append(f"{escape_newlines(prompt)}\t{escape_newlines(response)}")
        outcome_lines.append(
            f"{case['caso_id']}\t{prob:.4f}\t{case['desfecho_observado']}\t{case['fonte']}"
        )

    if args.repeat > 1:
        # The G1 proof at the provider: same question, two calls, byte-for-byte equal answer.
        if args.runner in ("demo", "file"):
            print(f"  determinism: runner {args.runner} is deterministic by construction, nothing to repeat")
        else:
            case = cases[0]
            prompt = build_prompt(
                case["bioma"],
                float(case["temp_c"]),
                int(case["umid_rel"]),
                int(case["vento_kmh"]),
                int(case["dias_sem_chuva"]),
            )
            equal = 0
            first = None
            for _ in range(args.repeat):
                if args.runner == "llama-cli":
                    r = runner_llama_cli(prompt, args.model, args.binary, args.seed, args.max_tokens)
                elif args.runner == "vllm":
                    r = runner_vllm(prompt, args.url, args.model, args.seed, args.max_tokens)
                else:
                    r = runner_api(
                        prompt,
                        args.api_url,
                        args.model,
                        os.environ.get(args.key_env, ""),
                        args.seed,
                        args.max_tokens,
                    )
                if first is None:
                    first = r
                elif r == first:
                    equal += 1
            print(
                f"  determinism: {equal} of {args.repeat - 1} repetitions identical to the first"
                + (" | closes G1" if equal == args.repeat - 1 else " | does NOT close G1")
            )

    inferences_output = Path(args.inferences_output)
    outcomes_output = Path(args.outcomes_output)
    inferences_output.parent.mkdir(parents=True, exist_ok=True)
    outcomes_output.parent.mkdir(parents=True, exist_ok=True)
    inferences_output.write_text("\n".join(inference_lines) + "\n", encoding="utf-8")
    outcomes_output.write_text("\n".join(outcome_lines) + "\n", encoding="utf-8")
    print(f"-> {inferences_output} ({len(inference_lines)} inferences)")
    print(f"-> {outcomes_output} ({len(outcome_lines) - 1} labeled pairs)")
    print("next step: cargo run --release --bin ingest-jev -- --check <inf> <desf>")
    return 0


if __name__ == "__main__":
    sys.exit(main())
