//! Funil de quatro filtros, poda por EVSI e contador de tentativas K.
//!
//! O livro descreve o funil caro em quatro degraus (valor da informacao, poda topologica e de
//! unidade, teste severo contra o nulo, inferencia pesada e banca). Este modulo implementa o
//! degrau *barato*, que corta a ficha antes de qualquer simulacao pesada:
//!
//! 1. os **quatro filtros de cabecalho** (F1 a F4), cada um com o motivo explicito da recusa;
//! 2. a **poda por EVSI**: o exame so se paga se o valor da informacao da amostra puder cobrir o
//!    seu custo. Como nenhum exame vale mais que saber o estado, a cota superior do EVSI e o
//!    EVPI, que so depende da razao custo/perda C/L e da probabilidade do estado;
//! 3. o **contador de tentativas K**: quantas especificacoes foram tentadas para a mesma
//!    pergunta, por portao e na ordem em que foram tentadas.
//!
//! Nada aqui usa relogio, entropia do sistema ou estado global: a unica aleatoriedade e o `Rng`
//! de semente explicita, e so no teste que confere a cota por amostragem.
//!
//! **Limites declarados.** O modelo da poda e de dois atos e dois estados, com 0 < C < L; a cota
//! e exata para esse modelo, e nao para problemas com mais estados ou mais atos. O funil corta a
//! ficha pelo cabecalho, nao pelo dado: passar nos quatro filtros e condicao necessaria, nao
//! suficiente — os invariantes Phi1-Phi5 e a banca sao cobrados depois. O funil corta antes para
//! nao gastar caro com o que nem cabecalho tem.

use crate::camada::Ficha;

// --------------------------------------------------------------------- filtros

/// Os quatro filtros do funil, na ordem em que cortam a ficha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Filtro {
    /// (F1) nao ha pergunta do decisor.
    PerguntaDoDecisor,
    /// (F2) nao ha processo gerador declarado.
    ProcessoGerador,
    /// (F3) nao ha unidade independente ou modelo nulo.
    UnidadeOuNulo,
    /// (F4) nao ha acao declarada com clausula de saida.
    AcaoComSaida,
}

impl Filtro {
    /// Os quatro portoes, na ordem canonica do funil.
    pub const TODOS: [Filtro; 4] = [
        Filtro::PerguntaDoDecisor,
        Filtro::ProcessoGerador,
        Filtro::UnidadeOuNulo,
        Filtro::AcaoComSaida,
    ];

    pub fn codigo(self) -> &'static str {
        match self {
            Filtro::PerguntaDoDecisor => "F1",
            Filtro::ProcessoGerador => "F2",
            Filtro::UnidadeOuNulo => "F3",
            Filtro::AcaoComSaida => "F4",
        }
    }

    pub fn nome(self) -> &'static str {
        match self {
            Filtro::PerguntaDoDecisor => "pergunta do decisor",
            Filtro::ProcessoGerador => "processo gerador declarado",
            Filtro::UnidadeOuNulo => "unidade independente e modelo nulo",
            Filtro::AcaoComSaida => "acao declarada com clausula de saida",
        }
    }

    /// Posicao no funil, de 0 a 3: e a chave da contagem por portao.
    pub fn indice(self) -> usize {
        match self {
            Filtro::PerguntaDoDecisor => 0,
            Filtro::ProcessoGerador => 1,
            Filtro::UnidadeOuNulo => 2,
            Filtro::AcaoComSaida => 3,
        }
    }
}

/// A recusa de uma ficha: qual filtro a barrou e o motivo explicito.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recusa {
    pub filtro: Filtro,
    pub motivo: String,
}

impl Recusa {
    pub fn nova(filtro: Filtro, motivo: impl Into<String>) -> Self {
        Recusa {
            filtro,
            motivo: motivo.into(),
        }
    }

    /// Uma linha legivel, no formato do relatorio do funil.
    pub fn linha(&self) -> String {
        format!(
            "{} {}: recusada por {}",
            self.filtro.codigo(),
            self.filtro.nome(),
            self.motivo
        )
    }
}

fn vazio(texto: Option<&str>) -> bool {
    texto.map(str::trim).unwrap_or("").is_empty()
}

fn falta_pergunta_do_decisor(f: &Ficha) -> Option<Recusa> {
    if f.pergunta_do_decisor.trim().is_empty() {
        Some(Recusa::nova(
            Filtro::PerguntaDoDecisor,
            "nao ha pergunta do decisor: a ficha nao diz o que se decide",
        ))
    } else {
        None
    }
}

fn falta_processo_gerador(f: &Ficha) -> Option<Recusa> {
    let mut faltas: Vec<&str> = Vec::new();
    if vazio(f.artefato_simulavel.as_deref()) {
        faltas.push("artefato simulavel");
    }
    if f.semente.is_none() {
        faltas.push("semente fixa");
    }
    if faltas.is_empty() {
        return None;
    }
    Some(Recusa::nova(
        Filtro::ProcessoGerador,
        format!(
            "nao ha processo gerador declarado: falta {}",
            faltas.join(" e ")
        ),
    ))
}

