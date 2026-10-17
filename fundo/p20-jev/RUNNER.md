# O runner local dos pesos congelados — manual do operador

O item 13 tem duas metades, e elas não se confundem: **o runner é código e já existe**; **os pesos são artefato e
só o operador os traz**. Este arquivo é o contrato de quem opera. Ele não pede julgamento humano, não pede painel e
não pede escuta: pede três arquivos, uma ordem de operações e um registro.

## 1. O que existe hoje

| peça | onde | estado |
| --- | --- | --- |
| o portão dos pesos | `crates/stochology-s2/src/ponte.rs`, `verificar_pesos` | fecha: sem arquivo ou com hash diferente, a ponte não roda |
| o runner | `crates/stochology-s2/src/runner.rs`, `RunnerLocal` | fecha: carrega o artefato, confere o hash declarado, lê os blocos e responde |
| a leitura do hash declarado | `runner.rs`, `hash_declarado` | fecha: sem `SHA256` no diretório, o runner recusa em vez de adivinhar |
| a prova de determinismo | `ponte.rs`, `provar_determinismo` | fecha no runner: duas execuções, o mesmo `sha256` |
| a adequação do espaço | `ponte.rs`, `adequacao_do_espaco` | fecha: pedido sem resposta congelada abstém e entra na conta |
| a regra de pontuação | `runner.rs`, `brier` | implementada; o número sai quando houver conjunto rotulado |
| **os pesos** | `fundo/p20-jev/pesos/modelo.congelado` | **ausente**: é o que falta, e é o que só o operador resolve |

O instrumento prova o caminho inteiro com um artefato de ensaio que ele escreve em diretório temporário e apaga:
`labs --jev` imprime o hash do ensaio, o número de respostas, o determinismo, a fração de abstenções e o custo por
chamada. Ensaio não é peso de modelo: o veredito de calibração continua sem número.

## 2. O que o operador faz, na ordem

A ordem importa: **o hash é declarado antes da execução**. Declarar depois de ver o resultado é escolher a
especificação olhando o número, e isso conta no K.

1. **Escolher a revisão do modelo.** Uma só, nomeada e datada: repositório, identificador da revisão e data do
   congelamento. Trocar de revisão depois é declaração nova.
2. **Produzir o artefato.** O formato está em §3. O artefato é a tabela de respostas congeladas, com o documento
   canônico da ficha em cada bloco. O operador colhe as respostas do modelo que congelou e escreve o arquivo com
   `stochology_s2::runner::escrever_artefato(dir, &[(pedido, resposta), …])`, que grava o artefato **e** o
   `SHA256`; o caminho manual é o mesmo formato escrito à mão. Não há tensor neste formato, e isso está declarado:
   um carregador de tensor é passo seguinte, não meio-caminho.
3. **Declarar o hash antes de rodar:**
   ```bash
   cd fundo/p20-jev/pesos
   sha256sum modelo.congelado > SHA256      # o operador declara, agora, antes da primeira execução
   ```
4. **Rodar a ponte**:
   ```bash
   cargo run --release --bin labs -- --jev
   ```
   Com o hash batendo, a saída muda de `RECUSADO` para `pesos verificados: <hash> | N respostas congeladas`, e o
   determinismo e a adequação passam a ser medidos no modelo de verdade.
5. **Preencher o registro.** `fundo/p20-jev/registro.tsv` ganha a linha do artefato: revisão, hash, data, número de
   respostas. O registro é a prova; o arquivo sozinho não é.
6. **Medir o Brier** contra o Jev determinístico. Exige o conjunto de pares (probabilidade, desfecho) fora do
   ajuste; sem ele, `runner::brier` existe e não tem entrada. Este é o único passo que pede dado rotulado, e não
   código novo.

## 3. O formato do artefato

Texto, primeira linha o formato, um bloco por resposta congelada:

```text
# p20-jev/runner-v1
@pedido <sha256 do texto do pedido, 64 hex>
<documento canonico da ficha, uma linha por campo>
@end
```

O `sha256` do pedido é o do texto que a ponte manda, que é `ponte::pedido(pergunta)` — não o da pergunta crua.
Pedido sem bloco **não** inventa resposta: devolve `fora do esquema`, que a ponte converte em abstenção
(`Categoria::Outros`) e conta na adequação.

O que invalida o artefato, e o runner recusa em vez de tolerar:

| defeito | o que acontece |
| --- | --- |
| falta o arquivo | `PesosAusentes` — a ponte não roda |
| falta o `SHA256` | `HashNaoDeclarado` — a ponte não roda |
| um byte trocado no artefato | `PesosComHashDiferente` — a ponte não roda |
| cabeçalho errado, bloco sem `@end`, chave que não é `sha256`, bloco vazio | `ArtefatoIlegivel` — a ponte não roda |

## 4. O que fica declarado, e não é pendência de código

- **Tensor não é lido.** O artefato é a tabela; o carregador de peso bruto fica como passo seguinte, com o formato
  declarado antes de existir.
- **O custo é contado em palavras do documento devolvido**, não no tokenizador do modelo. O número está declarado,
  e a comparação com o custo real entra quando o artefato real trouxer a contagem do próprio modelo.
- **O Brier exige conjunto rotulado.** Sem ele não há pontuação, e a ponte diz isso em vez de publicar número de
  ensaio como se fosse calibração.

## 5. Verificação do que aqui se afirma

```bash
cargo test -p stochology-s2      # os testes do runner: carga, hash, adulteração, formato, G1, abstenção, Brier
cargo run --release --bin labs -- --jev
```
