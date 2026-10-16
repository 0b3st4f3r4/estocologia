//! O ponto 16: a tese da ponte em escala, com o teste justo por fazer.
//!
//! O piloto P01 (`fundo/p01-tese/`) deu **empate no teto**: os dois braços apontaram os 8 defeitos plantados e
//! não deram nenhum alarme falso nos 4 casos limpos, e a declaração do piloto fixou que empate no teto conta como
//! **tese não sustentada**. O README do piloto diz o que falta para um teste justo: casos mais difíceis escolhidos
//! por alguém **de fora do programa**, analistas **humanos** e um juiz **humano**.
//!
//! Este módulo faz duas coisas. Primeiro, **re-deriva** o veredito do piloto das notas do juiz cego
//! (`fundo/p01-tese/juiz/notas.yaml`), em vez de repetir o número do README. Segundo, **trava a tese**: enquanto o
//! teste justo não tiver escolhedor, analistas e juiz designados, nenhuma linha do registro pode dizer que a tese
//! está sustentada — o portão reprova.
//!
//! Rodar: ~labs --ponte~

/// O registro da frente do teste justo.
pub const REGISTRO: &str = "fundo/p16-ponte/registro.tsv";
/// As notas do juiz cego do piloto.
pub const NOTAS: &str = "fundo/p01-tese/juiz/notas.yaml";

/// Uma nota do juiz: um caso, um braço, o significado da nota.
#[derive(Debug, Clone, PartialEq)]
pub struct Nota {
    pub caso: String,
    /// X ou Y — o rótulo cego; o mapa está em `juiz/mapa_XY.json`.
    pub braco: char,
    pub tipo: String,
    /// A nota do juiz: em caso com defeito, 1 = apontou o defeito; em caso limpo, 1 = alarme falso.
    pub nota: i64,
}

/// A pontuação do piloto, re-derivada das notas.
#[derive(Debug, Clone, PartialEq)]
pub struct Pontuacao {
    pub casos: usize,
    pub com_defeito: usize,
    pub limpos: usize,
    pub apontou_x: usize,
    pub apontou_y: usize,
    pub alarmes_x: usize,
    pub alarmes_y: usize,
    /// Casos em que os dois braços divergiram no significado da nota.
    pub pares_discordantes: usize,
}

/// Lê as notas do juiz do arquivo do piloto: `caso_XX:` e as duas linhas `X:` e `Y:` com campos em fluxo.
pub fn ler_notas(texto: &str) -> Vec<Nota> {
    let mut notas = Vec::new();
    let mut caso = String::new();
    for linha in texto.lines() {
        let l = linha.trim_end();
        if l.trim().is_empty() || l.trim_start().starts_with('#') {
            continue;
        }
        if !l.starts_with(' ') && l.ends_with(':') {
            caso = l.trim_end_matches(':').trim().to_string();
            continue;
        }
        let t = l.trim_start();
        let braco = if t.starts_with("X:") {
            Some('X')
        } else if t.starts_with("Y:") {
            Some('Y')
        } else {
            None
        };
        let Some(braco) = braco else { continue };
        let campo_nota = |chave: &str| -> i64 {
            let alvo = format!("{chave}:");
            match t.find(&alvo) {
                Some(i) => {
                    let resto = &t[i + alvo.len()..];
                    resto
                        .trim_start()
                        .split(|c: char| c == ',' || c == '}')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .parse::<i64>()
                        .unwrap_or(0)
                }
                None => 0,
            }
        };
        let campo = |chave: &str| -> String {
            let alvo = format!("{chave}:");
            match t.find(&alvo) {
                Some(i) => {
                    let resto = &t[i + alvo.len()..];
                    resto
                        .trim_start()
                        .split(|c: char| c == ',' || c == '}')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string()
                }
                None => String::new(),
            }
        };
        let tipo = campo("tipo");
        let _ = tipo.clone();
        notas.push(Nota { caso: caso.clone(), braco, tipo, nota: campo_nota("nota") });
    }
    notas
}