fn falta_unidade_ou_nulo(f: &Ficha) -> Option<Recusa> {
    let mut faltas: Vec<&str> = Vec::new();
    if vazio(f.unidade_independente.as_deref()) {
        faltas.push("unidade independente");
    }
    if vazio(f.modelo_nulo.as_deref()) {
        faltas.push("modelo nulo");
    }
    if faltas.is_empty() {
        return None;
    }
    Some(Recusa::nova(
        Filtro::UnidadeOuNulo,
        format!(
            "nao ha unidade independente ou modelo nulo: falta {}",
            faltas.join(" e ")
        ),
    ))
}

fn falta_acao_com_saida(f: &Ficha) -> Option<Recusa> {
    let mut faltas: Vec<&str> = Vec::new();
    if f.acao_do_pacote.trim().is_empty() {
        faltas.push("acao declarada");
    }
    if vazio(f.clausula_de_saida.as_deref()) {
        faltas.push("clausula de saida");
    }
    if faltas.is_empty() {
        return None;
    }
    Some(Recusa::nova(
        Filtro::AcaoComSaida,
        format!(
            "nao ha acao declarada com clausula de saida: falta {}",
            faltas.join(" e ")
        ),
    ))
}

fn avaliar_filtro(filtro: Filtro, f: &Ficha) -> Option<Recusa> {
    match filtro {
        Filtro::PerguntaDoDecisor => falta_pergunta_do_decisor(f),
        Filtro::ProcessoGerador => falta_processo_gerador(f),
        Filtro::UnidadeOuNulo => falta_unidade_ou_nulo(f),
        Filtro::AcaoComSaida => falta_acao_com_saida(f),
    }
}

/// Passa a ficha pelos quatro filtros, na ordem, e para no primeiro que a recusar.
pub fn filtrar(f: &Ficha) -> Result<(), Recusa> {
    for filtro in Filtro::TODOS {
        if let Some(recusa) = avaliar_filtro(filtro, f) {
            return Err(recusa);
        }
    }
    Ok(())
}

/// Todas as recusas da ficha, uma por portao — o que cada filtro tem a dizer sobre ela.
pub fn recusas(f: &Ficha) -> Vec<Recusa> {
    Filtro::TODOS
        .iter()
        .filter_map(|filtro| avaliar_filtro(*filtro, f))
        .collect()
}

// --------------------------------------------------- poda pelo valor da informacao

/// Ato declarado no problema de dois atos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ato {
    Agir,
    NaoAgir,
}

impl Ato {
    pub fn codigo(self) -> &'static str {
        match self {
            Ato::Agir => "agir",
            Ato::NaoAgir => "nao-agir",
        }
    }
}

/// Razao custo/perda do ato: `custo` e o que se gasta agindo (inclusive sobre quem nao precisava)
/// e `perda` e o que se perde deixando passar o caso.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustoPerda {
    pub custo: f64,
    pub perda: f64,
}

impl CustoPerda {
    /// Exige 0 < custo < perda: se o custo alcanca a perda, agir nunca compensa e nao ha decisao
    /// a informar — o caso degenerado fica fora do contrato.
    pub fn nova(custo: f64, perda: f64) -> Self {
        assert!(
            custo.is_finite() && custo > 0.0,
            "custo precisa ser positivo e finito, veio {custo}"
        );
        assert!(
            perda.is_finite() && perda > custo,
            "perda precisa superar o custo ({custo}), veio {perda}"
        );
        CustoPerda { custo, perda }
    }

    pub fn razao(&self) -> f64 {
        self.custo / self.perda
    }

    /// Limiar do ato: p* = C/L. Abaixo dele o melhor ato e nao agir; a partir dele, agir.
    pub fn limiar(&self) -> f64 {
        self.razao()
    }

    /// O melhor ato antes de olhar qualquer dado. No limiar os dois empatam e a funcao escolhe
    /// agir.
    pub fn melhor_ato(&self, p: f64) -> Ato {
        if p.clamp(0.0, 1.0) >= self.limiar() {
            Ato::Agir
        } else {
            Ato::NaoAgir
        }
    }

    /// O valor esperado da informacao perfeita: o que se paga para saber o estado antes de agir.
    /// Funcao tenda: sobe com p ate o limiar C/L e desce depois dele.
    pub fn evpi(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        if p <= self.limiar() {
            p * (self.perda - self.custo)
        } else {
            self.custo * (1.0 - p)
        }
    }

    /// Cota superior do valor esperado da informacao da amostra (EVSI) de qualquer exame: nenhum
    /// exame imperfeito vale mais que saber o estado, logo EVSI <= EVPI.
    pub fn cota_superior_evsi(&self, p: f64) -> f64 {
        self.evpi(p)
    }

