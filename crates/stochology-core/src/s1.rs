//! Sistema 1: triagem determinística de fichas (declarado no PLAN.md §4 e no Capítulo 13,
//! implementado aqui).
//!
//! # O que este módulo faz
//!
//! Recebe a **definição em uma linha** de uma ficha e devolve uma **categoria de um conjunto
//! fechado** mais uma **probabilidade**. Nada é gerado: a resposta é uma escolha entre seis
//! rótulos declarados, com evidência lexical explícita e conferível.
//!
//! # Por que isto é "LLM-as-Jev"
//!
//! A expressão declarada no `PLAN.md` §4 pede um modo em O(1) de triagem categórica **fechada**.
//! Um LLM livre gera texto novo a cada chamada: duas execuções divergem, e o portão G1
//! (determinismo) não fecha. Aqui o lugar do LLM é ocupado por um **Jev** — um juiz mudo:
//!
//! 1. **sem geração livre** — a saída é um rótulo de `Categoria::TODAS`, nunca uma frase nova;
//! 2. **mesma entrada, mesma saída** — a árvore de sufixos é materializada num mapa ordenado por
//!    chave, e o desempate é pela ordem declarada das categorias;
//! 3. **nenhum 0% nem 100%** — a pontuação já nasce na escala [0,01; 0,99] (ver abaixo), a massa
//!    acima do teto é transferida para quem ainda tem folga e a soma é conferida a cada rodada;
//! 4. **o juiz não inventa vocabulário** — se nenhum tema declarado casa, a resposta é
//!    `outros`, com probabilidade baixa-mas-não-zero; o juiz diz "não sei", não improvisa.
//!
//! # A árvore de sufixos determinística
//!
//! Cada categoria declara temas (palavras-chave). No caminho de indexação a árvore de sufixos
//! é materializada como o conjunto **de todos os sufixos** de todos os temas, cada sufixo
//! apontando para o **tema mais curto** que o tem como sufixo. No caminho de consulta, para cada
//! posição do texto normalizado procura-se o **sufixo indexado mais longo** que ali começa — é a
//! travessia clássica da árvore, e é o que permite achar `ativas` dentro de `retroativas` e
//! `espera` dentro de `esperava` sem nenhuma regra morfológica.
//!
//! A pontuação de uma categoria é o **comprimento do tema casado** dividido pelo comprimento do
//! tema mais longo **do vocabulário** (treze, `contrafactual`). Denominador global, e não por
//! categoria: é o que torna dois casamentos de mesmo comprimento diretamente comparáveis e o que
//! faz o desempate ser uma operação sobre números iguais. A transferência para probabilidade é
//! afim, `p = 0,01 + 0,94 x escore`: a diferença de um caractere casado vale sempre o mesmo, e o
//! intervalo epistemológico deixa de ser um corte aplicado no fim para virar a escala da própria
//! pontuação. Um casamento de quatro caracteres devolve 0,9866 antes da normalização; o de dez,
//! 0,9966 — nunca 1.
//!
//! # Limite declarado
//!
//! O Sistema 1 é triagem, não julgamento: localiza a família de erro de uma ficha e devolve uma
//! distribuição sobre rótulos fechados. A pergunta que exige deliberação — eixo R, síntese de
//! código, estresse adversarial — fica com o Sistema 2, que vive em `crates/stochology-s2`: LATS,
//! motor de estado de crença, inferência de Metropolis e estresse deliberativo da banca.

use std::collections::BTreeMap;

/// Piso epistemológico: nenhuma probabilidade desce a 0%.
pub const PISO: f64 = 0.01;

/// Teto epistemológico: nenhuma probabilidade sobe a 100%.
pub const TETO: f64 = 0.99;

/// Comprimento mínimo de um tema casado para valer como evidência.
///
/// Abaixo disso o que casou é ruído (`de`, `a`, `em`) e o casamento é descartado.
pub const TEMA_MINIMO: usize = 4;

/// Nome do rótulo de ausência de evidência.
const ROTULO_OUTROS: &str = "outros";

/// Peso declarado do rótulo `outros` quando nenhum tema declarado casa na definição.
///
/// Baixa-mas-não-zero: o "não sei" tem massa, e não é dogma. Depois de normalizar sobre a
/// priori declarada (`outros` = 0,34 e cada tema = 0,01) a probabilidade efetiva fica em
/// 34/39 = 0,8718, com as cinco categorias temáticas em 0,0256 cada — nenhuma em zero, e
/// nenhuma no teto. Quando a definição casa algum tema, esta priori não entra: a massa sai do
/// casamento lexical.
const MASSA_SEM_EVIDENCIA: f64 = 0.34;

