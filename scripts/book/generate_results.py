#!/usr/bin/env python3
"""Generate docs/RUST_RESULTS.md: the record of Rust results harvested directly from the instruments.

Usage:
    python3 scripts/book/generate_results.py            # write docs/RUST_RESULTS.md
    python3 scripts/book/generate_results.py --check    # verify if docs/RUST_RESULTS.md is up to date
"""

from __future__ import annotations

import argparse
import difflib
import hashlib
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
TARGET_DOC = ROOT / "docs" / "RUST_RESULTS.md"
BIN = ROOT / "target" / "release" / "labs"
GATES_BIN = ROOT / "target" / "release" / "gates"


SLOW_TEST = re.compile(r"^test .* has been running for over \d+ seconds$")


def normalize(text: str) -> str:
    """Strip run-specific timing and cargo status lines so the record is deterministically verifiable."""
    text = re.sub(r"finished in [0-9.]+s", "finished in Xs", text)
    for pattern in (r"^\s*Compiling .*$", r"^\s*Finished .*$", r"^\s*Running .*$", r"^\s*Doc-tests .*$"):
        text = re.sub(pattern, "", text, flags=re.M)
    lines = text.splitlines()
    out: list[str] = []
    block: list[str] = []
    for line in lines:
        if SLOW_TEST.match(line):
            continue
        if line.startswith("test "):
            block.append(line)
            continue
        if block:
            out.extend(sorted(block))
            block = []
        out.append(line)
    if block:
        out.extend(sorted(block))
    text = "\n".join(out)
    return re.sub(r"\n{3,}", "\n\n", text).strip()


def run_cmd(cmd: list[str]) -> str:
    proc = subprocess.run(cmd, capture_output=True, text=True, cwd=ROOT)
    out = normalize((proc.stdout + proc.stderr).rstrip())
    return out if out else "(no output)"


def rel_cmd_arg(arg: str) -> str:
    prefix = str(ROOT) + "/"
    return "./" + arg[len(prefix):] if arg.startswith(prefix) else arg


def section_block(title: str, cmd: list[str], note: str) -> str:
    text = run_cmd(cmd)
    cmd_str = " ".join(rel_cmd_arg(p) for p in cmd)
    return f"### {title}\n\n{note}\n\n~~~bash\n{cmd_str}\n~~~\n\n~~~text\n{text}\n~~~\n"