    /// EVSI de um exame com sensibilidade e especificidade declaradas, calculado exatamente no
    /// modelo de dois atos e dois estados.
    pub fn evsi(&self, p: f64, sensibilidade: f64, especificidade: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        let se = sensibilidade.clamp(0.0, 1.0);
        let esp = especificidade.clamp(0.0, 1.0);
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        let prob_positivo = se * p + (1.0 - esp) * (1.0 - p);
        let prob_negativo = esp * (1.0 - p) + (1.0 - se) * p;
        let ppv = if prob_positivo > 0.0 {
            se * p / prob_positivo
        } else {
            0.0
        };
        let npv = if prob_negativo > 0.0 {
            esp * (1.0 - p) / prob_negativo
        } else {
            0.0
        };
        let posterior_negativo = 1.0 - npv;
        // o ato otimo de cada resultado: agir custa C, nao agir perde L com a posterior do estado
        let ganho_positivo = (-self.custo).max(-ppv * self.perda);
        let ganho_negativo = (-self.custo).max(-posterior_negativo * self.perda);
        let com_exame = prob_positivo * ganho_positivo + prob_negativo * ganho_negativo;
        let sem_informacao = (-self.custo).max(-p * self.perda);
        (com_exame - sem_informacao).max(0.0)
    }

    /// Poda o exame quando a cota superior do EVSI nao cruza o limiar (o limiar e o custo do
    /// exame, na mesma unidade de C e de L).
    pub fn podar(&self, p: f64, limiar: f64) -> Poda {
        assert!(
            limiar.is_finite() && limiar >= 0.0,
            "o limiar precisa ser nao negativo e finito, veio {limiar}"
        );
        let p = p.clamp(0.0, 1.0);
        let cota = self.cota_superior_evsi(p);
        let (podado, motivo) = if cota <= 0.0 {
            (true, Some(MotivoDePoda::SemIncerteza))
        } else if cota <= limiar {
            (true, Some(MotivoDePoda::CotaAbaixoDoLimiar))
        } else {
            (false, None)
        };
        Poda {
            razao_custo_perda: self.razao(),
            probabilidade: p,
            cota_superior: cota,
            limiar,
            decisao_sem_informacao: self.melhor_ato(p),
            podado,
            motivo,
        }
    }
}

/// Por que o exame foi podado.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotivoDePoda {
    /// Nao ha incerteza a resolver: a probabilidade esta no extremo e o EVPI e zero.
    SemIncerteza,
    /// A cota superior da informacao da amostra nao cobre o custo do exame.
    CotaAbaixoDoLimiar,
}

impl MotivoDePoda {
    pub fn codigo(self) -> &'static str {
        match self {
            MotivoDePoda::SemIncerteza => "sem-incerteza",
            MotivoDePoda::CotaAbaixoDoLimiar => "cota-abaixo-do-limiar",
        }
    }

    pub fn texto(self) -> &'static str {
        match self {
            MotivoDePoda::SemIncerteza => {
                "nao ha incerteza a resolver: a probabilidade esta no extremo e o EVPI e zero"
            }
            MotivoDePoda::CotaAbaixoDoLimiar => {
                "a cota superior do EVSI nao cobre o limiar do exame; nenhum exame pode cruzar"
            }
        }
    }
}

/// O veredito da poda, com o motivo registrado.
#[derive(Debug, Clone, PartialEq)]
pub struct Poda {
    pub razao_custo_perda: f64,
    pub probabilidade: f64,
    pub cota_superior: f64,
    pub limiar: f64,
    pub decisao_sem_informacao: Ato,
    pub podado: bool,
    pub motivo: Option<MotivoDePoda>,
}

impl Poda {
    /// Uma linha legivel com todos os numeros da decisao.
    pub fn linha(&self) -> String {
        let veredito = match self.motivo {
            Some(m) => format!("podado por {} ({})", m.codigo(), m.texto()),
            None => "segue para o exame".to_string(),
        };
        format!(
            "razao C/L = {:.4} | p = {:.4} | cota superior do EVSI = {:.4} | limiar = {:.4} | ato sem informacao = {} | {}",
            self.razao_custo_perda,
            self.probabilidade,
            self.cota_superior,
            self.limiar,
            self.decisao_sem_informacao.codigo(),
            veredito
        )
    }
}

// ------------------------- parada otima (tau* = 0) e dominancia invariante a p

/// Politica de acao local diante de parametro latente nao-identificavel p in [0, 1].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoliticaLocal {
    /// Engajamento realista (a_R): coleta valor local V_L e preserva acoplamento a V_I.
    Realista,
    /// Ruminacao / busca sem informacao (a_U): paga custo c_R e reduz acoplamento por alfa in [0, 1).
    Ruminacao,
    /// Paralisia / esquiva (a_D): paga custo fixo de esquiva c_E.
    Paralisia,
}

impl PoliticaLocal {
    pub fn codigo(self) -> &'static str {
        match self {
            PoliticaLocal::Realista => "realista",
            PoliticaLocal::Ruminacao => "ruminacao",
            PoliticaLocal::Paralisia => "paralisia",
        }
    }
}

/// Matriz de payoff com utilidade separavel U(a, p) sobre variavel oculta p in [0, 1].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatrizDominancia {
    pub valor_intrinsico: f64,
    pub valor_local: f64,
    pub custo_ruminacao: f64,
    pub custo_esquiva: f64,
    pub acoplamento_ruminacao: f64,
}

