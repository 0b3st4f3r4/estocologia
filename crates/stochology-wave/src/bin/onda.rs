//! CLI do motor ondulatorio.
//!
//! Uso:
//!   cargo run --release --bin onda                    # demonstracao completa
//!   cargo run --release --bin onda -- --check         # os seis portoes, sai 1 se algum falhar
//!   cargo run --release --bin onda -- --defeito mutacao
//!   cargo run --release --bin onda -- --defeito inversao

use stochology_hypergraph::Lente;
use stochology_wave::complexo::Complexo;
use stochology_wave::espicula::{BarramentoStdp, ConfigStdp, Espicula, Nota, Palavra};
use stochology_wave::fasor::{Acoplamento, Limiares, superpoe};
use stochology_wave::magnetico::{HiperarestaMag, HipergrafoMag, Incidencia, verifica_cadeia};
use stochology_wave::plasticidade::{ConfigPlasticidade, VereditoS2, consolida};
use stochology_wave::portao::{Cenario, verifica};

fn hipergrafo(destrutivo: bool) -> HipergrafoMag {
    HipergrafoMag::novo(
        vec!["V0_MUNDO", "V1_TESE", "V2_ANTITESE", "V3_AGENTE"],
        vec![
            HiperarestaMag::nova(
                "H0_DIFUSAO",
                1.0,
                vec![
                    Incidencia::nova(0, 1.0, 0.0),
                    Incidencia::nova(1, 1.0, 0.0),
                    Incidencia::nova(2, 1.0, 0.0),
                ],
            ),
            HiperarestaMag::nova(
                "H1_VIA_TESE",
                1.0,
                vec![Incidencia::nova(1, 1.0, 0.0), Incidencia::nova(3, 1.0, 0.0)],
            ),
            HiperarestaMag::nova(
                "H2_VIA_ANTITESE",
                1.0,
                vec![
                    Incidencia::nova(2, 1.0, 0.0),
                    Incidencia::nova(3, 1.0, if destrutivo { 1.0 } else { 0.0 }),
                ],
            ),
        ],
        0.5,
    )
}

fn pulso() -> Vec<Complexo> {
    vec![
        Complexo::polar(1.0, 0.0),
        Complexo::ZERO,
        Complexo::ZERO,
        Complexo::ZERO,
    ]
}

fn cadeia() -> Vec<(Lente, Lente)> {
    vec![
        (Lente::Morfonomia, Lente::Fluxonomia),
        (Lente::Fluxonomia, Lente::Crononomia),
        (Lente::Crononomia, Lente::Escalonomia),
        (Lente::Escalonomia, Lente::Crisonomia),
    ]
}

fn vocabulario() -> Vec<Palavra> {
    vec![
        Palavra {
            id: "W_CASCATA",
            sentido: "deformacao, gargalo, desvio de ciclo, amplificacao e ruptura",
            tau_us: 8_000.0,
            limiar: 0.70,
            notas: vec![
                Nota::nova(Lente::Morfonomia, 0, 0.0),
                Nota::nova(Lente::Fluxonomia, 10_000, 0.25),
                Nota::nova(Lente::Crononomia, 22_000, 0.50),
                Nota::nova(Lente::Escalonomia, 35_000, 0.80),
                Nota::nova(Lente::Crisonomia, 50_000, 1.10),
            ],
        },
        Palavra {
            id: "W_FREIO",
            sentido: "inversao brusca de fase entre fluxo e forma",
            tau_us: 5_000.0,
            limiar: 0.70,
            notas: vec![
                Nota::nova(Lente::Fluxonomia, 0, 0.0),
                Nota::nova(Lente::Morfonomia, 6_000, std::f64::consts::PI),
            ],
        },
    ]
}

fn cascata(base: u64) -> Vec<Espicula> {
    let lentes = [
        Lente::Morfonomia,
        Lente::Fluxonomia,
        Lente::Crononomia,
        Lente::Escalonomia,
        Lente::Crisonomia,
    ];
    let tempos = [0u64, 10_200, 21_800, 35_100, 49_900];
    let fases = [0.10, 0.35, 0.60, 0.90, 1.20];
    let amps = [0.90, 0.88, 0.85, 0.92, 0.96];
    (0..5)
        .map(|k| {
            Espicula::nova(
                lentes[k],
                10 + k as u32,
                base + tempos[k],
                amps[k],
                fases[k],
                0xA000 + base + k as u64,
            )
        })
        .collect()
}