/// Peso declarado de cada categoria temática quando nenhum tema casa.
const PESO_SEM_CASAMENTO: f64 = 0.01;

/// Transferência do escore para massa: p = PISO + (1 - 6 x PISO) x escore.
///
/// O escore é o comprimento do tema casado dividido pelo tema mais longo do vocabulário, então
/// p = 0,01 exige casamento nenhum e p = 0,99 exige o tema mais longo inteiro. A reta é
/// monótona e afim: a diferença de um caractere casado vale sempre o mesmo em probabilidade, e
/// o intervalo epistemológico deixa de ser um corte aplicado no fim para virar a escala da
/// própria pontuação. Um casamento de quatro caracteres devolve 0,9866; o de dez, 0,9966.
fn transferir(escore: f64) -> f64 {
    let n = Categoria::TODAS.len() as f64;
    PISO + (1.0 - n * PISO) * escore
}

/// Viés declarado de ordinal: quanto o escore de uma categoria ganha por ser declarada antes.
///
/// É da ordem de `1e-9` — invisível para qualquer força que difira em um caractere, e decisivo
/// quando duas categorias casam tema de **mesmo comprimento**. Sem ele, um empate exato ficaria
/// à mercê do arredondamento do ponto flutuante; com ele, o desempate é uma regra declarada:
/// o menor ordinal vence.
const VIES_DE_ORDINAL: f64 = 1e-9;

/// Comprimento do tema mais longo do vocabulário (`contrafactual`, treze caracteres), usado
/// como denominador **global** do escore e conferido no `debug_assert` de `S1::novo`.
///
/// Denominador global, e não por categoria: é o que torna dois casamentos de mesmo comprimento
/// diretamente comparáveis, e portanto o que faz o desempate ser uma operação sobre números
/// iguais, e não um artefato de qual categoria tem o tema mais longo.
const TEMA_MAIS_LONGO: f64 = 13.0;

/// Conjunto **fechado** de categorias de triagem.
///
/// A ordem da enumeração é normativa: ela é o desempate canônico quando o casamento lexical é
/// exatamente empatado. Categoria nova só entra com declaração nova — não há categoria aberta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Categoria {
    /// Espera, fila, serviço, saturação de atendimento (Fluxonomia).
    Fila,
    /// Causalidade, intervenção, identificação (faixa causal).
    Causal,
    /// Cauda pesada, evento raro, variância, não linearidade (Crisonomia).
    Cauda,
    /// Calibração, cobertura, posto, escore (faixa 4).
    Calibracao,
    /// Justiça, grupo afetado, quem paga pelo erro (faixa 6).
    Justica,
    /// Ausência de evidência no vocabulário declarado.
    Outros,
}

impl Categoria {
    /// As seis categorias, na ordem normativa de desempate.
    pub const TODAS: [Categoria; 6] = [
        Categoria::Fila,
        Categoria::Causal,
        Categoria::Cauda,
        Categoria::Calibracao,
        Categoria::Justica,
        Categoria::Outros,
    ];

    /// O rótulo estável que acompanha a categoria em ficha, log e registro.
    pub fn nome(self) -> &'static str {
        match self {
            Categoria::Fila => "fila",
            Categoria::Causal => "causal",
            Categoria::Cauda => "cauda",
            Categoria::Calibracao => "calibracao",
            Categoria::Justica => "justica",
            Categoria::Outros => ROTULO_OUTROS,
        }
    }

    /// Índice da categoria na ordem declarada.
    pub fn indice(self) -> usize {
        match self {
            Categoria::Fila => 0,
            Categoria::Causal => 1,
            Categoria::Cauda => 2,
            Categoria::Calibracao => 3,
            Categoria::Justica => 4,
            Categoria::Outros => 5,
        }
    }

    /// Recupera a categoria pelo índice declarado.
    pub fn por_indice(i: usize) -> Option<Categoria> {
        Categoria::TODAS.get(i).copied()
    }
}

/// Resultado de uma triagem: a categoria vencedora e a distribuição inteira sobre o conjunto
/// fechado, já presa em [0,01; 0,99] e somando 1.
#[derive(Debug, Clone, PartialEq)]
pub struct Triagem {
    /// Categoria vencedora (maior probabilidade; desempate pela ordem declarada).
    pub categoria: Categoria,
    /// Probabilidade da categoria vencedora, em [0,01; 0,99].
    pub probabilidade: f64,
    /// Probabilidade de cada categoria, na ordem de `Categoria::TODAS`.
    pub distribuicao: [f64; 6],
}