impl MatrizDominancia {
    /// Exige V_I > 0, V_L > 0, c_R > 0, c_E > 0 e 0 <= alfa < 1.
    pub fn nova(
        valor_intrinsico: f64,
        valor_local: f64,
        custo_ruminacao: f64,
        custo_esquiva: f64,
        acoplamento_ruminacao: f64,
    ) -> Self {
        assert!(
            valor_intrinsico.is_finite() && valor_intrinsico > 0.0,
            "valor_intrinsico precisa ser positivo e finito, veio {valor_intrinsico}"
        );
        assert!(
            valor_local.is_finite() && valor_local > 0.0,
            "valor_local precisa ser positivo e finito, veio {valor_local}"
        );
        assert!(
            custo_ruminacao.is_finite() && custo_ruminacao > 0.0,
            "custo_ruminacao precisa ser positivo e finito, veio {custo_ruminacao}"
        );
        assert!(
            custo_esquiva.is_finite() && custo_esquiva > 0.0,
            "custo_esquiva precisa ser positivo e finito, veio {custo_esquiva}"
        );
        assert!(
            acoplamento_ruminacao.is_finite() && (0.0..1.0).contains(&acoplamento_ruminacao),
            "acoplamento_ruminacao precisa estar em [0, 1), veio {acoplamento_ruminacao}"
        );
        MatrizDominancia {
            valor_intrinsico,
            valor_local,
            custo_ruminacao,
            custo_esquiva,
            acoplamento_ruminacao,
        }
    }

    /// Utilidade esperada E[U(a)] para p in [0, 1].
    pub fn utilidade_esperada(&self, politica: PoliticaLocal, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        match politica {
            PoliticaLocal::Realista => p * self.valor_intrinsico + self.valor_local,
            PoliticaLocal::Ruminacao => {
                p * self.acoplamento_ruminacao * self.valor_intrinsico - self.custo_ruminacao
            }
            PoliticaLocal::Paralisia => -self.custo_esquiva,
        }
    }

    /// Margem liquida E[U(a_R)] - E[U(a_U)] = p(1 - alfa)V_I + V_L + c_R.
    pub fn margem_sobre_ruminacao(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        p * (1.0 - self.acoplamento_ruminacao) * self.valor_intrinsico
            + self.valor_local
            + self.custo_ruminacao
    }

    /// Margem liquida E[U(a_R)] - E[U(a_D)] = p V_I + V_L + c_E.
    pub fn margem_sobre_paralisia(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);
        p * self.valor_intrinsico + self.valor_local + self.custo_esquiva
    }

    /// Politica que maximiza a utilidade esperada em p.
    pub fn melhor_politica(&self, p: f64) -> PoliticaLocal {
        let u_r = self.utilidade_esperada(PoliticaLocal::Realista, p);
        let u_u = self.utilidade_esperada(PoliticaLocal::Ruminacao, p);
        let u_d = self.utilidade_esperada(PoliticaLocal::Paralisia, p);
        if u_r >= u_u && u_r >= u_d {
            PoliticaLocal::Realista
        } else if u_u >= u_d {
            PoliticaLocal::Ruminacao
        } else {
            PoliticaLocal::Paralisia
        }
    }

    /// Confere se a politica otima e invariante em todo o intervalo p in [0, 1] (da*/dp = 0).
    pub fn invariante_a_p(&self) -> bool {
        self.margem_sobre_ruminacao(0.0) > 0.0
            && self.margem_sobre_ruminacao(1.0) > 0.0
            && self.margem_sobre_paralisia(0.0) > 0.0
            && self.margem_sobre_paralisia(1.0) > 0.0
            && self.melhor_politica(0.0) == PoliticaLocal::Realista
            && self.melhor_politica(1.0) == PoliticaLocal::Realista
    }

    /// Regra de parada de Bellman (VOC = EVSI - c_R): quando o canal nao traz informacao
    /// (se + esp == 1 => Delta I_t = 0 => EVSI = 0) ou nao paga o passo, tau* = 0.
    pub fn parada_otima(
        &self,
        cp: &CustoPerda,
        p: f64,
        sensibilidade: f64,
        especificidade: f64,
    ) -> ParadaOtima {
        let p = p.clamp(0.0, 1.0);
        let se = sensibilidade.clamp(0.0, 1.0);
        let esp = especificidade.clamp(0.0, 1.0);
        let canal_sem_informacao = (se + esp - 1.0).abs() <= 1e-12;
        let evsi = if canal_sem_informacao {
            0.0
        } else {
            cp.evsi(p, se, esp)
        };
        let voc = evsi - self.custo_ruminacao;
        let tau_estrela = if voc <= 0.0 { 0 } else { 1 };
        ParadaOtima {
            probabilidade: p,
            evsi,
            custo_passo: self.custo_ruminacao,
            voc,
            canal_sem_informacao,
            tau_estrela,
            politica_dominante: self.melhor_politica(p),
        }
    }
}