/// Pontua o piloto: quantos defeitos cada braço apontou e quantos alarmes falsos deu, mais os pares discordantes.
pub fn pontuar(notas: &[Nota]) -> Pontuacao {
    let mut por_caso: Vec<(String, i64, i64)> = Vec::new();
    for n in notas {
        match por_caso.iter_mut().find(|(c, _, _)| *c == n.caso) {
            Some(entrada) => {
                if n.braco == 'X' {
                    entrada.1 = n.nota;
                } else {
                    entrada.2 = n.nota;
                }
            }
            None => {
                let (x, y) = if n.braco == 'X' { (n.nota, 0) } else { (0, n.nota) };
                por_caso.push((n.caso.clone(), x, y));
            }
        }
    }
    let tipo_do_caso = |caso: &str| -> String {
        notas.iter().find(|n| n.caso == caso).map(|n| n.tipo.clone()).unwrap_or_default()
    };
    let mut apontou_x = 0;
    let mut apontou_y = 0;
    let mut alarmes_x = 0;
    let mut alarmes_y = 0;
    let mut com_defeito = 0;
    let mut limpos = 0;
    let mut discordantes = 0;
    for (caso, x, y) in &por_caso {
        if tipo_do_caso(caso) == "defeito" {
            com_defeito += 1;
            if *x == 1 {
                apontou_x += 1;
            }
            if *y == 1 {
                apontou_y += 1;
            }
        } else {
            limpos += 1;
            if *x == 1 {
                alarmes_x += 1;
            }
            if *y == 1 {
                alarmes_y += 1;
            }
        }
        if x != y {
            discordantes += 1;
        }
    }
    Pontuacao { casos: por_caso.len(), com_defeito, limpos, apontou_x, apontou_y, alarmes_x, alarmes_y, pares_discordantes: discordantes }
}

/// O veredito do piloto pela regra declarada antes: o empate no teto conta como tese não sustentada.
pub fn veredito_do_piloto(p: &Pontuacao) -> String {
    let teto = p.apontou_x == p.com_defeito && p.apontou_y == p.com_defeito && p.alarmes_x == 0 && p.alarmes_y == 0;
    if teto && p.pares_discordantes == 0 {
        format!(
            "empate no teto: os dois bracos apontaram {} de {} defeitos e deram {} alarmes falsos em {} casos limpos; pela regra declarada, a tese NAO e sustentada",
            p.apontou_x, p.com_defeito, p.alarmes_x, p.limpos
        )
    } else if p.pares_discordantes > 0 {
        format!(
            "ha {} par(es) discordante(s): sensibilidade {} e {} de {}, alarmes {} e {} — o piloto discrimina e o veredito sai das notas do juiz",
            p.pares_discordantes, p.apontou_x, p.apontou_y, p.com_defeito, p.alarmes_x, p.alarmes_y
        )
    } else {
        "as duas linhagens empatam fora do teto: falta poder, e o veredito nao se sustenta".to_string()
    }
}

/// Uma exigência do teste justo: o papel, o que se exige e quem já foi designado.
#[derive(Debug, Clone, PartialEq)]
pub struct Exigencia {
    pub papel: String,
    pub exigencia: String,
    pub quem: String,
    pub estado: String,
}

/// O registro da frente: prazo, mínimo de casos e os papéis.
#[derive(Debug, Clone, PartialEq)]
pub struct Ponte {
    pub carimbo: String,
    pub prazo: String,
    pub casos_minimos: usize,
    /// O veredito declarado pelo dono se a janela vencer sem os papeis: não sustentada em definitivo.
    pub veredito_se_o_prazo_vencer: String,
    pub papeis: Vec<Exigencia>,
}

/// O que o portão decide sobre a tese.
#[derive(Debug, Clone, PartialEq)]
pub struct VereditoDaPonte {
    pub papeis_designados: Vec<String>,
    pub papeis_pendentes: Vec<String>,
    pub casos_faltando: usize,
    pub status_declarado: String,
    /// Nenhuma linha pode dizer que a tese está sustentada antes do teste justo.
    pub divergencias: Vec<String>,
    pub caixa_fechada: bool,
}