impl Triagem {
    /// Probabilidade de uma categoria específica.
    pub fn probabilidade_de(&self, c: Categoria) -> f64 {
        self.distribuicao[c.indice()]
    }

    /// Soma da distribuição. Deve ser 1 dentro de `1e-12`.
    pub fn soma(&self) -> f64 {
        self.distribuicao.iter().sum()
    }

    /// Quantas categorias estão fora de [0,01; 0,99]. Deve ser sempre zero.
    pub fn fora_do_intervalo(&self) -> usize {
        self.distribuicao
            .iter()
            .filter(|p| **p < PISO || **p > TETO)
            .count()
    }

    /// Linha estável para registro.
    pub fn linha(&self) -> String {
        let produtos: Vec<String> = self
            .distribuicao
            .iter()
            .map(|p| format!("{:.4}", p))
            .collect();
        format!(
            "categoria={} probabilidade={:.4} produtos=[{}] soma={:.4} fora_de[0.01;0.99]={}",
            self.categoria.nome(),
            self.probabilidade,
            produtos.join(", "),
            self.soma(),
            self.fora_do_intervalo()
        )
    }
}

/// Um tema indexado: o texto normalizado e a categoria que o declarou.
#[derive(Debug, Clone, Copy)]
struct Tema {
    texto: &'static str,
    categoria: Categoria,
}

/// Vocabulário declarado das seis categorias.
///
/// Só entra aqui o que é verificável à mão: tema curto, em minúsculas, sem acento, sem espaço.
const VOCABULARIO: [Tema; 35] = [
    // fila — espera e saturação de serviço
    Tema { texto: "fila", categoria: Categoria::Fila },
    Tema { texto: "espera", categoria: Categoria::Fila },
    Tema { texto: "atraso", categoria: Categoria::Fila },
    Tema { texto: "servico", categoria: Categoria::Fila },
    Tema { texto: "posto", categoria: Categoria::Fila },
    Tema { texto: "demanda", categoria: Categoria::Fila },
    // causal — intervenção e identificação
    Tema { texto: "causal", categoria: Categoria::Causal },
    Tema { texto: "causa", categoria: Categoria::Causal },
    Tema { texto: "efeito", categoria: Categoria::Causal },
    Tema { texto: "tratamento", categoria: Categoria::Causal },
    Tema { texto: "intervencao", categoria: Categoria::Causal },
    Tema { texto: "contrafactual", categoria: Categoria::Causal },
    Tema { texto: "identificacao", categoria: Categoria::Causal },
    // cauda — evento raro, variância, cauda pesada
    Tema { texto: "cauda", categoria: Categoria::Cauda },
    Tema { texto: "raro", categoria: Categoria::Cauda },
    Tema { texto: "extremo", categoria: Categoria::Cauda },
    Tema { texto: "variancia", categoria: Categoria::Cauda },
    Tema { texto: "volatilidade", categoria: Categoria::Cauda },
    Tema { texto: "media", categoria: Categoria::Cauda },
    // calibracao — verificação do tamanho da incerteza
    Tema { texto: "calibracao", categoria: Categoria::Calibracao },
    Tema { texto: "cobertura", categoria: Categoria::Calibracao },
    Tema { texto: "posto", categoria: Categoria::Calibracao },
    Tema { texto: "probabilidade", categoria: Categoria::Calibracao },
    Tema { texto: "brier", categoria: Categoria::Calibracao },
    Tema { texto: "platt", categoria: Categoria::Calibracao },
    // justica — quem paga pelo erro
    Tema { texto: "justica", categoria: Categoria::Justica },
    Tema { texto: "equidade", categoria: Categoria::Justica },
    Tema { texto: "subgrupo", categoria: Categoria::Justica },
    Tema { texto: "grupo", categoria: Categoria::Justica },
    Tema { texto: "paga", categoria: Categoria::Justica },
    Tema { texto: "erro", categoria: Categoria::Justica },
    Tema { texto: "etica", categoria: Categoria::Justica },
    // outros — o próprio vocabulário da ausência declarada
    Tema { texto: "outros", categoria: Categoria::Outros },
    Tema { texto: "indefinida", categoria: Categoria::Outros },
    Tema { texto: "generica", categoria: Categoria::Outros },
];

/// Palavras vazias: normalizadas, mas sem valor de tema. Nunca entram na árvore.
const VAZIAS: [&str; 27] = [
    "a", "o", "as", "os", "um", "uma", "de", "do", "da", "dos", "das", "e", "em", "no", "na",
    "nos", "nas", "que", "para", "por", "com", "sem", "ao", "se", "sobre", "como", "entre",
];