/// Resultado do corte de Bellman sobre um passo de deliberacao interna.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParadaOtima {
    pub probabilidade: f64,
    pub evsi: f64,
    pub custo_passo: f64,
    pub voc: f64,
    pub canal_sem_informacao: bool,
    pub tau_estrela: usize,
    pub politica_dominante: PoliticaLocal,
}

impl ParadaOtima {
    pub fn linha(&self) -> String {
        format!(
            "p = {:.4} | EVSI = {:.4} | c_R = {:.4} | VOC = {:.4} | sem informacao = {} | tau* = {} | politica = {}",
            self.probabilidade,
            self.evsi,
            self.custo_passo,
            self.voc,
            self.canal_sem_informacao,
            self.tau_estrela,
            self.politica_dominante.codigo()
        )
    }
}

// ---------------------------------------------------- contador de tentativas K

/// Uma especificacao tentada, com o portao onde ela parou (ou nenhum, se passou).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tentativa {
    pub numero: usize,
    pub recusa: Option<Filtro>,
    pub rotulo: String,
}

impl Tentativa {
    pub fn passou(&self) -> bool {
        self.recusa.is_none()
    }
}

/// O placar honesto da busca: quantas especificacoes foram tentadas, por portao, na ordem.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContadorK {
    tentativas: Vec<Tentativa>,
    por_filtro: [usize; 4],
}

impl ContadorK {
    pub fn novo() -> Self {
        ContadorK {
            tentativas: Vec::new(),
            por_filtro: [0; 4],
        }
    }

    /// Registra uma tentativa e devolve o seu numero na sequencia.
    pub fn registrar(&mut self, rotulo: &str, recusa: Option<Filtro>) -> usize {
        let numero = self.tentativas.len() + 1;
        if let Some(filtro) = recusa {
            self.por_filtro[filtro.indice()] += 1;
        }
        self.tentativas.push(Tentativa {
            numero,
            recusa,
            rotulo: rotulo.to_string(),
        });
        numero
    }

    /// Atalho para a tentativa barrada por um portao.
    pub fn recusada(&mut self, filtro: Filtro, rotulo: &str) -> usize {
        self.registrar(rotulo, Some(filtro))
    }

    /// Atalho para a tentativa que atravessou o funil.
    pub fn aprovada(&mut self, rotulo: &str) -> usize {
        self.registrar(rotulo, None)
    }

    pub fn total(&self) -> usize {
        self.tentativas.len()
    }

    pub fn aprovadas(&self) -> usize {
        self.tentativas.iter().filter(|t| t.passou()).count()
    }

    pub fn por_filtro(&self, filtro: Filtro) -> usize {
        self.por_filtro[filtro.indice()]
    }

    pub fn sequencia(&self) -> &[Tentativa] {
        &self.tentativas
    }

    /// Fecha a conta: o relatorio declara o K e a versao final aceita.
    pub fn relatorio(&self, versao_final: Option<&str>) -> Relatorio {
        Relatorio {
            total: self.total(),
            por_filtro: self.por_filtro,
            aprovadas: self.aprovadas(),
            sequencia: self.tentativas.clone(),
            versao_final: versao_final.map(str::to_string),
        }
    }
}

/// O que o contador devolve: o total, a conta por portao, a sequencia e a versao final.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relatorio {
    pub total: usize,
    /// Barradas por filtro, na ordem F1 a F4.
    pub por_filtro: [usize; 4],
    pub aprovadas: usize,
    pub sequencia: Vec<Tentativa>,
    pub versao_final: Option<String>,
}

impl Relatorio {
    /// Uma linha legivel, no formato K = ... .
    pub fn linha(&self) -> String {
        let por_portao: Vec<String> = Filtro::TODOS
            .iter()
            .map(|f| format!("{} {}", f.codigo(), self.por_filtro[f.indice()]))
            .collect();
        let versao = self.versao_final.as_deref().unwrap_or("nao declarada");
        format!(
            "K = {} tentativas ({}); aprovadas {}; versao final: {}",
            self.total,
            por_portao.join(", "),
            self.aprovadas,
            versao
        )
    }
}

// ------------------------------------------------------------------- o funil

/// O funil inteiro: os quatro filtros e o contador K no mesmo lugar.
#[derive(Debug, Clone, Default)]
pub struct Funil {
    contador: ContadorK,
}

impl Funil {
    pub fn novo() -> Self {
        Funil {
            contador: ContadorK::novo(),
        }
    }

    /// Tenta a ficha: devolve a recusa e conta a tentativa no portao que a barrou — ou a conta
    /// como aprovada quando os quatro filtros deixam passar.
    pub fn tentar(&mut self, rotulo: &str, f: &Ficha) -> Result<(), Recusa> {
        match filtrar(f) {
            Ok(()) => {
                self.contador.aprovada(rotulo);
                Ok(())
            }
            Err(recusa) => {
                self.contador.recusada(recusa.filtro, rotulo);
                Err(recusa)
            }
        }
    }

    pub fn contador(&self) -> &ContadorK {
        &self.contador
    }