/// Lê o registro da frente: comentários `# chave<TAB>valor` e a tabela por tabulação.
pub fn ler_ponte(texto: &str) -> Result<Ponte, String> {
    let mut carimbo = String::new();
    let mut prazo = String::new();
    let mut casos_minimos = 0usize;
    let mut veredito_se_o_prazo_vencer = String::new();
    let mut papeis = Vec::new();
    for linha in texto.lines() {
        let l = linha.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        if let Some(resto) = l.strip_prefix('#') {
            let campos: Vec<&str> = resto.trim().split('\t').collect();
            if campos.len() >= 2 {
                match campos[0] {
                    "registros" => carimbo = campos[1].trim().to_string(),
                    "prazo" => prazo = campos[1].trim().to_string(),
                    "casos_minimos" => casos_minimos = campos[1].trim().parse().map_err(|_| "casos_minimos nao e numero")?,
                    "veredito_se_o_prazo_vencer" => veredito_se_o_prazo_vencer = campos[1].trim().to_string(),
                    _ => {}
                }
            }
            continue;
        }
        let campos: Vec<&str> = l.split('\t').collect();
        if campos.len() < 4 || campos[0] == "papel" {
            continue;
        }
        papeis.push(Exigencia {
            papel: campos[0].to_string(),
            exigencia: campos[1].to_string(),
            quem: campos[2].to_string(),
            estado: campos[3].to_string(),
        });
    }
    if prazo.is_empty() || papeis.is_empty() || veredito_se_o_prazo_vencer.is_empty() {
        return Err("registro da ponte sem prazo, sem papeis ou sem o veredito da janela".to_string());
    }
    Ok(Ponte { carimbo, prazo, casos_minimos, veredito_se_o_prazo_vencer, papeis })
}

impl Ponte {
    /// O portão: os papéis designados, os pendentes, e as divergências (quem afirma o que não pode).
    pub fn veredito(&self) -> VereditoDaPonte {
        let mut designados = Vec::new();
        let mut pendentes = Vec::new();
        let mut casos_faltando = 0usize;
        let mut divergencias = Vec::new();
        let mut status = "nao sustentada".to_string();
        let mut tudo_designado = true;
        for p in &self.papeis {
            if p.papel == "status_da_tese" {
                status = p.estado.clone();
                continue;
            }
            if p.papel == "casos" {
                let feitos: usize = p.estado.split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0);
                casos_faltando = self.casos_minimos.saturating_sub(feitos);
                if casos_faltando > 0 {
                    tudo_designado = false;
                    pendentes.push(format!("casos ({} de {})", feitos, self.casos_minimos));
                } else {
                    designados.push(format!("casos ({} de {})", feitos, self.casos_minimos));
                }
                continue;
            }
            if p.estado == "designado" {
                designados.push(p.papel.clone());
            } else {
                tudo_designado = false;
                pendentes.push(p.papel.clone());
            }
        }
        if status != "nao sustentada" && status != "em teste" {
            divergencias.push(format!("status declarado \"{status}\" com o teste justo por fazer: a tese so muda de estado depois do resultado do juiz"));
        }
        if !tudo_designado && status == "em teste" {
            divergencias.push("status em teste sem os papeis designados e os casos no minimo".to_string());
        }
        VereditoDaPonte {
            papeis_designados: designados,
            papeis_pendentes: pendentes,
            casos_faltando,
            status_declarado: status,
            divergencias,
            caixa_fechada: !tudo_designado,
        }
    }
}

/// Um caso elegível da lista externa: o identificador e a classificação do catálogo de fora.
#[derive(Debug, Clone, PartialEq)]
pub struct CasoElegivel {
    pub id: String,
    /// "defeito" ou "limpo", pela classificação do catálogo externo.
    pub grupo: String,
}