/// Triador do Sistema 1: uma árvore de sufixos construída uma vez e consultada em O(1).
#[derive(Debug, Clone)]
pub struct S1 {
    /// Sufixo indexado -> comprimento do **tema mais curto** que termina nesse sufixo.
    sufixos: BTreeMap<String, usize>,
}

impl Default for S1 {
    fn default() -> Self {
        Self::novo()
    }
}

impl S1 {
    /// Constrói o triador a partir do vocabulário declarado.
    ///
    /// A construção é canônica: percorre `VOCABULARIO` na ordem declarada e, ao inserir um
    /// sufixo repetido, conserva o **menor** comprimento — o mapa final não depende da ordem de
    /// inserção e é idêntico em toda execução.
    pub fn novo() -> S1 {
        let mut sufixos: BTreeMap<String, usize> = BTreeMap::new();

        for tema in VOCABULARIO.iter() {
            let tam = tema.texto.chars().count();
            for inicio in 0..tam {
                let sufixo: String = tema.texto.chars().skip(inicio).collect();
                if sufixo.chars().count() < TEMA_MINIMO {
                    continue;
                }
                match sufixos.get(&sufixo) {
                    Some(atual) if *atual <= tam => {}
                    _ => {
                        sufixos.insert(sufixo, tam);
                    }
                }
            }
        }

        debug_assert_eq!(
            VOCABULARIO
                .iter()
                .map(|t| t.texto.chars().count())
                .max()
                .unwrap_or(0) as f64,
            TEMA_MAIS_LONGO,
            "TEMA_MAIS_LONGO tem de ser o tema mais longo do vocabulario"
        );

        S1 { sufixos }
    }

    /// Quantos sufixos a árvore tem. Instrumento de auditoria do tamanho do índice.
    pub fn sufixos_indexados(&self) -> usize {
        self.sufixos.len()
    }

    /// Força de casamento de cada categoria para uma definição, normalizada pelo
    /// `DENOMINADOR` global. É o que o desempate lê antes da distribuição de probabilidade, e é
    /// exposto para que o empate possa ser testado diretamente.
    pub fn forcas(&self, definicao: &str) -> [f64; 6] {
        let mut forca = [0usize; 6];
        for (categoria, tema) in self.casamentos(definicao) {
            let escore = tema.chars().count();
            if escore > forca[categoria.indice()] {
                forca[categoria.indice()] = escore;
            }
        }
        let mut saida = [0.0f64; 6];
        for c in Categoria::TODAS.iter() {
            saida[c.indice()] = (forca[c.indice()] as f64) / TEMA_MAIS_LONGO;
        }
        saida
    }

    /// Todos os casamentos encontrados: (categoria, tema declarado que casou), na ordem em que
    /// aparecem no texto.
    ///
    /// Cada posição de início contribui com **no máximo um** casamento, o mais longo — é a
    /// definição de caminhada em árvore de sufixos, e é o que impede que um tema e o seu
    /// próprio sufixo contem duas vezes. O mesmo tema pode casar em posições diferentes.
    fn casamentos(&self, definicao: &str) -> Vec<(Categoria, &'static str)> {
        let unidades = normalizar(definicao);
        let mut saida: Vec<(Categoria, &'static str)> = Vec::new();

        for inicio in 0..unidades.len() {
            let mut vencedor_local: Option<&'static str> = None;
            for fim in (inicio + 1)..=unidades.len() {
                let trecho = unidades[inicio..fim].join(" ");
                match self.tema_do_sufixo(&trecho) {
                    Some(tema) => vencedor_local = Some(tema),
                    None => break,
                }
            }
            if let Some(tema) = vencedor_local {
                saida.push((self.categoria_do_tema(tema).expect("tema declarado"), tema));
            }
        }
        saida
    }

    /// O menor tema declarado que termina nesse sufixo.
    fn tema_do_sufixo(&self, sufixo: &str) -> Option<&'static str> {
        VOCABULARIO
            .iter()
            .filter(|t| t.texto.ends_with(sufixo))
            .min_by_key(|t| t.texto.chars().count())
            .map(|t| t.texto)
    }