    /// Fecha a busca declarando a versao final.
    pub fn fechar(&self, versao_final: &str) -> Relatorio {
        self.contador.relatorio(Some(versao_final))
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::camada::{Camada, Ficha, TipoIncerteza};
    use crate::rng::Rng;

    /// A ficha completa: passa nos quatro filtros e tambem nos invariantes Phi1-Phi4.
    fn ficha_completa() -> Ficha {
        let mut f = Ficha::nova("funil.1", Camada::Modelo, TipoIncerteza::Epistemica).completa();
        f.definicao = "servico do dia de pico com o plano de contingencia".to_string();
        f.pergunta_do_decisor = "devemos acionar o plano de contingencia hoje?".to_string();
        f.acao_do_pacote = "acionar o plano de contingencia".to_string();
        f
    }

    #[test]
    fn ficha_completa_passa_pelos_quatro_filtros() {
        let f = ficha_completa();
        assert!(filtrar(&f).is_ok(), "ficha completa devia passar");
        assert!(recusas(&f).is_empty());
        // o cabecalho que os quatro filtros leem esta inteiro na ficha
        assert!(!f.pergunta_do_decisor.trim().is_empty());
        assert!(!f.acao_do_pacote.trim().is_empty());
        assert!(f.artefato_simulavel.is_some() && f.semente.is_some());
        assert!(f.unidade_independente.is_some() && f.modelo_nulo.is_some());
        assert!(f.clausula_de_saida.is_some());
    }

    #[test]
    fn f1_recusa_ficha_sem_pergunta_do_decisor() {
        let mut f = ficha_completa();
        f.pergunta_do_decisor = "   ".to_string();
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::PerguntaDoDecisor);
        assert_eq!(recusa.filtro.codigo(), "F1");
        assert!(
            recusa.motivo.contains("pergunta do decisor"),
            "{}",
            recusa.motivo
        );
        assert_eq!(recusas(&f).len(), 1);
    }