/// A escolha mecânica dos casos: a permutação da lista pela semente declarada, na proporção declarada.
/// Se a lista externa não sustenta a proporção pedida, a escolha é **reprovada** em vez de completada
/// com o que houver — é o portão da regra de escolha.
pub fn escolher_casos(
    lista: &[CasoElegivel],
    com_defeito: usize,
    limpos: usize,
    semente: u64,
) -> Result<Vec<String>, String> {
    let mut rng = stochology_core::Rng::novo(semente);
    let mut escolhidos = Vec::new();
    for (grupo, quantos) in [("defeito", com_defeito), ("limpo", limpos)] {
        let mut candidatos: Vec<&CasoElegivel> = lista.iter().filter(|c| c.grupo == grupo).collect();
        if candidatos.len() < quantos {
            return Err(format!(
                "a lista externa nao sustenta a escolha: {quantos} casos do grupo {grupo} pedidos, {} elegiveis",
                candidatos.len()
            ));
        }
        for i in (1..candidatos.len()).rev() {
            let j = rng.uniforme(i as u64 + 1) as usize;
            candidatos.swap(i, j);
        }
        for c in candidatos.into_iter().take(quantos) {
            escolhidos.push(c.id.clone());
        }
    }
    Ok(escolhidos)
}

/// Relatório: o piloto re-derivado e o estado do teste justo.
pub fn relatar(pontuacao: Option<&Pontuacao>, ponte: Option<&Ponte>) -> String {
    let mut s = String::new();
    s.push_str("-- o piloto P01, re-derivado das notas do juiz cego --\n");
    match pontuacao {
        Some(p) => {
            s.push_str(&format!(
                "casos {} | com defeito {} | limpos {} | pares discordantes {}\n",
                p.casos, p.com_defeito, p.limpos, p.pares_discordantes
            ));
            s.push_str(&format!(
                "apontou o defeito: braco X {} de {} | braco Y {} de {}\n",
                p.apontou_x, p.com_defeito, p.apontou_y, p.com_defeito
            ));
            s.push_str(&format!(
                "alarme falso nos limpos: braco X {} de {} | braco Y {} de {}\n",
                p.alarmes_x, p.limpos, p.alarmes_y, p.limpos
            ));
            s.push_str(&format!("veredito: {}\n", veredito_do_piloto(p)));
        }
        None => s.push_str("as notas do juiz nao foram lidas\n"),
    }
    s.push_str("\n-- o teste justo --\n");
    match ponte {
        Some(p) => {
            let v = p.veredito();
            s.push_str(&format!("prazo da janela: {} | casos minimos: {}\n", p.prazo, p.casos_minimos));
            s.push_str(&format!("se a janela vencer sem os papeis: {}\n", p.veredito_se_o_prazo_vencer));
            for papel in &p.papeis {
                s.push_str(&format!("  {:<22} {:<10} {}\n", papel.papel, papel.estado, papel.exigencia));
            }
            s.push_str(&format!(
                "designados: {:?} | pendentes: {:?} | casos faltando: {}\n",
                v.papeis_designados, v.papeis_pendentes, v.casos_faltando
            ));
            if v.divergencias.is_empty() {
                s.push_str("divergencias: nenhuma — nenhuma linha afirma o que o teste justo ainda nao sustenta\n");
            } else {
                s.push_str(&format!("divergencias: REPROVADO em {:?}\n", v.divergencias));
            }
            if v.caixa_fechada {
                s.push_str(&format!(
                    "veredito: TESE NAO SUSTENTADA e caixa fechada — com {} pendente(s), a tese nao vira \"sustentada\" por nenhum caminho que nao o teste justo, e que seja ate {}\n",
                    v.papeis_pendentes.len(),
                    p.prazo
                ));
            } else {
                s.push_str("veredito: TESTE JUSTO PODE RODAR — os papeis estao designados e os casos no minimo; o status so muda com o resultado do juiz\n");
            }
        }
        None => s.push_str("o registro da frente nao foi lido\n"),
    }
    s
}

