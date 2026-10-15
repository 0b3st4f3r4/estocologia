"""Baixa o challenge-2019 (PhysioNet/CinC 2019, sepse precoce) do espelho aberto do PhysioNet no S3.

Espelho: https://physionet-open.s3.amazonaws.com/ (sem credencial, listavel). O physionet.org esta bloqueado
pelo proxy desta maquina; o espelho S3 e oficial e aberto (ODC-By).

Uso:  python3 baixar.py [DESTINO]
DESTINO padrao: dados-reais/p10/challenge-2019/1.0.0  (relativo a raiz do repositorio)

Escreve cada .psv em <DESTINO>/training/training_set{A,B}/pNNNNNN.psv, preservando a estrutura do espelho.
(A primeira versão gravava <DESTINO>/challenge-2019/1.0.0/training/... — o prefixo do espelho duplicado; a correção
veio depois do download, e o README relata.)
Nao imprime nenhum valor de desfecho: so contagens, bytes e o hash por arquivo.
"""

import hashlib
import sys
import time
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

BASE = "https://physionet-open.s3.amazonaws.com"
PREFIXOS = ["challenge-2019/1.0.0/training/training_setA/", "challenge-2019/1.0.0/training/training_setB/"]
NS = "{http://s3.amazonaws.com/doc/2006-03-01/}"
SEGUNDOS = 60
TENTATIVAS = 4


def listar(prefixo: str) -> list[str]:
    chaves, token = [], None
    while True:
        url = f"{BASE}/?list-type=2&prefix={prefixo}&max-keys=1000"
        if token:
            url += "&continuation-token=" + urllib.parse.quote(token, safe="")
        with urllib.request.urlopen(url, timeout=SEGUNDOS) as r:
            raiz = ET.fromstring(r.read())
        chaves += [c.find(NS + "Key").text for c in raiz.findall(NS + "Contents")]
        trunc = raiz.find(NS + "IsTruncated")
        if trunc is None or trunc.text != "true":
            break
        token = raiz.find(NS + "NextContinuationToken").text
    return chaves


def baixar(chave: str, destino: Path) -> tuple[str, int, str]:
    alvo = destino / Path(*chave.split("/")[2:])   # destino já termina em challenge-2019/1.0.0
    if alvo.exists() and alvo.stat().st_size > 0:
        dado = alvo.read_bytes()
    else:
        alvo.parent.mkdir(parents=True, exist_ok=True)
        for t in range(TENTATIVAS):
            try:
                req = urllib.request.Request(f"{BASE}/{chave}", headers={"User-Agent": "estocologia-p10/1.0"})
                with urllib.request.urlopen(req, timeout=SEGUNDOS) as r:
                    dado = r.read()
                break
            except Exception:
                if t == TENTATIVAS - 1:
                    raise
                time.sleep(1.5 * (t + 1))
        tmp = alvo.with_suffix(alvo.suffix + ".parcial")
        tmp.write_bytes(dado)
        tmp.rename(alvo)
    return chave, len(dado), hashlib.sha256(dado).hexdigest()


def main() -> int:
    destino = Path(sys.argv[1] if len(sys.argv) > 1 else "dados-reais/p10/challenge-2019/1.0.0")
    destino.mkdir(parents=True, exist_ok=True)
    t0 = time.time()
    linhas, total = [], 0
    for prefixo in PREFIXOS:
        chaves = listar(prefixo)
        print(f"{prefixo}: {len(chaves)} arquivos listados", flush=True)
        with ThreadPoolExecutor(24) as ex:
            for i, (chave, tam, h) in enumerate(ex.map(lambda k: baixar(k, destino), chaves), 1):
                linhas.append((h, chave))
                total += tam
                if i % 4000 == 0 or i == len(chaves):
                    print(f"  {i}/{len(chaves)}  {total/1e6:.1f} MB  {i/(time.time()-t0):.1f} arq/s", flush=True)
    # manifest do proprio modulo (nome completo do modulo de origem), conferido por sha256sum -c
    manifesto = destino.parent / "SHA256SUMS.parcial"
    manifesto.write_text("".join(f"{h}  {k.split('1.0.0/', 1)[1]}\n" for h, k in sorted(linhas)))
    print(f"{len(linhas)} arquivos, {total/1e6:.1f} MB, {time.time()-t0:.1f} s; manifesto em {manifesto}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
