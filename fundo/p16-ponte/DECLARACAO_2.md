# Declaração 2 do ponto 16: a escolha mecânica e o veredito da janela

**Congelada em 2026-10-16T22:51:23Z.** A `DECLARACAO.md` fica como está; esta segunda declaração vive ao lado dela, que é o
que o protocolo manda quando um critério ganha conteúdo novo.

| arquivo | antes | agora | por quê |
| --- | --- | --- | --- |
| `registro.tsv` | `fb547bcd6de4fc7f047c10f0b095b551fb5c1b3ace09784a2f25f235ee0c23c8` | `7ebd1c65c59e6d46e6c5335e8355e4a2b5858b225c7f782146d03a8679c7780c` | entrou o veredito da janela e o ponteiro da regra de escolha |
| `regra_de_escolha.md` | — | `18648bee18c09e856d4f568026e1b15dd8f1f8d31f9c4b631fce13f2d4ebb7cd` | a regra mecânica, congelada antes de existir lista |
| `DECLARACAO.md` | `109910beac758a052676c928da9ef7e890d4bf60fa165b22262441795b5fa96a` | o mesmo | fica como registro do congelamento |
| `DECLARACAO_2.md` (esta) | — | — | as respostas do dono; o hash próprio sai em `DECLARACAO.sha256` |
| `README.md` | `c204219a96e412398df83da135422821cbefc930da6afed2b85ee46b711a3ff5` | `3b60e6bf8f7608547aa83a0164b546e34612b9a5b7e13eabcc201a96663e8501` | entrou o registro das respostas do dono e o ponteiro da regra de escolha |

## Declaração 5 — as respostas do dono, registradas

Perguntado em 2026-10-16 sobre os quatro papéis do teste justo, o dono respondeu o seguinte, e a resposta fica
registrada como declaração, e não como conversa:

1. **Escolhedor dos casos:** não há ninguém hoje, fora do programa, para o papel de escolhedor exógeno.
2. **Analistas:** não há equipe. O dono pode atuar como analista interno em um dos braços; não há segundo analista
   humano, e muito menos analista externo independente, para compor os dois braços.
3. **Juiz:** não existe. Sem pesquisador independente fora do círculo do programa, a figura do juiz cego está
   vazia.
4. **Regra de escolha:** aceita integralmente, e a razão é a certa — qualquer escolha feita pelo autor contamina a
   exogeneidade e invalida o teste por viés de seleção. A escolha tem de ser algorítmica ou delegada a fonte
   externa cega. A regra está em `regra_de_escolha.md`, com a semente declarada **20270114**.
5. **Se a janela vencer sem ninguém:** o dono aceita **sem ressalvas** o veredito definitivo — a tese da ponte em
   escala fica **não sustentada**, e o programa a trata exatamente assim. Uma tese não vive em promessa perpétua.

## Declaração 6 — o que isto muda, e o que não muda

**Não muda nenhum critério.** A cláusula de saída da declaração 4 já dizia que a janela vencida sem cobertura
mínima deixa a tese não sustentada em definitivo; o dono confirmou a consequência, e por isso nada entra no K.
O que muda é o registro: o `registro.tsv` passa a carregar, em máquina, o veredito da janela e o ponteiro da regra
de escolha, e o `labs --ponte` imprime os dois.

**O que ganha forma é a regra de escolha.** Ela é mecânica por desenho: uma lista externa com hash registrado antes
da escolha, um filtro de elegibilidade que não usa nada do programa (catálogos públicos de falha de replicação,
correção e retratação, e casos com replicação bem-sucedida para os limpos), a semente declarada, a proporção de
8 com defeito e 4 limpos, e a ordem saindo do próprio sorteio. O portão da regra: uma lista que não sustenta a
proporção pedida **reprova a escolha** em vez de ser completada com o que houver — há teste para isso.

## Verificação

Rodada em 2026-10-16, depois do congelamento. Código de saída entre parênteses.

| Comando | Resultado |
| --- | --- |
| `cargo test -p stochology-book` | 37 testes aprovados, entre eles os oito do `ponte`: o piloto re-derivado, o registro que declara o que falta, a tese sustentada sem teste justo reprovada, os papéis designados que liberam o teste, o status `em teste` sem os papéis reprovado, a escolha mecânica e determinística na proporção, a lista que não sustenta a proporção reprovada e o veredito da janela lido do registro (0) |
| `labs --ponte` | o piloto re-derivado, os cinco papéis pendentes, o veredito da janela e as duas provas sintéticas (0) |