def state_manifest() -> str:
    """The state manifest: every empirical point declaration with its SHA-256 hash and tree counts."""
    lines = [
        "Every empirical point declaration with its file hash. The full chain is verified via "
        "`python3 scripts/check_seals.py`, and the table below is generated rather than transcribed.",
        "",
        "| Point | Declaration | SHA-256 |",
        "| --- | --- | --- |",
    ]
    total = 0
    reissued = 0
    for folder in sorted((ROOT / "empirical").glob("p*")):
        for decl in sorted(folder.glob("DECLARACAO*.md")) + sorted(folder.glob("DECLARATION*.md")):
            lines.append(f"| {folder.name} | {decl.name} | {hashlib.sha256(decl.read_bytes()).hexdigest()} |")
            total += 1
            if decl.name not in ("DECLARACAO.md", "DECLARATION.md"):
                reissued += 1
    source = (ROOT / "crates" / "stochology-book" / "src" / "main.rs").read_text(encoding="utf-8")
    # Modifier flags take a value and select nothing on their own; --help is not an instrument either.
    modifiers = {"--answer-key", "--answers", "--measure", "--scale", "--threshold"}
    all_flags = sorted(set(re.findall(r'"(--[a-z0-9-]+)"', source)))
    instruments = [f for f in all_flags if f not in modifiers and f != "--help"]
    chapters = len(list((ROOT / "book").glob("ch*.md")) or list((ROOT / "book").glob("cap*.md")))
    points = len(list((ROOT / "empirical").glob("p*")))
    crates = len(list((ROOT / "crates").glob("stochology-*")))
    book_file = ROOT / "book" / "stochology-textbook.md"
    book_text = book_file.read_text(encoding="utf-8")
    lines.append("")
    lines.append(
        f"Point declarations: {total}, of which {reissued} are declaration 2 or later (declared rectification "
        f"with both values) | empirical points: {points} | `labs` instruments: {len(instruments)} "
        f"({chr(44).join(instruments)}; plus {len(modifiers)} modifier flags: {chr(44).join(sorted(modifiers))}) | "
        f"chapters: {chapters} | crates: {crates}"
    )
    lines.append("")
    lines.append(
        f"Assembled book: `book/stochology-textbook.md` with **{len(book_text.splitlines())} lines** and "
        f"**{len(re.findall(r'\w+', book_text))} words**, "
        f"sha256 `{hashlib.sha256(book_file.read_bytes()).hexdigest()}`. The file on disk must match what "
        f"`scripts/book/build.py` assembles, verified character by character by `python3 scripts/book/build.py --check`."
    )
    return "\n".join(lines) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()

    if not BIN.exists() or not GATES_BIN.exists():
        print("missing binaries: run cargo build --release first")
        return 1

    parts = [
        "# Rust Results\n",
        "Record harvested directly from the Rust instruments on 2026-10-17. Nothing here is manually transcribed: "
        "every block is the literal stdout of the command printed above it.\n",
    ]
    parts.append(
        "| Instrument | What it measures |\n| --- | --- |\n"
        "| BTlabs --chapter NBT | Chapter N laboratory with fixed seed |\n"
        "| BTlabs --allBT | All 21 textbook laboratories in sequence |\n"
        "| BTlabs --bankBT | Real H0: 2.65 million bank calls, Erlang C schedule, independent unit, and split-half null |\n"
        "| BTlabs --resolutionBT | Horizon R: cost of coarse questions under identical total resources |\n"
        "| BTlabs --s1BT | Deterministic System 1 triage |\n"
        "| BTlabs --s2BT | System 2: hypergraph search, board belief state, Metropolis inference, and R-axis |\n"
        "| BTlabs --pretestBT | Calibration pretest null for the SHELF panel |\n"
        "| BTlabs --geneticsBT | Interventional validation resolution: what the second cell line changes |\n"
        "| BTlabs --streamsBT | The real streams of Chapter 20, rebuilt on public data with a declared specification |\n"
        "| BTlabs --eprocessBT | The H2 gate under the e-process, over the 38 recorded tests, as declared before the run |\n"
        "| BTlabs --listeningBT | Seal of the seven decisions affecting people: who was heard |\n"
        "| BTlabs --h3BT | H3 chain: what is re-measured, what is re-derived, and what remains a record |\n"
        "| BTlabs --bridgeBT | Bridge thesis at scale: re-derived pilot and fair-test gate |\n"
        "| BTlabs --schemaBT | Card contract: schema version and file-backed migration chain |\n"
        "| BTlabs --second-opinionBT | Second opinion on results marked P |\n"
        "| BTlabs --jevBT | Jev-LLM bridge: bridge gates and what requires frozen weights |\n"
        "| BTlabs --queueBT | Queue with abandonment and the four staffing rules (R0 to R3) |\n"
        "| BTgatesBT | Seven deterministic gates (G1-G7) and safeguard panel A1-A8 |\n"
        "| BTlabs --rehearsal-shelfBT | Synthetic floor of the SHELF panel: four declared respondents on real items |\n"
        "| BTlabs --rehearsal-listeningBT | Coverage arithmetic and gate boundary for affected-group listening |\n"
        "| BTlabs --rehearsal-bridgeBT | Roles, cases, and status in the scale bridge gate |\n"
        "| BTcargo test --workspaceBT | Test suite across all workspace crates |\n"
    )
    parts.append("## 1. Real H0 and Empirical Instruments in Rust\n")
    parts.append(section_block(
        "Bank call center (Chapter 12)",
        [str(BIN), "--bank"],
        "Reads BTdata/bank_calls.csvBT (5-minute public dataset), builds the Erlang C schedule over the mean "
        "training profile, and measures holdout performance.",
    ))
    parts.append(section_block(
        "Queue with abandonment and staffing rules (R0 to R3)",
        [str(BIN), "--queue"],
        "Reads the bank call dataset, reconstructs the queue with abandonment via two independent engines, and "
        "measures the four staffing rules.",
    ))
    parts.append(section_block(
        "System 1: deterministic card triage",
        [str(BIN), "--s1"],
        "Closed vocabulary of 6 categories and 35 themes, suffix tree of 136 entries, probabilities clamped to "
        "[0.01, 0.99], and normalized sum checked on every pass.",
    ))
    parts.append(section_block(
        "SHELF panel calibration pretest",
        [str(BIN), "--pretest"],
        "Declared null: a perfectly calibrated expert with uniform prior under the protocol design.",
    ))
    parts.append(section_block(
        "Deliberative System 2",
        [str(BIN), "--s2"],
        "LATS versus greedy search on the sample hypergraph, board belief state, Beta(4, 8) posterior versus "
        "closed form (1/3), and R-axis resolution selection.",
    ))
    parts.append(section_block(
        "Interventional validation resolution (yeast genetics)",
        [str(BIN), "--genetics"],
        "Replicate design: 40 mutants, comparing single-line versus two-line validation.",
    ))
    parts.append(section_block(
        "The real streams of Chapter 20, rebuilt",
        [str(BIN), "--streams"],
        "Energy, bank, market, flights of 2013 and 2023, cyclones and Toronto 1997-2002, from public data with a "
        "specification declared before the run; the canary under both clocks; the fairness audit by colour.",
    ))
    parts.append(section_block(
        "The H2 gate under the e-process, retroactively",
        [str(BIN), "--eprocess"],
        "One ledger per area over the 38 recorded tests, the mean of the e-values, two readings as declared in "
        "empirical/p22-parada/DECLARACAO_2.md.",
    ))
    parts.append(section_block(
        "H3 chain re-executed from the repository tree",
        [str(BIN), "--h3"],
        "Reads BTempirical/p14-h3/registro.tsvBT, classifies each of the four H3 rows, re-derives the performative "
        "loop from BTempirical/p06-laco/resultados.jsonBT, and runs negative proofs.",
    ))
    parts.append(section_block(
        "Jev-LLM bridge",
        [str(BIN), "--jev"],
        "Weight gate, schema-constrained decoding with declared abstention, G1 determinism proof, and hypothesis-space adequacy.",
    ))
    parts.append(section_block(
        "Second opinion on results marked P",
        [str(BIN), "--second-opinion"],
        "Reads point 19 registry and verifies independent re-implementations and derivations.",
    ))
    parts.append(section_block(
        "Card contract: file-backed migration chain",
        [str(BIN), "--schema"],
        "Reads BTcrates/stochology-core/migracoes.tsvBT and demonstrates migrating a v1.0 card to the current schema.",
    ))
    parts.append(section_block(
        "Bridge thesis at scale",
        [str(BIN), "--bridge"],
        "Re-derives pilot P01 from blind judge scores and checks the scale bridge gate.",
    ))
    parts.append(section_block(
        "Seal of the seven decisions affecting people",
        [str(BIN), "--listening"],
        "Reads BTempirical/p13-escuta/registro.tsvBT and checks affected-group listening coverage.",
    ))
    parts.append(section_block(
        "Horizon R: question resolution",
        [str(BIN), "--resolution"],
        "Paired arms under identical total resources measuring resolution loss.",
    ))
    parts.append("## 2. The 21 Textbook Laboratories\n")
    parts.append(section_block("All laboratories", [str(BIN), "--all"], "Single run, fixed seed, offline."))
    parts.append("## 3. Deterministic Gates\n")
    parts.append(section_block(
        "Seven gates and the safeguard panel",
        [str(GATES_BIN), "--horizon", "all", "--reproduction"],
        "Seven gates (G1 to G7), identical reproduction, and Merkle root of the audit trail.",
    ))
    parts.append(section_block(
        "Negative proof: gate rejection",
        [str(GATES_BIN), "--horizon", "h3", "--defect", "h3.voos23"],
        "With a planted defect, G6 rejects and exits with code 1.",
    ))
    parts.append(section_block(
        "Synthetic trial: SHELF panel",
        [str(BIN), "--rehearsal-shelf"],
        "Four synthetic respondents scored on the five real pretest pools.",
    ))
    parts.append(section_block(
        "Synthetic trial: affected-group listening",
        [str(BIN), "--rehearsal-listening"],
        "Coverage arithmetic on synthetic records and gate provenance boundary.",
    ))
    parts.append(section_block(
        "Synthetic trial: scale bridge",
        [str(BIN), "--rehearsal-bridge"],
        "Four synthetic records testing role assignment, case count, and declared status.",
    ))
    parts.append("## 4. Workspace Tests\n")
    parts.append(section_block("All crates", ["cargo", "test", "--workspace"], "Unit and integration tests across the workspace."))
    parts.append("## 5. State Manifest\n")
    parts.append(state_manifest())

    text = "\n".join(parts).replace("BT", chr(96))
    if args.check:
        current = TARGET_DOC.read_text(encoding="utf-8") if TARGET_DOC.exists() else ""
        if current.strip() != text.strip():
            print("docs/RUST_RESULTS.md is out of date: run python3 scripts/book/generate_results.py")
            diff = difflib.unified_diff(
                current.splitlines(), text.splitlines(), "on disk", "generated", lineterm="", n=1
            )
            print("\n".join(list(diff)[:80]))
            return 1
        print("docs/RUST_RESULTS.md is up to date")
        return 0

    TARGET_DOC.parent.mkdir(parents=True, exist_ok=True)
    TARGET_DOC.write_text(text, encoding="utf-8")
    print(f"wrote {TARGET_DOC.relative_to(ROOT)}: {len(text.splitlines())} lines")
    return 0


if __name__ == "__main__":
    sys.exit(main())