fn treina_barramento() -> BarramentoStdp {
    let cfg = ConfigStdp::default();
    let mut barramento = BarramentoStdp::novo(0.25);
    for ep in 0..5u64 {
        for e in cascata(ep * 200_000) {
            barramento.publica(e, &cfg);
        }
    }
    barramento
}

fn mostra_superposicao() {
    println!("=== 1. SUPERPOSICAO LOCAL (Phi de uma hiperaresta) ===");
    let casos = [
        (
            "em fase",
            vec![
                Acoplamento::novo(0.80, 0.10, 1.0, 0.0),
                Acoplamento::novo(0.75, 0.15, 1.0, 0.0),
            ],
        ),
        (
            "antifase",
            vec![
                Acoplamento::novo(0.90, 0.0, 1.0, 0.0),
                Acoplamento::novo(0.85, std::f64::consts::PI * 0.96, 1.0, 0.0),
            ],
        ),
    ];
    for (nome, acs) in casos {
        let campo = superpoe(&acs, &Limiares::default());
        println!(
            "  {nome:9} | |Psi| = {:.4} | E = {:.4} | E classica = {:.4} | interferencia = {:+.4} | C = {:.4} | {}",
            campo.fasor.modulo(),
            campo.energia,
            campo.energia_classica,
            campo.interferencia,
            campo.coerencia,
            campo.regime.nome()
        );
    }
}

