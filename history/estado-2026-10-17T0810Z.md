# Declaração da edição 2: a retificação de 2026-10-17T08:10Z

A edição foi congelada às 06:47:33Z e retificada às 07:27Z. Esta é a segunda retificação, e ela existe porque a
primeira deixou uma divergência dentro do próprio congelamento: o livro ganhou duas linhas em `book/cap13.md`
na retificação do trecho sobre o rótulo M15, o manifesto `RESULTADOS_RUST.md` foi regerado com a contagem nova e
o texto do `README.md`, do `PLAN.md` e de `DECLARACAO_DA_EDICAO.md` continuou afirmando a contagem velha. O
`book/gerar.py --check` passava, porque conferia o texto e não o número escrito à mão.

## 1. Os dois hashes, lado a lado

O mesmo arquivo, `DECLARACAO_DA_EDICAO.md`, antes e depois desta retificação:

| momento | sha256 de `DECLARACAO_DA_EDICAO.md` |
| --- | --- |
| congelamento de 06:47:33Z, retificado em 07:27Z (o texto com as afirmações corrigidas aqui) | `1e8d61d5a72b8021a52b6b921c0f3ea78e478a1872dcefd06bd06dc0d9c4765c` |
| retificação de 2026-10-17T08:10Z (o texto desta árvore) | `dd88ff4661c2dd19e5a0c996ecf62d7511d5b34ab0c6ea50ed6d42ea7e5cc0c5` |

O hash posterior é o que `DECLARACAO_DA_EDICAO.sha256` passa a declarar, e o comando que o confere é
`sha256sum -c DECLARACAO_DA_EDICAO.sha256` rodado da raiz, ou `python3 conferir_lacres.py`. O hash anterior fica
nesta tabela porque a linha do lacre não guarda histórico: ela aponta para o presente, e o passado desta árvore
está no commit `24ed78d`.

## 2. O que a retificação mudou

1. **A contagem.** `README.md` (duas linhas), `PLAN.md` e `DECLARACAO_DA_EDICAO.md` §6 passaram de **5 903 linhas e
   74 847 palavras** para **5 904 linhas e 74 861 palavras**, que é o que `montar()` mede e o que o manifesto já
   declarava.
2. **O §7.** A frase que dava o transcript `Integração de Modelos Jev à Estocologia.md` como o "único arquivo não
   versionado" era falsa nos dois lados: o arquivo está no Git desde o commit `5393515`, e a árvore tinha seis
   arquivos modificados quando esta retificação começou. O §7 passou a dizer o que é verificável.
3. **O método.** `book/gerar.py --check` ganhou a terceira cobrança: lê os pares "N linhas ... N palavras" do
   `README.md`, do `PLAN.md`, de `DECLARACAO_DA_EDICAO.md` e de `RESULTADOS_RUST.md`, compara com o que o mesmo
   `montar()` mede e sai com 1 quando divergem. Registro datado não entra na lista — um documento que afirma
   "rodada em 2026-10-16, com 5 652 linhas" está falando do passado, e a divergência dele se declara, não se
   reescreve.
4. **O registro datado do ponto 18.** `fundo/p18-eixo-r/DECLARACAO.md` manteve a linha de 2026-10-16 intacta e
   ganhou a declaração de divergência com os dois valores, o motivo e o hash anterior do arquivo; o lacre da pasta
   foi refeito, com o hash velho registrado no próprio documento.
5. **O instrumental de estilo.** `estilo/decisoes.md` passou a declarar os cinco scripts de `estilo/` —
   `varredura.py`, `dois_pontos.py`, `ocorrencias.py`, `atualizar-indice.py` e `conferir-passada.py` —, como
   `AGENTS.md` §VIII exige; dois deles não estavam nomeados.
6. **A contagem de comandos.** A frase de fechamento falava em "os oito comandos da seção 5". São dezoito: dez no
   bloco da reprodução e oito no dos instrumentos que provam os itens fechados.

## 3. O que fica escrito

Esta retificação não deixa promessa. Ela corrige um número, uma afirmação falsa e uma contagem de comandos,
fecha o método que deixou o número envelhecer e re-sela o que tocou. O que ela **não** fez: não reescreveu
registro datado, não mudou resultado nenhum, não mexeu em `book/` nem no livro montado, e não tocou nos quatro
itens vivos de `PROPOSTAS_DECLARADAS.md` §1 — SHELF (3), escuta (5), ponte em escala (9) e Jev-LLM (13), que
seguem abertos e não se fecham com código.

Quem quiser conferir não precisa acreditar em nada:

~~~bash
sha256sum -c DECLARACAO_DA_EDICAO.sha256
python3 conferir_lacres.py
python3 book/gerar.py --check      # a rampa, o disco e a contagem declarada
python3 book/gerar_resultados.py --check
~~~
