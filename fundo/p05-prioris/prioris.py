"""Constrói as duas tabelas de prioris alternativas do ponto 5 (fundo/p05-prioris).

- prioris_literatura.yaml: uma priori por domínio tirada de uma fonte publicada e conferida online (URL em FONTES),
  pelo espírito do SHELF (mediana e quartis vindos da fonte, com a conta escrita aqui). Onde não há fonte conferida,
  a entrada é a priori fraca, marcada "sem fonte conferida".
- prioris_fracas.yaml: priori fracamente informativa declarada, centrada em zero (razão de chances 1), com dp igual a
  10 vezes o dp da priori original; no H1 real, Beta(1, 1) pelos quartis 0,25 / 0,5 / 0,75.

Regras (as mesmas da DECLARACAO.md):
  R1  fonte dá estimativa e e erro padrão s: mediana e, dp = 2 s (fator 2 de transporte entre estudo e domínio).
  R2  fonte dá uma faixa [a, b]: mediana (a + b)/2, quartis a e b.
  R3  fonte dá só um ponto e: mediana e, dp = |e|/2.
  Nos domínios binomiais a conta é feita no log da razão de chances, e a tabela guarda razões de chances.
Uso: python3 prioris.py   (grava os dois YAML ao lado deste arquivo)
"""

from pathlib import Path

import numpy as np
import yaml
from scipy import stats

RAIZ = Path(__file__).resolve().parent
REPO = RAIZ.parents[1]
Z25, Z05 = stats.norm.ppf(0.75), stats.norm.ppf(0.95)


def quartis_normal(m, dp):
    return {"q05": m - Z05 * dp, "q25": m - Z25 * dp, "q50": m, "q75": m + Z25 * dp, "q95": m + Z05 * dp}


def r1(e, s):
    return quartis_normal(e, 2 * s), f"R1: e = {e:.4g}, s = {s:.4g}, dp = 2s = {2 * s:.4g}"


def r2(a, b):
    m, dp = (a + b) / 2, (b - a) / (2 * Z25)
    q = quartis_normal(m, dp)
    q["q25"], q["q75"] = a, b
    return q, f"R2: faixa [{a:.4g}, {b:.4g}], mediana {m:.4g}, dp implicado {dp:.4g}"


def r3(e):
    return quartis_normal(e, abs(e) / 2), f"R3: ponto {e:.4g}, dp = |e|/2 = {abs(e) / 2:.4g}"


def em_rc(q):
    """quartis no log da razão de chances -> razões de chances"""
    return {k: float(np.exp(v)) for k, v in q.items()}


def original(h, nome):
    m = yaml.safe_load((REPO / h / "dominios" / nome / "pacote.yaml").read_text())
    q = dict(m["shelf"]["quartis"])
    binom = m["familia"] == "binomial"
    if binom:
        q = {k: float(np.log(v)) for k, v in q.items()}
    # o mesmo ajuste do dominio.ajustar_normal, sem otimizador: quartis simétricos ou quase
    import importlib.util
    spec = importlib.util.spec_from_file_location("dom", REPO / h / "estocologia" / "dominio.py")
    mod = importlib.util.module_from_spec(spec)
    import sys
    sys.modules["dom"] = mod
    spec.loader.exec_module(mod)
    return mod.ajustar_normal(q), binom


def fraca(h, nome):
    pri, binom = original(h, nome)
    q = quartis_normal(0.0, 10 * pri["dp"])
    nota = f"fraca: centro 0 (log-RC 0) e dp = 10 x {pri['dp']:.4g} (dp original)"
    q = em_rc(q) if binom else q
    return {"quartis": {k: float(v) for k, v in q.items()}, "fonte": nota,
            "painel": "priori fracamente informativa declarada; nenhum especialista humano consultado"}


# ---------------- fontes conferidas (URL e trecho conferido em FONTES no README) ----------------
LE_PS = np.mean([50, 61, 69, 76])          # Pritchett & Summers 1996, Tabela 1: médias de expectativa de vida por quartil de renda, 1990
AH = lambda dp: 6.7 * 0.644 * 2 * dp ** (0.644 - 0.5)   # d V / d sqrt(dp) de V = 6,7 dp^0,644 (Atkinson & Holliday 1977, em Knaff & Zehr)
PCT_SD = 1 / (100 * stats.norm.pdf(0))    # 1 ponto de percentil perto da mediana em desvios-padrão
SD_STAR_K = 73.138                         # dp de leitura+matemática nas turmas regulares do jardim (AER_STAR.csv; conta no README)
USO_CE = 0.528851797207922                 # uso médio do cinto nas 556 linhas de Cohen & Einav (AER_USSeatBelts.csv; conta no README)