fn mostra_propagacao(destrutivo: &HipergrafoMag, construtivo: &HipergrafoMag) {
    println!("\n=== 2. PROPAGACAO MULTI-SALTO (Cayley, 3 saltos, dt = 0.65) ===");
    for (nome, g) in [("construtivo", construtivo), ("destrutivo", destrutivo)] {
        match g.propaga(&pulso(), 3, 0.65) {
            Ok(p) => {
                let amps: Vec<String> = p
                    .estado
                    .iter()
                    .enumerate()
                    .map(|(i, c)| format!("V{i}:{:.4}@{:+.2}", c.modulo(), c.fase()))
                    .collect();
                println!(
                    "  {nome:12} | norma^2 = {:.10} | frustracao = {:.4} | estados [{}]",
                    p.norma2,
                    p.frustracao,
                    amps.join(" ")
                );
                println!(
                    "  {:12} | entradas em V3 = {:.4} | coerencia V3 = {:.4} | dissonantes {:?}",
                    "", p.entradas[3], p.coerencia_local[3], p.dissonantes
                );
            }
            Err(e) => println!("  {nome}: propagacao falhou: {e}"),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let modo = args.first().map(String::as_str).unwrap_or("--demo");
    let defeito = if modo == "--defeito" {
        args.get(1).map(String::as_str)
    } else {
        None
    };

    let construtivo = hipergrafo(false);
    let mut destrutivo = hipergrafo(true);
    let barramento = treina_barramento();

    if modo == "--demo" {
        println!("=== MOTOR ONDULATORIO DA ESTOCOLOGIA ===");
        mostra_superposicao();
        mostra_propagacao(&destrutivo, &construtivo);

        println!("\n=== 3. PLASTICIDADE KURAMOTO-OJA (Sistema 2 cicatriza a antifase) ===");
        let antes = destrutivo
            .propaga(&pulso(), 3, 0.65)
            .expect("propagacao inicial");
        let veredito = VereditoS2 {
            alvo: 3,
            fasor: Complexo::polar(0.8, antes.estado[1].fase()),
            polaridades: vec![(1, 1.0), (2, 1.0)],
        };
        match consolida(
            &mut destrutivo,
            &pulso(),
            3,
            0.65,
            &veredito,
            &ConfigPlasticidade::default(),
        ) {
            Ok(c) => println!(
                "  commit aceito | amplitude V3 {:.4} -> {:.4} | ganho {:.4} -> {:.4} | frustracao {:.4} -> {:.4} | selo 0x{:016x} -> 0x{:016x}",
                c.amplitude_antes,
                c.amplitude_depois,
                c.ganho_antes,
                c.ganho_depois,
                c.frustracao_antes,
                c.frustracao_depois,
                c.selo_antes,
                c.selo_depois
            ),
            Err(r) => println!(
                "  commit recusado: {} (ganho {:.4} -> {:.4})",
                r.motivo, r.ganho_antes, r.ganho_depois
            ),
        }

        println!("\n=== 4. ESPICULAS: VOCABULARIO E STDP ===");
        for palavra in vocabulario() {
            match palavra.casa(&barramento.fluxo) {
                Some(p) => println!(
                    "  palavra {} | R(W) = {:.4} | fase {:.4} | selo 0x{:016x}",
                    p.id, p.ressonancia, p.fase_resultante, p.selo
                ),
                None => println!("  palavra {} nao aparece no fluxo", palavra.id),
            }
        }
        for (de, para) in cadeia() {
            println!(
                "  {:11} -> {:11} | eta forward {:.4} | reverso {:.4} | assimetria {:+.4} | alfa {:+.4}",
                de.nome(),
                para.nome(),
                barramento.condutancia[stochology_wave::espicula::indice(de)]
                    [stochology_wave::espicula::indice(para)],
                barramento.condutancia[stochology_wave::espicula::indice(para)]
                    [stochology_wave::espicula::indice(de)],
                barramento.assimetria(de, para),
                barramento.alfa[stochology_wave::espicula::indice(de)]
                    [stochology_wave::espicula::indice(para)]
            );
        }
    }

    let cenario = Cenario {
        grafo: &destrutivo,
        pulso: pulso(),
        barramento: &barramento,
        cadeia: cadeia(),
        saltos: 3,
        dt: 0.65,
        assimetria_minima: 0.05,
    };

    if modo == "--demo" {
        println!("\n=== 5. PORTAO ===");
    }
    let relatorio = verifica(&cenario);
    for r in &relatorio.resultados {
        println!(
            "  [{}] {} | {}",
            if r.passou { "OK" } else { "FALHA" },
            r.portao,
            r.detalhe
        );
    }

    let mut codigo = if relatorio.passou() { 0 } else { 1 };

    match (modo, defeito) {
        ("--defeito", Some("mutacao")) => {
            println!("\n[PROVA NEGATIVA] adulterando 0.001 no ultimo estado da cadeia");
            let mut saltos = destrutivo
                .propaga_auditado(&pulso(), 3, 0.65)
                .expect("auditado");
            if let Some(s) = saltos.last_mut() {
                s.estado[3].re += 0.001;
            }
            match verifica_cadeia(&saltos) {
                Ok(()) => {
                    println!("  a cadeia aceitou a mutacao: portao quebrado");
                    codigo = 1;
                }
                Err(e) => {
                    println!("  cadeia recusou: {e}");
                    codigo = 1;
                }
            }
        }
        ("--defeito", Some("inversao")) => {
            println!("\n[PROVA NEGATIVA] invertendo um elo causal da matriz sinaptica");
            let mut corrompido = barramento.clone();
            let a = stochology_wave::espicula::indice(Lente::Fluxonomia);
            let b = stochology_wave::espicula::indice(Lente::Crononomia);
            corrompido.condutancia[a][b] = 0.05;
            corrompido.condutancia[b][a] = 0.80;
            let cenario_corrompido = Cenario {
                grafo: &destrutivo,
                pulso: pulso(),
                barramento: &corrompido,
                cadeia: cadeia(),
                saltos: 3,
                dt: 0.65,
                assimetria_minima: 0.05,
            };
            let rel = verifica(&cenario_corrompido);
            if rel.passou() {
                println!("  o portao aceitou a inversao anticausal: portao quebrado");
            } else {
                for f in rel.falhas() {
                    println!("  {}", f.detalhe);
                }
            }
            codigo = 1;
        }
        ("--defeito", Some(outro)) => {
            println!("defeito desconhecido: {outro} (use mutacao ou inversao)");
            codigo = 1;
        }
        ("--defeito", None) => {
            println!("--defeito exige um nome: mutacao ou inversao");
            codigo = 1;
        }
        _ => {}
    }

    if modo == "--demo" {
        println!("\nselo do relatorio: 0x{:016x}", relatorio.selo);
    }

    if codigo != 0 {
        std::process::exit(codigo);
    }
}