    #[test]
    fn f2_recusa_ficha_sem_processo_gerador() {
        let mut f = ficha_completa();
        f.artefato_simulavel = None;
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::ProcessoGerador);
        assert!(
            recusa.motivo.contains("artefato simulavel"),
            "{}",
            recusa.motivo
        );
    }

    #[test]
    fn f2_recusa_tambem_ficha_sem_semente() {
        let mut f = ficha_completa();
        f.semente = None;
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::ProcessoGerador);
        assert!(recusa.motivo.contains("semente fixa"), "{}", recusa.motivo);
    }

    #[test]
    fn f3_recusa_ficha_sem_unidade_independente() {
        let mut f = ficha_completa();
        f.unidade_independente = None;
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::UnidadeOuNulo);
        assert!(
            recusa.motivo.contains("unidade independente"),
            "{}",
            recusa.motivo
        );
    }

    #[test]
    fn f3_recusa_ficha_sem_modelo_nulo() {
        let mut f = ficha_completa();
        f.modelo_nulo = None;
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::UnidadeOuNulo);
        assert!(recusa.motivo.contains("modelo nulo"), "{}", recusa.motivo);
    }

    #[test]
    fn f4_recusa_ficha_sem_acao_declarada() {
        let mut f = ficha_completa();
        f.acao_do_pacote = String::new();
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::AcaoComSaida);
        assert!(
            recusa.motivo.contains("acao declarada"),
            "{}",
            recusa.motivo
        );
    }

    #[test]
    fn f4_recusa_ficha_sem_clausula_de_saida() {
        let mut f = ficha_completa();
        f.clausula_de_saida = None;
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::AcaoComSaida);
        assert!(
            recusa.motivo.contains("clausula de saida"),
            "{}",
            recusa.motivo
        );
    }

    #[test]
    fn o_funil_para_no_primeiro_portao() {
        let mut f = ficha_completa();
        f.pergunta_do_decisor = String::new();
        f.artefato_simulavel = None;
        f.unidade_independente = None;
        f.acao_do_pacote = String::new();
        let todas = recusas(&f);
        assert_eq!(todas.len(), 4, "os quatro filtros tinham o que recusar");
        assert_eq!(todas[0].filtro, Filtro::PerguntaDoDecisor);
        let recusa = filtrar(&f).expect_err("devia recusar");
        assert_eq!(recusa.filtro, Filtro::PerguntaDoDecisor, "para no primeiro");
    }

    #[test]
    fn a_poda_dispara_quando_o_evsi_nao_cobre_o_limiar() {
        let cp = CustoPerda::nova(1.0, 10.0);
        assert!((cp.limiar() - 0.1).abs() < 1e-12);
        let poda = cp.podar(0.001, 0.05);
        assert!(poda.podado, "{}", poda.linha());
        assert_eq!(poda.motivo, Some(MotivoDePoda::CotaAbaixoDoLimiar));
        assert!(
            (poda.cota_superior - 0.009).abs() < 1e-12,
            "cota {:.6}",
            poda.cota_superior
        );
        assert_eq!(poda.decisao_sem_informacao, Ato::NaoAgir);
    }

    #[test]
    fn a_poda_nao_dispara_quando_o_evsi_cobre() {
        let cp = CustoPerda::nova(1.0, 10.0);
        // no proprio limiar a cota e maxima: C(1 - C/L) = 0,9
        let poda = cp.podar(0.1, 0.05);
        assert!(!poda.podado, "{}", poda.linha());
        assert_eq!(poda.motivo, None);
        assert!(
            (poda.cota_superior - 0.9).abs() < 1e-12,
            "cota {:.6}",
            poda.cota_superior
        );
        assert_eq!(poda.decisao_sem_informacao, Ato::Agir);
        // longe do limiar, mas ainda cobrindo: p = 0,3 da cota 0,7
        let outra = cp.podar(0.3, 0.5);
        assert!(!outra.podado);
        assert!((outra.cota_superior - 0.7).abs() < 1e-12);
    }

    #[test]
    fn a_poda_declara_sem_incerteza_no_extremo() {
        let cp = CustoPerda::nova(1.0, 10.0);
        let poda = cp.podar(1.0, 0.0);
        assert!(poda.podado);
        assert_eq!(poda.motivo, Some(MotivoDePoda::SemIncerteza));
        assert!(poda.cota_superior.abs() < 1e-15);
    }

    #[test]
    fn a_cota_superior_limita_o_evsi_de_qualquer_exame() {
        let cp = CustoPerda::nova(1.0, 10.0);
        let acuracidades = [0.50, 0.70, 0.90, 0.99];
        let mut pior_folga = f64::MIN;
        for p in [0.02, 0.05, 0.10, 0.30, 0.60, 0.90] {
            let cota = cp.cota_superior_evsi(p);
            for se in acuracidades {
                for esp in acuracidades {
                    let v = cp.evsi(p, se, esp);
                    assert!(v >= -1e-12, "EVSI negativo: {v:.6}");
                    assert!(
                        v <= cota + 1e-12,
                        "EVSI {v:.6} passou da cota {cota:.6} em p={p}, se={se}, esp={esp}"
                    );
                    pior_folga = pior_folga.max(v - cota);
                }
            }
            // o exame perfeito gasta a cota inteira
            let perfeito = cp.evsi(p, 1.0, 1.0);
            assert!(
                (perfeito - cota).abs() < 1e-12,
                "exame perfeito {perfeito:.6} contra cota {cota:.6}"
            );
        }
        assert!(
            pior_folga <= 0.0,
            "nenhum exame passou da cota, pior folga {pior_folga:.6}"
        );
    }

    #[test]
    fn exame_mais_acurado_nao_vale_menos() {
        let cp = CustoPerda::nova(2.0, 10.0);
        let p = 0.4;
        let fraco = cp.evsi(p, 0.60, 0.60);
        let medio = cp.evsi(p, 0.80, 0.80);
        let forte = cp.evsi(p, 0.95, 0.95);
        assert!(fraco <= medio + 1e-12, "{fraco:.6} > {medio:.6}");
        assert!(medio <= forte + 1e-12, "{medio:.6} > {forte:.6}");
        assert!(
            forte < cp.cota_superior_evsi(p),
            "exame imperfeito nao gasta a cota"
        );
    }

    #[test]
    fn evsi_medido_bate_com_o_exato() {
        let cp = CustoPerda::nova(1.0, 10.0);
        let (p, se, esp) = (0.30_f64, 0.90_f64, 0.70_f64);
        let limiar = cp.limiar();
        let n = 200_000_usize;
        let mut rng = Rng::novo(2026);
        let mut soma_informado = 0.0_f64;
        let mut soma_fixo = 0.0_f64;
        let agir_fixo = cp.melhor_ato(p);
        for _ in 0..n {
            let u = (rng.proximo() >> 11) as f64 / (1u64 << 53) as f64;
            let doente = u < p;
            let v = (rng.proximo() >> 11) as f64 / (1u64 << 53) as f64;
            let positivo = if doente { v < se } else { v >= esp };
            let posterior = if positivo {
                se * p / (se * p + (1.0 - esp) * (1.0 - p))
            } else {
                let npv = esp * (1.0 - p) / (esp * (1.0 - p) + (1.0 - se) * p);
                1.0 - npv
            };
            let agir = posterior >= limiar;
            let pago_informado = if agir {
                -cp.custo
            } else if doente {
                -cp.perda
            } else {
                0.0
            };
            let pago_fixo = match agir_fixo {
                Ato::Agir => -cp.custo,
                Ato::NaoAgir => {
                    if doente {
                        -cp.perda
                    } else {
                        0.0
                    }
                }
            };
            soma_informado += pago_informado;
            soma_fixo += pago_fixo;
        }
        let medido = (soma_informado - soma_fixo) / n as f64;
        let exato = cp.evsi(p, se, esp);
        let cota = cp.cota_superior_evsi(p);
        assert!(
            (medido - exato).abs() < 0.02,
            "medido {medido:.4} contra exato {exato:.4} (cota {cota:.4})"
        );
        assert!(
            medido <= cota + 0.02,
            "medido {medido:.4} passou da cota {cota:.4}"
        );
    }

    #[test]
    fn o_contador_soma_por_portao_e_guarda_a_sequencia() {
        let mut k = ContadorK::novo();
        assert_eq!(k.total(), 0);
        assert_eq!(
            k.relatorio(None).linha(),
            "K = 0 tentativas (F1 0, F2 0, F3 0, F4 0); aprovadas 0; versao final: nao declarada"
        );
        assert_eq!(k.recusada(Filtro::PerguntaDoDecisor, "v1"), 1);
        assert_eq!(k.recusada(Filtro::AcaoComSaida, "v2"), 2);
        assert_eq!(k.recusada(Filtro::AcaoComSaida, "v3"), 3);
        assert_eq!(k.aprovada("v4"), 4);
        assert_eq!(k.total(), 4);
        assert_eq!(k.por_filtro(Filtro::PerguntaDoDecisor), 1);
        assert_eq!(k.por_filtro(Filtro::ProcessoGerador), 0);
        assert_eq!(k.por_filtro(Filtro::AcaoComSaida), 2);
        assert_eq!(k.aprovadas(), 1);
        let rotulos: Vec<&str> = k.sequencia().iter().map(|t| t.rotulo.as_str()).collect();
        assert_eq!(rotulos, vec!["v1", "v2", "v3", "v4"]);
        assert!(k.sequencia()[3].passou());
        assert_eq!(k.sequencia()[0].recusa, Some(Filtro::PerguntaDoDecisor));
        assert_eq!(k.sequencia()[1].numero, 2);
        let r = k.relatorio(Some("v4"));
        assert_eq!(r.total, 4);
        assert_eq!(r.por_filtro, [1, 0, 0, 2]);
        assert_eq!(r.aprovadas, 1);
        assert_eq!(r.sequencia.len(), 4);
        assert_eq!(
            r.linha(),
            "K = 4 tentativas (F1 1, F2 0, F3 0, F4 2); aprovadas 1; versao final: v4"
        );
    }

    #[test]
    fn o_funil_conta_as_tentativas_ate_a_versao_final() {
        let mut funil = Funil::novo();
        let mut sem_pergunta = ficha_completa();
        sem_pergunta.pergunta_do_decisor = String::new();
        let mut sem_saida = ficha_completa();
        sem_saida.clausula_de_saida = None;
        let final_ = ficha_completa();

        assert!(funil.tentar("v1", &sem_pergunta).is_err());
        assert!(funil.tentar("v2", &sem_saida).is_err());
        assert!(funil.tentar("v3", &final_).is_ok());
        assert_eq!(funil.contador().total(), 3);
        let r = funil.fechar("v3");
        assert_eq!(r.total, 3);
        assert_eq!(r.aprovadas, 1);
        assert_eq!(
            r.linha(),
            "K = 3 tentativas (F1 1, F2 0, F3 0, F4 1); aprovadas 1; versao final: v3"
        );
    }

    #[test]
    fn canal_sem_informacao_zera_evsi_e_impoe_parada_imediata() {
        let cp = CustoPerda::nova(1.0, 10.0);
        let m = MatrizDominancia::nova(5.0, 2.0, 0.25, 1.50, 0.30);
        for p in [0.0, 0.01, 0.10, 0.30, 0.50, 0.90, 0.99, 1.0] {
            for (se, esp) in [(0.50, 0.50), (0.80, 0.20), (0.10, 0.90)] {
                let evsi = cp.evsi(p, se, esp);
                assert!(
                    evsi.abs() < 1e-12,
                    "canal com se+esp=1 devia ter EVSI=0, veio {evsi:.6} em p={p}"
                );
                let corte = m.parada_otima(&cp, p, se, esp);
                assert!(corte.canal_sem_informacao, "{}", corte.linha());
                assert!(corte.evsi.abs() < 1e-12);
                assert!((corte.voc - (-0.25)).abs() < 1e-12);
                assert_eq!(corte.tau_estrela, 0);
                assert_eq!(corte.politica_dominante, PoliticaLocal::Realista);
            }
        }
    }

    #[test]
    fn dominancia_estrita_invariante_a_probabilidade_oculta() {
        let m = MatrizDominancia::nova(4.0, 1.5, 0.40, 1.20, 0.25);
        assert!(m.invariante_a_p());
        // no pior caso (p = 0), agir ainda supera ruminar por V_L + c_R e paralisia por V_L + c_E
        assert!((m.margem_sobre_ruminacao(0.0) - 1.90).abs() < 1e-12);
        assert!((m.margem_sobre_paralisia(0.0) - 2.70).abs() < 1e-12);
        for k in 0..=100 {
            let p = k as f64 / 100.0;
            assert_eq!(m.melhor_politica(p), PoliticaLocal::Realista);
            let u_r = m.utilidade_esperada(PoliticaLocal::Realista, p);
            let u_u = m.utilidade_esperada(PoliticaLocal::Ruminacao, p);
            let u_d = m.utilidade_esperada(PoliticaLocal::Paralisia, p);
            assert!((u_r - u_u - m.margem_sobre_ruminacao(p)).abs() < 1e-12);
            assert!((u_r - u_d - m.margem_sobre_paralisia(p)).abs() < 1e-12);
            assert!(u_r > u_u && u_r > u_d);
        }
    }
}