/// O laboratório: o piloto re-derivado, o estado do teste justo e as duas provas.
pub fn rodar(caminho_registro: &str, caminho_notas: &str) {
    println!("== Ponto 16 :: a tese da ponte em escala ==");
    let pontuacao = std::fs::read_to_string(caminho_notas).ok().map(|t| pontuar(&ler_notas(&t)));
    let ponte = std::fs::read_to_string(caminho_registro).ok().and_then(|t| ler_ponte(&t).ok());
    print!("{}", relatar(pontuacao.as_ref(), ponte.as_ref()));

    println!("\n-- prova negativa: um registro que diz a tese sustentada sem o teste justo --");
    let mentiroso = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\nescolhedor_dos_casos\tde fora\t-\tnao designado\nanalistas_braco_A\tdois humanos\t-\tnao designado\nanalistas_braco_B\tdois humanos\t-\tnao designado\njuiz\tum humano de fora\t-\tnao designado\ncasos\tmais dificeis\t-\t0 de 12\nstatus_da_tese\tnao sustentada ate o teste justo\t-\tsustentada\n";
    match ler_ponte(mentiroso) {
        Ok(p) => {
            let v = p.veredito();
            println!("  divergencias: {:?}", v.divergencias);
            println!("  caixa fechada: {}", v.caixa_fechada);
        }
        Err(e) => println!("  caso sintetico nao lido: {e}"),
    }

    println!("-- prova positiva: papeis designados e casos no minimo --");
    let honesto = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\nescolhedor_dos_casos\tde fora\tfulano (de fora)\tdesignado\nanalistas_braco_A\tdois humanos\tfulano e beltrano\tdesignado\nanalistas_braco_B\tdois humanos, cegos\tsicrano e beltrano\tdesignado\njuiz\tum humano de fora\tjuiz de fora\tdesignado\ncasos\tmais dificeis\tfulano\t12 de 12\nstatus_da_tese\tnao sustentada ate o resultado\t-\tem teste\n";
    match ler_ponte(honesto) {
        Ok(p) => {
            let v = p.veredito();
            println!("  divergencias: {:?} | caixa fechada: {}", v.divergencias, v.caixa_fechada);
        }
        Err(e) => println!("  caso sintetico nao lido: {e}"),
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::path::Path;

    fn ler_arquivo(rel: &str) -> String {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).parent().and_then(Path::parent).expect("raiz").to_path_buf();
        std::fs::read_to_string(raiz.join(rel)).expect("arquivo do registro")
    }

    #[test]
    fn o_piloto_re_deriva_o_empate_no_teto() {
        let p = pontuar(&ler_notas(&ler_arquivo(NOTAS)));
        assert_eq!(p.casos, 12, "os doze casos do piloto");
        assert_eq!(p.com_defeito, 8);
        assert_eq!(p.limpos, 4);
        assert_eq!((p.apontou_x, p.apontou_y), (8, 8), "os dois bracos apontaram todos os defeitos");
        assert_eq!((p.alarmes_x, p.alarmes_y), (0, 0), "nenhum alarme falso");
        assert_eq!(p.pares_discordantes, 0, "sem par discordante, o McNemar nao tem o que testar");
        let v = veredito_do_piloto(&p);
        assert!(v.starts_with("empate no teto"), "veredito: {v}");
        assert!(v.contains("NAO e sustentada"));
    }

    #[test]
    fn o_registro_do_teste_justo_declara_o_que_falta() {
        let ponte = ler_ponte(&ler_arquivo(REGISTRO)).expect("registro da ponte");
        let v = ponte.veredito();
        assert!(v.caixa_fechada, "sem escolhedor, analistas e juiz, a caixa fica fechada");
        assert_eq!(v.casos_faltando, ponte.casos_minimos);
        assert_eq!(v.status_declarado, "nao sustentada");
        assert!(v.divergencias.is_empty(), "o registro nao mente: {:?}", v.divergencias);
        assert_eq!(v.papeis_pendentes.len(), 5);
    }

    #[test]
    fn uma_tese_sustentada_sem_teste_justo_e_reprovada() {
        let mentiroso = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\njuiz\tum humano de fora\t-\tnao designado\ncasos\tmais dificeis\t-\t0 de 12\nstatus_da_tese\tnao sustentada ate o teste justo\t-\tsustentada\n";
        let v = ler_ponte(mentiroso).expect("caso sintetico").veredito();
        assert_eq!(v.divergencias.len(), 1, "{:?}", v.divergencias);
        assert!(v.caixa_fechada);
    }

    #[test]
    fn com_os_papeis_designados_o_teste_pode_rodar() {
        let honesto = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\nescolhedor_dos_casos\tde fora\tfulano\tdesignado\nanalistas_braco_A\tdois\tfulano e beltrano\tdesignado\nanalistas_braco_B\tdois\tsicrano\tdesignado\njuiz\tum de fora\tjuiz\tdesignado\ncasos\tmais dificeis\tfulano\t12 de 12\nstatus_da_tese\tnao sustentada ate o resultado\t-\tem teste\n";
        let v = ler_ponte(honesto).expect("caso sintetico").veredito();
        assert!(!v.caixa_fechada);
        assert!(v.divergencias.is_empty(), "{:?}", v.divergencias);
        assert_eq!(v.status_declarado, "em teste");
    }

    #[test]
    fn o_status_em_teste_sem_os_papeis_reprova() {
        let apressado = "# registros\t2026-10-16\n# prazo\t2027-01-14\n# casos_minimos\t12\n# veredito_se_o_prazo_vencer\tnao sustentada em definitivo\npapel\texigencia\tquem\testado\njuiz\tum de fora\t-\tnao designado\ncasos\tmais dificeis\t-\t0 de 12\nstatus_da_tese\tnao sustentada ate o resultado\t-\tem teste\n";
        let v = ler_ponte(apressado).expect("caso sintetico").veredito();
        assert_eq!(v.divergencias.len(), 1, "{:?}", v.divergencias);
    }

    // ---------------------------------------------------------------- a regra de escolha mecânica

    fn lista_de_teste(com_defeito: usize, limpos: usize) -> Vec<CasoElegivel> {
        let mut lista = Vec::new();
        for i in 0..com_defeito {
            lista.push(CasoElegivel { id: format!("defeito_{i:02}"), grupo: "defeito".to_string() });
        }
        for i in 0..limpos {
            lista.push(CasoElegivel { id: format!("limpo_{i:02}"), grupo: "limpo".to_string() });
        }
        lista
    }

    #[test]
    fn a_escolha_e_mecanica_deterministica_e_na_proporcao() {
        let lista = lista_de_teste(40, 20);
        let a = escolher_casos(&lista, 8, 4, 20270114).expect("escolha");
        let b = escolher_casos(&lista, 8, 4, 20270114).expect("escolha");
        assert_eq!(a, b, "a mesma lista e a mesma semente dao a mesma escolha");
        assert_eq!(a.len(), 12);
        let com_defeito = a.iter().filter(|id| id.starts_with("defeito")).count();
        let limpos = a.iter().filter(|id| id.starts_with("limpo")).count();
        assert_eq!((com_defeito, limpos), (8, 4), "a proporção declarada");
        let mut distintos = a.clone();
        distintos.sort();
        distintos.dedup();
        assert_eq!(distintos.len(), 12, "sem repetição");
        let c = escolher_casos(&lista, 8, 4, 20270115).expect("escolha");
        assert_ne!(a, c, "outra semente da outra escolha");
        let mut ordenada = lista.iter().map(|c| c.id.clone()).collect::<Vec<_>>();
        ordenada.sort();
        let mut escolhidos = a.clone();
        escolhidos.sort();
        assert_eq!(escolhidos.len(), 12, "a escolha sai da lista, e nao de fora dela");
        assert!(escolhidos.iter().all(|id| ordenada.contains(id)));
    }

    #[test]
    fn a_lista_que_nao_sustenta_a_proporcao_reprova() {
        let curta = lista_de_teste(5, 20);
        let erro = escolher_casos(&curta, 8, 4, 20270114).expect_err("deveria reprovar");
        assert!(erro.contains("nao sustenta a escolha"), "erro: {erro}");
        let sem_limpos = lista_de_teste(40, 2);
        assert!(escolher_casos(&sem_limpos, 8, 4, 20270114).is_err());
        let vazia: Vec<CasoElegivel> = Vec::new();
        assert!(escolher_casos(&vazia, 8, 4, 20270114).is_err());
    }

    #[test]
    fn o_registro_declara_o_veredito_se_o_prazo_vencer() {
        let ponte = ler_ponte(&ler_arquivo(REGISTRO)).expect("registro da ponte");
        assert!(
            ponte.veredito_se_o_prazo_vencer.contains("nao sustentada em definitivo"),
            "o registro carrega a decisao do dono: {}",
            ponte.veredito_se_o_prazo_vencer
        );
    }
}