def literatura():
    L = {}

    def put(h, nome, q, conta, fonte, mesma_base, binom=False):
        L[(h, nome)] = {"quartis": {k: float(v) for k, v in (em_rc(q) if binom else q).items()},
                        "fonte": f"{fonte}; {conta}" + ("; MESMA BASE DE DADOS do domínio (dupla contagem)" if mesma_base else ""),
                        "painel": "literatura publicada, conferida online (fundo/p05); nenhum especialista humano consultado"}

    # H2g
    q, c = r2(15.0, 25.0)
    put("h2g", "farmacologia_qt", q, c, "Rótulo do Tikosyn (FDA/Pfizer): 'positive slope of approximately 15-25 msec/(ng/mL) after the first dose' em voluntários normais", False)
    q, c = r2(0.30, 0.40)
    put("h2g", "direito_teto_beneficio", q, c, "Meyer, Viscusi & Durbin (1995), NBER w3494: 'elasticity of spell duration with respect to benefits of approximately .3 to .4'", True)
    q, c = r3(4 * PCT_SD)
    put("h2g", "educacao_star_turmas", q, c + f" (4 pontos de percentil x {PCT_SD:.4f} dp por ponto)", "Krueger (1999), NBER w6051: 'about 4 percentile points the first year'", True)
    q, c = r1(-0.133 / USO_CE, 0.047 / USO_CE)
    put("h2g", "transporte_cinto", q, c + f" (elasticidade -0,133 (0,047) dividida pelo uso médio {USO_CE:.4f})", "Cohen & Einav (2003), REStat 85(4), Tabela 2: coeficiente de log(uso) em log(mortes de ocupantes) -0,133 (EP 0,047)", True)
    # H2
    q, c = r2(AH(10.0), AH(100.0))
    put("h2", "clima_tempestades", q, c + " (derivada de V = 6,7 Δp^0,644 em √Δp, nos déficits de 10 e 100 hPa)", "Atkinson & Holliday (1977), MWR 105:421, V = 6,7(1010 - p)^0,644, como reproduzido por Knaff & Zehr (CIRA)", False)
    q, c = r1(0.024 * LE_PS, 0.024 / 1.18 * LE_PS)
    put("h2", "desenvolvimento_preston", q, c + f" (elasticidade 0,024, t = 1,18, vezes a expectativa de vida média {LE_PS:.1f} anos)", "Pritchett & Summers (1996), 'Wealthier is Healthier', JHR / Banco Mundial: elasticidade-renda da expectativa de vida por IV 0,024 (t = 1,18)", False)
    q, c = r2(0.07, 0.31)
    put("h2", "agronomia_nitrogenio", q, c + " (7 a 31 kg de grão por kg de N, em quintais: /100)", "Correndo et al. (2021), Field Crops Research 273:108299: eficiência agronômica do N no ótimo agronômico de 7 a 31 kg/kg (788 ensaios na Argentina)", False)
    q, c = r1(-0.66, 0.29)
    put("h2", "transito_alcool", q, c, "Stock & Watson (regressão de efeito fixo de estado das mortes por 10 mil no imposto da cerveja, base Fatalities), -0,66 (0,29), reproduzido em Hanck et al., Introduction to Econometrics with R, 10.3", True)
    q, c = r3(4 * PCT_SD * SD_STAR_K)
    put("h2", "educacao_star", q, c + f" (4 pontos de percentil x {PCT_SD:.4f} dp x {SD_STAR_K} pontos)", "Krueger (1999), NBER w6051: 'about 4 percentile points the first year'", True)
    # binomiais: no log da razão de chances
    pw = 2 * 0.08 / (1 + 1 / 1.36)
    pb = pw / 1.36
    lor = np.log(pb / (1 - pb)) - np.log(pw / (1 - pw))
    q, c = r3(lor)
    put("h2", "trabalho_curriculos", q, c + f" (razão de taxas 1/1,36 com taxa média 0,08: brancos {pw:.4f}, negros {pb:.4f}, log-RC {lor:.4f})",
        "Quillian et al. (2017), PNAS, doi 10.1073/pnas.1706255114: 'whites receive on average 36% more callbacks than African Americans'; a meta-análise inclui Bertrand & Mullainathan", False, binom=True)
    q, c = r2(np.log(1 / 1.52), np.log(1 / 1.32))
    put("h2", "direito_prisoes", q, c + " (log da RC inversa de 1,32-1,52; transposição da prisão para a liberação, como no pacote original)",
        "Kochel, Wilson & Mastrofski (2011), Criminology 49(2):473: efeito da raça na prisão entre 1,32 e 1,52 conforme o método (27 bases)", False, binom=True)
    return L


