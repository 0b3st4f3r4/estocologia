# Ponto 13 — a escuta de quem paga o erro

**Estado: protocolo congelado, escuta não executada, caixa lacrada.**

O programa tem sete decisões que mexem com pessoas reais e não tinha ouvido nenhuma delas
([`governanca/GRUPOS.md`](../../governanca/GRUPOS.md), coluna "ouvidos" em **nenhum** nas sete). Esta frente
congela o protocolo da escuta e lacra a caixa até existir escuta registrada.

| arquivo | o que é |
| --- | --- |
| `protocolo.md` | o protocolo, congelado com hash: objeto, o fato declarado, quem conduz, o roteiro, a regra de não substituição, a cláusula de saída, a voz do operador, mudança de critério |
| `DECLARACAO.md` | a declaração com os hashes e a medição do lacre |
| `DECLARACAO.sha256` | os hashes, em forma de `sha256sum` |
| `registro.tsv` | o registro em máquina das sete decisões, lido pelo instrumento |

**O que foi medido.** Cobertura de escuta: **0 de 7**. Cobertura mínima declarada para abrir a caixa: **2 dos 7**
grupos com registro assinado. Prazo da janela: **60 dias**, até **2026-12-15**. Veredito: **caixa lacrada** — as sete
decisões ficam como análise de sensibilidade ($n = 1$) e não viram diretriz de política pública nem decisão de
portão F8 até lá ou até a cobertura mínima.

**O que falta.** A escuta, e só ela: alcançar equivalentes estruturais dos sete grupos (associações de classe,
defensorias, representantes setoriais, herdeiros das bases originais), com facilitador de fora ou protocolo cego, e
registro assinado de quem falar. Se o prazo vencer sem a cobertura mínima, o artefato fica arquivado como exercício
metodológico de sensibilidade, com o lacre em hash.

**Como reproduzir.**

```bash
cargo run --release --bin labs -- --escuta
```

O instrumento lê `registro.tsv`, conta a cobertura, confere se alguma decisão se diz emitida sem ninguém ouvido e
dá o veredito do lacre. As duas provas — a decisão que mente e a caixa que abre — saem no relatório, e os testes do
módulo prendem as duas.
