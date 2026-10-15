"""Roda só o ICP de um domínio com o código de um horizonte, sobre o mesmo treino que o funil desse horizonte monta.

Serve para os testes ICP que não deixaram saída gravada:
- as 6 reexecuções dos domínios do H2 com a correção do posto (o ICP do H2b sobre os dados e o funil do H2);
- os 2 ICPs diagnósticos do H2b (Sachs e cinto, depois da poda no F3 e no F2).

O treino é montado como em `rodar_dominio` (carregar, preparar, checar, ingerir no Armazem, ler sem o cofre,
Desenho). O ICP é chamado como o funil do horizonte o chama. Nada do horizonte é modificado: o código é importado
da pasta original.

Uso:  python3 icp_direto.py <pasta_horizonte_funil> <pasta_horizonte_icp> <dados> <saida.json> dominio [dominio ...]
"""

import importlib.util
import json
import sys
import tempfile
from pathlib import Path

funil_dir, icp_dir, dados, saida = (Path(a) for a in sys.argv[1:5])
nomes = sys.argv[5:]
sys.path.insert(0, str(funil_dir))

from estocologia import dominio as D            # noqa: E402
from estocologia.armazem import Armazem         # noqa: E402
from estocologia.funil import Desenho           # noqa: E402

spec = importlib.util.spec_from_file_location("icp_alt", icp_dir / "estocologia" / "icp.py")
icp_mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(icp_mod)

out = {}
with tempfile.TemporaryDirectory() as tmp:
    arm = Armazem(Path(tmp) / "armazem")
    for nome in nomes:
        p = D.carregar(funil_dir / "dominios" / nome)
        df = p.preparar(dados)
        erros = D.checar_tabela(p, df)
        assert not erros, erros
        df = df.assign(ambiente=df["ambiente"].astype(str))
        arm.ingerir(p.nome, df, cofre=p.ambientes["cofre"])
        colunas = ["y", "unidade", "ambiente", *p.candidatos]
        treino = arm.ler(p.nome, colunas, "agent-f2-data", excluir=p.ambientes["cofre"])
        des = Desenho(p, treino)
        blocos = {v: des.bloco(treino, v) for v in p.icp_candidatos}
        y = treino["y"].to_numpy(float)
        amb = treino["ambiente"].astype(str).to_numpy()
        r = icp_mod.icp(blocos, y, amb, p.familia)        # chamada do funil do H2 e do H2b: padrões do módulo
        out[nome] = {"area": p.area, "S_hat": r["S_hat"], "aceitos": len(r["aceitos"]),
                     "modelo_rejeitado": r["modelo_rejeitado"], "alpha": r["alpha"],
                     "p_por_subconjunto": {"+".join(t["S"]) or "∅": t["p"] for t in r["testes"]}}
        print(nome, out[nome]["S_hat"], out[nome]["aceitos"], max(out[nome]["p_por_subconjunto"].values()), flush=True)
saida.write_text(json.dumps(out, indent=1, ensure_ascii=False))