SEM_FONTE = {
    ("h2", "credito_hmda"): "procurado; o único número achado é do próprio Fed de Boston (mesma base, que o pacote original excluiu de propósito)",
    ("h2", "epidemiologia_contracepcao"): "procurado; nenhuma razão de chances publicada e conferida para Bangladesh 1988-89",
    ("h2", "energia_vitoria"): "procurado; nenhum MW por grau-dia publicado para Victoria achado",
    ("h2", "logistica_voos"): "procurado; nenhuma razão de chances por hora da grade conferida",
    ("h2", "saude_consultas"): "Deb & Trivedi (1997) usa a mesma base e o resumo não dá o coeficiente; nada conferido",
    ("h2g", "saude_rand"): "Aron-Dine et al. (2013) dão médias por plano, mas a conversão para log(1 + gasto) pede a média condicional do log, que a fonte não traz",
    ("h2g", "biologia_sachs"): "não há estimativa quantitativa publicada da elasticidade (o pacote original já dizia isso)",
    ("h2g", "engenharia_tunel_vento"): "argumento físico do pacote, sem número publicado",
    ("h2g", "fisica_tunel_luz"): "argumento físico do pacote, sem número publicado",
    **{("h2g", f"genetica_y{i:02d}"): "argumento genérico do pacote, sem número publicado" for i in range(1, 11)},
}
H1 = {"a": "nenhuma fração de anos positivos de um índice de ações fora do próprio mercado dos EUA conferida",
      "b": "o número publicado do emprego dos controles do NSW é da mesma base (LaLonde 1986; MDRC)",
      "c": "a frequência mensal da camada 100 xs 50 só existe na própria base dinamarquesa (McNeil 1997)"}


def main():
    lit, fr = {"h2": {}, "h2g": {}, "h1-real": {}}, {"h2": {}, "h2g": {}, "h1-real": {}}
    L = literatura()
    for h in ("h2", "h2g"):
        for p in sorted((REPO / h / "dominios").iterdir()):
            if not (p / "pacote.yaml").exists():
                continue
            f = fraca(h, p.name)
            fr[h][p.name] = f
            if (h, p.name) in L:
                lit[h][p.name] = L[(h, p.name)]
            else:
                lit[h][p.name] = {**f, "fonte": "SEM FONTE CONFERIDA (" + SEM_FONTE[(h, p.name)] + "); usa a " + f["fonte"]}
    for n, motivo in H1.items():
        e = {"quartis": {"q25": 0.25, "q50": 0.5, "q75": 0.75}, "painel": "priori fraca Beta(1, 1) pelos quartis",
             "fonte": "fraca: Beta(1, 1)"}
        fr["h1-real"][n] = e
        lit["h1-real"][n] = {**e, "fonte": f"SEM FONTE CONFERIDA ({motivo}); usa a fraca Beta(1, 1)"}
    for h in lit:
        (RAIZ / "prioris" / h).mkdir(parents=True, exist_ok=True)
        (RAIZ / "prioris" / h / "literatura.yaml").write_text(yaml.safe_dump(lit[h], allow_unicode=True, sort_keys=True))
        (RAIZ / "prioris" / h / "fraca.yaml").write_text(yaml.safe_dump(fr[h], allow_unicode=True, sort_keys=True))
    for h in lit:
        for n, e in lit[h].items():
            if not e["fonte"].startswith("SEM"):
                print(h, n, {k: round(v, 4) for k, v in e["quartis"].items()}, "|", e["fonte"][-160:])


if __name__ == "__main__":
    main()
