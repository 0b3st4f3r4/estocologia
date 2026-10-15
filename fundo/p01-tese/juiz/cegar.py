import re, sys, numpy as np, pathlib, json
P=pathlib.Path('/mnt/project-files/estocologia/fundo/p01-tese/pareceres')
out=pathlib.Path(sys.argv[1])
rng=np.random.default_rng(20261017)
termos=[r'\bL[123]\b',r'\bMundo\b',r'\bModelo\b(?= \(L2\))',r'\bfichas?\b',r'\bFichas?\b',r'\bport[õo]es?\b',r'\bPort[õo]es?\b',r'\binvariantes?\b',r'Φ\s?[1-4]',r'\bF[1-8]\b',r'\bM1[0-5]\b|\bM[0-9]\b',r'Estocologia',r'Fluxonomia',r'Crononomia',r'Escalonomia',r'Morfonomia',r'Crisonomia',r'\bbanca\b',r'\bBanca\b',r'orquestrador',r'hipergrafo',r'\bAgente\b',r'\bcamadas?\b',r'\blentes?\b',r'SHELF',r'Winton']
mapa={}
for i in range(1,13):
    c=f'caso_{i:02d}'; perm=rng.permutation(2)
    lab={'A':'X','B':'Y'} if perm[0]==0 else {'A':'Y','B':'X'}
    mapa[c]=lab
    for b in 'AB':
        t=(P/b/f'{c}.md').read_text()
        for pat in termos: t=re.sub(pat,'[…]',t)
        m=re.search(r'VEREDITO:.*',t,re.S)
        (out/f'{c}_{lab[b]}_texto.md').write_text(t)
        (out/f'{c}_{lab[b]}_bloco.md').write_text(m.group(0) if m else 'SEM BLOCO')
json.dump(mapa,open(sys.argv[2],'w'),indent=1)
