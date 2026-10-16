# Ponto 14 — a cadeia do H3, re-executada até onde a árvore deixa

**Estado: a parte reprodutível está instrumentada; duas linhas ficam declaradas, com o motivo.**

O registro empírico traz quatro linhas do H3: os seis fluxos sob relógio estrito com canário (**P**), o cofre de
Rhode Island (**D**), o laço performativo medido (**P**) e o hipergrafo do H3 (**P**). Esta frente re-executa o que
a árvore permite e declara o resto, em vez de deixar a marca **P** sem motivo.

| arquivo | o que é |
| --- | --- |
| `registro.tsv` | as quatro linhas, com a classe, a marca, o registro versionado e o critério declarado do laço |
| `DECLARACAO.md` | as cinco declarações, com a marca nova **V** e o limite linha por linha |
| `DECLARACAO.sha256` | os hashes, em forma de `sha256sum` |

**O resultado.** O laço performativo é a única linha derivável, porque `fundo/p06-laco/resultados.json` está na
árvore: o instrumento refaz a conta do critério declarado antes de rodar (sinal, ordem de grandeza, compatibilidade
estrita, força do instrumento) e chega ao mesmo veredito da frente — **o laço do H3 superestima o medido**, com a
razão de 4,1483 fora da banda de 1/3 a 3, zero divergência entre a conta e o registro. As outras três linhas ficam
como **registro** porque o dado bruto saiu da árvore com o andaime `h3/` (`DECLARACAO_REMOVIDO_NAO_SIGNIFICANTE.md`);
o que existe em Rust para elas é o **mecanismo**, no Capítulo 20 (o canário com o relógio honesto e o cofre de
laboratório), e mecanismo medido não é cadeia medida.

**Como reproduzir.**

```bash
cargo run --release --bin labs -- --h3
```

O instrumento lê `registro.tsv` e o `resultados.json`, e imprime a classe de cada linha, a re-derivação do laço e
as duas provas negativas: um registro em que a razão caia dentro da banda declarada muda o veredito, e um registro
em que o campo da razão não bata com a conta é reprovado.