    /// Triei uma definição em uma linha.
    ///
    /// Devolve a categoria vencedora, a sua probabilidade e a distribuição inteira sobre o
    /// conjunto fechado. Entrada vazia ou fora do vocabulário cai em `outros`, com probabilidade
    /// baixa-mas-não-zero.
    pub fn triar(&self, definicao: &str) -> Triagem {
        let forcas = self.forcas(definicao);

        let n = Categoria::TODAS.len() as f64;
        let sem_evidencia = forcas.iter().all(|f| *f <= 0.0);

        let mut bruto = [0.0f64; 6];
        for c in Categoria::TODAS.iter() {
            // O vies de ordinal carrega o desempate e entra no escore, antes da transferencia.
            let ordem = (n - c.indice() as f64) * VIES_DE_ORDINAL;
            bruto[c.indice()] = if forcas[c.indice()] > 0.0 {
                transferir(forcas[c.indice()] + ordem)
            } else if sem_evidencia {
                // Nenhum tema casou: vale a priori declarada, que ja respeita o piso.
                if *c == Categoria::Outros {
                    MASSA_SEM_EVIDENCIA
                } else {
                    PESO_SEM_CASAMENTO
                }
            } else {
                // Esta categoria nao casou, mas outra casou: fica no piso.
                PISO
            };
        }

        // Sem evidencia nenhuma, a priori declarada e a resposta: a distribuicao e ela
        // normalizada, e nao passa pelo ajuste de teto. Com evidencia, vale o ajuste.
        let distribuicao = if sem_evidencia {
            distribuir_priori(bruto)
        } else {
            distribuir(bruto)
        };

        let (indice, probabilidade) = vencedora(&distribuicao);

        Triagem {
            categoria: Categoria::TODAS[indice],
            probabilidade,
            distribuicao,
        }
    }

    /// Qual categoria declarou o tema que termina nesse texto.
    fn categoria_do_tema(&self, tema: &str) -> Option<Categoria> {
        VOCABULARIO
            .iter()
            .find(|t| t.texto == tema)
            .map(|t| t.categoria)
    }
}

/// Normaliza a definição: minúsculas, sem acento, sem pontuação, sem palavras vazias.
///
/// Determinístico e sem tabela externa: cada caractere não ASCII é transliterado por uma regra
/// fixa do português, e o que sobra fora de `[a-z0-9]` vira separador.
pub fn normalizar(definicao: &str) -> Vec<String> {
    let mut atual = String::new();
    let mut saida: Vec<String> = Vec::new();

    for ch in definicao.chars() {
        for simples in transliterar(ch).chars() {
            if simples.is_ascii_alphanumeric() {
                atual.push(simples.to_ascii_lowercase());
            } else if !atual.is_empty() {
                if !VAZIAS.contains(&atual.as_str()) {
                    saida.push(atual.clone());
                }
                atual.clear();
            }
        }
    }
    if !atual.is_empty() && !VAZIAS.contains(&atual.as_str()) {
        saida.push(atual);
    }
    saida
}

/// Transfere um caractere para a forma simples sem acento. Devolve sempre no máximo um caractere.
fn transliterar(ch: char) -> &'static str {
    match ch {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'Á' | 'À' | 'Â' | 'Ã' | 'Ä' => "a",
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => "e",
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => "i",
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => "o",
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => "u",
        'ç' | 'Ç' => "c",
        'ñ' | 'Ñ' => "n",
        c if c.is_ascii() => tabela_ascii(c),
        _ => " ",
    }
}

/// Tabela de um caractere para cada código ASCII: letra ou dígito fica como está, o resto é
/// separador. Sem dependência externa e sem ramo dependente de estado.
fn tabela_ascii(c: char) -> &'static str {
    const TABELA: [&str; 128] = construir_tabela_ascii();
    TABELA[c as usize]
}

