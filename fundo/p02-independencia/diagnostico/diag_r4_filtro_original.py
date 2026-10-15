"""Diagnóstico pós-congelamento: quantas paradas do cofre sobram com o filtro do original
(h3/fluxos/prisoes_ri.py:72, que tira também linhas sem sexo ou sem idade), contra o filtro declarado."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import pandas as pd
from comum import DADOS
d = pd.read_csv(os.path.join(DADOS, "h3c", "backup_rhode_island_traffic_stops_2005_2015.csv"))
a = d[d.stop_outcome.notna()]
b = a[a.driver_gender.notna() & a.driver_age.notna()]
for nome, x in (("so_desfecho", a), ("desfecho_sexo_idade", b)):
    c = x[x.stop_date >= "2013-01-01"]
    print(nome, "n_total", len(x), "n_cofre", len(c), "ybar_cofre", round((c.stop_outcome == "Arrest Driver").mean(), 5))
