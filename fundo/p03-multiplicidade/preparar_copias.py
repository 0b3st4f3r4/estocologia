"""Monta as cópias de trabalho em copias/ a partir dos horizontes originais, que não são tocados.

- copias/h2_v2     cópia intacta de h2/ (o arcabouço v2, como foi publicado);
- copias/h2_v1     h2/ com o arcabouço v1 reconstruído (DECLARACAO.md, ponto 4);
- copias/h2g_bonf  h2g/ com alfa_icp = 0,05/M nos 18 pacotes e nada mais (DECLARACAO.md, ponto 3);
- copias/h2g_v1    h2g/ com os mesmos três reversos do v1 no funil;
- copias/h3_ri32   h3/ com os grupos de linhas do prisoes_ri de volta a 32 (a especificação de antes da correção
                   feita depois de abrir o cofre), para comparar o diário.

O v1 não existe no git (um único commit da pesquisa, 4bb7181). A reconstrução reverte as mudanças marcadas "(v2)"
em h2/estocologia/funil.py, conforme h2/README.md:
  1. F3: a chave é permutada linha a linha dentro de cada ambiente (cabeçalho do funil: "nulo de permutação dentro
     do ambiente");
  2. verossimilhança sem têmpera (peso = 1);
  3. conforme calibrado num quinto do treino sorteado (ajuste nos outros quatro quintos).

Uso:  python3 preparar_copias.py [M]      (M padrão 38)
"""

import re
import shutil
import sys
from pathlib import Path

AQUI = Path(__file__).resolve().parent
RAIZ = AQUI.parent.parent
COPIAS = AQUI / "copias"
M = int(sys.argv[1]) if len(sys.argv) > 1 else 38

IGNORAR = shutil.ignore_patterns("saida", "__pycache__", ".hypothesis", ".pytest_cache")

F3_V2_INICIO = "    # a chave é permutada no nível em que foi atribuída (v2):"
F3_V2_FIM = "    zs = np.array(zs)\n"
F3_V1 = '''    # v1 (reconstruído em fundo/p03-multiplicidade): a chave é permutada linha a linha dentro de cada ambiente
    k = treino[p.chave].to_numpy(float)
    nivel = "dentro do ambiente, linha a linha (v1 reconstruído)"
    idx_a = [np.flatnonzero(amb == a) for a in np.unique(amb)]
    zs = []
    for _ in range(PERMUTACOES):
        kk = k.copy()
        for s_ in idx_a:
            kk[s_] = rng.permutation(k[s_])
        Xp = X.copy()
        Xp[:, 1] = kk - k.mean()
        zs.append(abs(_z_chave(Xp, y, g, p.familia)))
    zs = np.array(zs)
'''

PESO_V2 = "    peso = float(min(1.0, 1.0 / deff)) if deff else 1.0\n"
PESO_V1 = "    peso = 1.0                                         # v1 (reconstruído): sem têmpera\n"

CONF_V2 = '''    esc_cal = []
    for a in np.unique(amb):
        s_ = amb == a
        fo = ajustar(X[~s_], y[~s_], fam)
        ph = _prever(p, X[s_], fo["beta"])
        esc_cal.append(y[s_] - ph if fam == "gaussiana" else 1 - np.where(y[s_] == 1, ph, 1 - ph))
    esc_cal = np.concatenate(esc_cal)
'''
CONF_V1 = '''    # v1 (reconstruído): calibração num quinto do treino sorteado; ajuste nos outros quatro quintos
    ordem = rng.permutation(X.shape[0])
    cal, fit_ = ordem[: X.shape[0] // 5], ordem[X.shape[0] // 5:]
    fo = ajustar(X[fit_], y[fit_], fam)
    ph = _prever(p, X[cal], fo["beta"])
    esc_cal = y[cal] - ph if fam == "gaussiana" else 1 - np.where(y[cal] == 1, ph, 1 - ph)
'''


def trocar(txt: str, velho: str, novo: str) -> str:
    assert txt.count(velho) == 1, f"trecho não encontrado exatamente uma vez: {velho[:60]!r}"
    return txt.replace(velho, novo)


def reverter_para_v1(funil: Path) -> None:
    t = funil.read_text()
    i, j = t.index(F3_V2_INICIO), t.index(F3_V2_FIM) + len(F3_V2_FIM)
    t = t[:i] + F3_V1 + t[j:]
    t = trocar(t, PESO_V2, PESO_V1)
    t = trocar(t, CONF_V2, CONF_V1)
    funil.write_text(t)


def copiar(origem: str, destino: str) -> Path:
    d = COPIAS / destino
    shutil.rmtree(d, ignore_errors=True)
    shutil.copytree(RAIZ / origem, d, ignore=IGNORAR)
    return d


if __name__ == "__main__":
    COPIAS.mkdir(exist_ok=True)
    copiar("h2", "h2_v2")
    reverter_para_v1(copiar("h2", "h2_v1") / "estocologia" / "funil.py")
    reverter_para_v1(copiar("h2g", "h2g_v1") / "estocologia" / "funil.py")
    d = copiar("h2g", "h2g_bonf")
    alfa = 0.05 / M
    for pac in sorted((d / "dominios").glob("*/pacote.yaml")):
        t = pac.read_text()
        if re.search(r"^alfa_icp:", t, flags=re.M):
            t = re.sub(r"^alfa_icp:.*$", f"alfa_icp: {alfa!r}", t, flags=re.M)
        else:
            t = t.rstrip("\n") + f"\nalfa_icp: {alfa!r}\n"
        pac.write_text(t)
    d = copiar("h3", "h3_ri32")
    f = d / "fluxos" / "prisoes_ri.py"
    f.write_text(trocar(f.read_text(), 'linhas_por_grupo = {"detencao": 4, "liberacao": 4, "canario": 4}',
                        'linhas_por_grupo = {"detencao": 32, "liberacao": 32, "canario": 32}'))
    print("cópias em", COPIAS, "; alfa_icp =", alfa)