/// Constrói a tabela ASCII em tempo de compilação.
const fn construir_tabela_ascii() -> [&'static str; 128] {
    const DIGITOS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const MAIUSCULAS: [&str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    const MINUSCULAS: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut t = [" "; 128];
    let mut i = 0usize;
    while i < 10 {
        t[48 + i] = DIGITOS[i];
        i += 1;
    }
    let mut j = 0usize;
    while j < 26 {
        t[65 + j] = MAIUSCULAS[j];
        t[97 + j] = MINUSCULAS[j];
        j += 1;
    }
    t
}

/// Clampa em [0,01; 0,99] e reprojeta a massa cortada, preservando soma 1.
///
/// O laço é determinístico e a soma é o alvo: cada rodada tira a massa excedente de quem está no
/// teto, entrega proporcionalmente a quem não está, e levanta quem ficou abaixo do piso. No
/// conjunto de seis categorias o intervalo é viável (6 x 0,01 <= 1 <= 6 x 0,99), então a
/// convergência é imediata — e o laço para por contagem, nunca por relógio.
pub fn distribuir(bruto: [f64; 6]) -> [f64; 6] {
    let n = Categoria::TODAS.len();
    assert!(
        (n as f64) * PISO <= 1.0 && (n as f64) * TETO >= 1.0,
        "intervalo inviavel para {} categorias",
        n
    );

    let soma: f64 = bruto.iter().sum();
    let mut p = [0.0f64; 6];
    for (i, valor) in p.iter_mut().enumerate() {
        *valor = if soma > 0.0 {
            bruto[i] / soma
        } else {
            1.0 / n as f64
        };
    }

    // Ajuste por transferencia exata: o que passa do teto vira sobra, a sobra e dividida
    // igualmente entre quem ainda tem folga, quem cai abaixo do piso e levantado ate o piso e o
    // que isso custa sai de quem tem mais massa. Cada rodada fecha a conta e nenhuma rodada
    // consegue empurrar uma categoria para fora do intervalo, entao a iteracao e estavel — ao
    // contrario da redistribuicao proporcional, que oscilava entre o teto e o piso.
    for _ in 0..64 {
        let mut sobra = 0.0f64;
        let mut com_folga: Vec<usize> = Vec::new();
        for (i, valor) in p.iter_mut().enumerate() {
            if *valor < PISO {
                sobra -= PISO - *valor;
                *valor = PISO;
            } else if *valor > TETO {
                sobra += *valor - TETO;
                *valor = TETO;
            }
            if *valor < TETO {
                com_folga.push(i);
            }
        }
        if sobra.abs() <= 1e-15 {
            break;
        }
        if sobra > 0.0 {
            if com_folga.is_empty() {
                break;
            }
            let cota = sobra / com_folga.len() as f64;
            for i in com_folga.iter() {
                p[*i] = (p[*i] + cota).min(TETO);
            }
        } else {
            let disponivel: f64 = p.iter().map(|v| (v - PISO).max(0.0)).sum();
            if disponivel > 0.0 {
                let cota = (-sobra) / disponivel;
                for valor in p.iter_mut() {
                    if *valor > PISO {
                        *valor -= (*valor - PISO) * cota;
                    }
                }
            }
        }
    }

    // Correção final de ponto flutuante: a massa residual vai para a categoria de **menor
    // ordinal declarado** que ainda tenha folga nos dois lados — o mesmo desempate que
    // `vencedora` usa. Determinística por construção, e é ela que faz um empate exato de peso
    // resolver a favor do menor ordinal em vez de virar moeda.
    let resto = 1.0 - p.iter().sum::<f64>();
    if resto.abs() > 1e-12 {
        for valor in p.iter_mut() {
            if *valor + resto >= PISO && *valor + resto <= TETO {
                *valor += resto;
                break;
            }
        }
    }

    p
}

/// Distribuição de uma **priori declarada**, sem evidência lexical nenhuma.
///
/// Normaliza o vetor, respeita o piso e corta no teto sem reprojetar: quando não há evidência, a
/// massa residual existe para não deixar nenhuma categoria em 0%, e devolvê-la ao teto seria
/// reintroduzir o dogma que o intervalo proíbe.
pub fn distribuir_priori(bruto: [f64; 6]) -> [f64; 6] {
    let n = Categoria::TODAS.len();
    let total: f64 = bruto.iter().sum();
    let mut p = [0.0f64; 6];
    for (i, valor) in p.iter_mut().enumerate() {
        let fracao = if total > 0.0 {
            bruto[i] / total
        } else {
            1.0 / n as f64
        };
        *valor = fracao.clamp(PISO, TETO);
    }
    p
}

/// Vencedora: maior probabilidade; empate resolvido pela ordem declarada de `Categoria::TODAS`.
fn vencedora(p: &[f64; 6]) -> (usize, f64) {
    let mut indice = 0usize;
    let mut valor = p[0];
    for (i, v) in p.iter().enumerate().skip(1) {
        if *v > valor {
            valor = *v;
            indice = i;
        }
    }
    (indice, valor)
}

/// Atalho: constrói o triador e tria uma definição.
pub fn triar(definicao: &str) -> Triagem {
    S1::novo().triar(definicao)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Definição canônica da ficha em ordem, usada em vários testes.
    const EM_ORDEM: &str = "tempo de espera na fila do posto de saude";
    /// Definição sem nenhum tema declarado.
    const DESCONHECIDA: &str = "xyzzy plugh abracadabra";

    #[test]
    fn caminho_felizes_cada_tema_acha_a_sua_categoria() {
        let s1 = S1::novo();
        let casos = [
            ("tempo de espera na fila do posto de saude", Categoria::Fila),
            ("efeito causal do tratamento sobre a renda", Categoria::Causal),
            ("cauda pesada de evento raro e extremo", Categoria::Cauda),
            ("calibracao do escore por platt com brier", Categoria::Calibracao),
            ("justica entre subgrupos e quem paga o erro", Categoria::Justica),
            ("ficha generica indefinida", Categoria::Outros),
        ];
        for (definicao, esperada) in casos {
            let t = s1.triar(definicao);
            assert_eq!(
                t.categoria,
                esperada,
                "definicao '{}' triada como {}",
                definicao,
                t.categoria.nome()
            );
        }
    }

    #[test]
    fn leitura_do_registro() {
        let s1 = S1::novo();
        for definicao in [
            EM_ORDEM,
            "efeito causal do tratamento sobre a renda",
            "cauda pesada de evento raro e extremo",
            "calibracao do escore por platt com brier",
            "justica entre subgrupos e quem paga o erro",
            DESCONHECIDA,
        ] {
            println!("{:>44} -> {}", definicao, s1.triar(definicao).linha());
        }
        println!("sufixos indexados na arvore: {}", s1.sufixos_indexados());
    }

    #[test]
    fn determinismo_mesma_entrada_mesma_saida() {
        let a = triar(EM_ORDEM);
        let b = triar(EM_ORDEM);
        assert_eq!(a, b);

        let s1 = S1::novo();
        let c = s1.triar(EM_ORDEM);
        assert_eq!(a, c);
        assert_eq!(a.linha(), c.linha());
        // E o índice também é reprodutível: duas construções, mesmo tamanho.
        assert_eq!(s1.sufixos_indexados(), S1::novo().sufixos_indexados());
    }

    #[test]
    fn nenhuma_probabilidade_em_zero_ou_um() {
        let s1 = S1::novo();
        for definicao in [
            EM_ORDEM,
            DESCONHECIDA,
            "cauda pesada",
            "justica",
            "",
            "a o de e",
            "causal causal causal causal",
        ] {
            let t = s1.triar(definicao);
            assert_eq!(t.fora_do_intervalo(), 0, "definicao '{}'", definicao);
            for p in t.distribuicao.iter() {
                assert!(*p > 0.0 && *p < 1.0, "probabilidade dogmatica {}", p);
                assert!(*p >= PISO && *p <= TETO);
            }
            // A vencedora também é estritamente menor que o teto.
            assert!(t.probabilidade < 1.0);
        }
    }

    #[test]
    fn soma_um_depois_do_ajuste() {
        let s1 = S1::novo();
        for definicao in [
            EM_ORDEM,
            DESCONHECIDA,
            "efeito causal",
            "posto",
            "",
            "grupo subgrupo paga erro etica justica",
        ] {
            let t = s1.triar(definicao);
            assert!(
                (t.soma() - 1.0).abs() < 1e-12,
                "soma {} para '{}'",
                t.soma(),
                definicao
            );
        }
    }

    #[test]
    fn entrada_desconhecida_cai_em_outros() {
        let s1 = S1::novo();
        let t = s1.triar(DESCONHECIDA);
        assert_eq!(t.categoria, Categoria::Outros);
        // A massa efetiva e o peso declarado dividido pela massa total da priori: as cinco
        // categorias tematicas ficam em 0,0256 e "outros" leva 0,8718. Valor analitico conferido.
        let esperado = MASSA_SEM_EVIDENCIA / (MASSA_SEM_EVIDENCIA + 5.0 * PESO_SEM_CASAMENTO);
        assert!(esperado > 0.5 && esperado < TETO, "priori declarada = {}", esperado);
        assert!(
            (t.probabilidade - esperado).abs() < 1e-12,
            "massa de outros {} != {}",
            t.probabilidade,
            esperado
        );
        assert!(
            t.probabilidade > PISO && t.probabilidade < TETO,
            "baixa-mas-nao-zero, e nunca dogma"
        );
        // Ela e a maior da distribuicao, e ainda assim nao e dogma.
        for c in Categoria::TODAS.iter().filter(|c| **c != Categoria::Outros) {
            assert!(t.probabilidade > t.probabilidade_de(*c));
        }
        // As cinco categorias tematicas continuam com massa positiva: o "outros" nao apaga as
        // alternativas, apenas as rebaixa.
        for c in Categoria::TODAS.iter().filter(|c| **c != Categoria::Outros) {
            assert!(t.probabilidade_de(*c) >= PISO);
        }
        // Entrada vazia segue o mesmo caminho.
        assert_eq!(s1.triar("").categoria, Categoria::Outros);
        assert_eq!(s1.triar("   ").categoria, Categoria::Outros);
    }

    #[test]
    fn caso_que_tem_de_falhar_casamento_curto_nao_vira_evidencia() {
        let s1 = S1::novo();
        let curto = s1.triar("os aa bb cc dd");
        assert_eq!(
            curto.categoria,
            Categoria::Outros,
            "trecho curto nao pode sustentar triagem tematica"
        );
        // Prova negativa do mesmo portao: sem o comprimento minimo, "os" nao casa; se casasse,
        // a energia lexical de qualquer preposicao viraria categoria.
        assert_ne!(curto.categoria, Categoria::Justica);
        assert!(TEMA_MINIMO >= 4);
    }

    #[test]
    fn caso_que_tem_de_falhar_a_ficha_com_evidencia_tematica_nao_cai_em_outros() {
        let s1 = S1::novo();
        let t = s1.triar(EM_ORDEM);
        assert_eq!(t.categoria, Categoria::Fila);
        // O mesmo portao que aceita a ficha em ordem tem de reprovar a leitura oposta.
        assert_ne!(t.categoria, Categoria::Outros);
        assert!(
            t.probabilidade >= 0.8 && t.probabilidade <= TETO,
            "a ficha em ordem tem evidencia forte e continua abaixo do teto: {}",
            t.probabilidade
        );
        assert!(
            t.probabilidade > MASSA_SEM_EVIDENCIA,
            "evidencia tematica nao pode devolver a massa do 'nao sei'"
        );
    }

    #[test]
    fn desempate_e_pela_ordem_declarada() {
        let s1 = S1::novo();
        // "posto" (5, em Fila) e "causa" (5, em Causal): com denominador global as duas forcas
        // sao 5/13, exatamente iguais.
        let definicao = "posto causa";
        let forcas = s1.forcas(definicao);
        let dif = (forcas[Categoria::Fila.indice()] - forcas[Categoria::Causal.indice()]).abs();
        assert!(dif < 1e-12, "forcas desiguais no empate, diferenca {}", dif);
        assert!((forcas[Categoria::Fila.indice()] - 5.0 / TEMA_MAIS_LONGO).abs() < 1e-12);
        assert!((forcas[Categoria::Causal.indice()] - 5.0 / TEMA_MAIS_LONGO).abs() < 1e-12);
        // O empate nao vira empate de probabilidade: o resto da massa vai para a categoria de
        // menor ordinal declarado, e a escolha e estavel em toda execucao.
        let t = s1.triar(definicao);
        assert_eq!(
            t.categoria,
            Categoria::Fila,
            "empate resolvido pelo menor ordinal declarado (Fila antes de Causal)"
        );
        assert!(
            t.probabilidade_de(Categoria::Fila) > t.probabilidade_de(Categoria::Causal),
            "a massa residual tambem vai para o menor ordinal"
        );
        assert_eq!(t, s1.triar(definicao));
    }

    #[test]
    fn sufixo_longo_vence_tema_curto() {
        let s1 = S1::novo();
        // "retroativas" termina em "ativas", que e sufixo de "contrafactual"? Nao: o casamento
        // valido e "tratamento" (10) na categoria causal.
        let t = s1.triar("efeitos retroativas do tratamento");
        assert_eq!(t.categoria, Categoria::Causal);
        assert!(s1.sufixos_indexados() > 0);
        // O divisor e global, entao o escore nunca passa de 1 e a ficha fica longe do teto.
        let forcas = s1.forcas("efeitos retroativas do tratamento");
        assert!((forcas[Categoria::Causal.indice()] - 10.0 / TEMA_MAIS_LONGO).abs() < 1e-12);
        assert!(t.probabilidade_de(Categoria::Causal) <= TETO);
        assert!(forcas[Categoria::Causal.indice()] < 1.0);
    }

    #[test]
    fn normalizacao_e_estavel() {
        assert_eq!(normalizar("Causal!"), vec!["causal".to_string()]);
        assert_eq!(normalizar("calibração"), vec!["calibracao".to_string()]);
        assert_eq!(
            normalizar("A FILA do Posto"),
            vec!["fila".to_string(), "posto".to_string()]
        );
        // Palavras vazias somem, nao importa quantas vezes apareçam.
        assert_eq!(normalizar("de de de"), Vec::<String>::new());
    }

    #[test]
    fn reprojecao_do_teto_preserva_a_soma() {
        // Um bruto degenerado: uma categoria com massa enorme, as outras quase nulas.
        let p = distribuir([1000.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!(p[0] <= TETO && p[0] >= PISO);
        for valor in p.iter() {
            assert!(*valor >= PISO && *valor <= TETO);
        }
        // O intervalo e ponto fixo: distribuir de novo nao muda nada.
        let q = distribuir(p);
        for i in 0..6 {
            assert!((p[i] - q[i]).abs() < 1e-12, "ponto fixo quebrado no indice {}", i);
        }
    }
}
