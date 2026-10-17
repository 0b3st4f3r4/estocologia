//! Teste de integracao do motor: as tres camadas no mesmo ciclo S1 -> S2 -> S1.

use stochology_hypergraph::Lente;
use stochology_wave::complexo::Complexo;
use stochology_wave::espicula::{BarramentoStdp, ConfigStdp, Espicula, Nota, Palavra, indice};
use stochology_wave::magnetico::{HiperarestaMag, HipergrafoMag, Incidencia};
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

fn palavra_cascata() -> Palavra {
    Palavra {
        id: "W_CASCATA",
        sentido: "deformacao, gargalo, desvio, amplificacao e ruptura",
        tau_us: 8_000.0,
        limiar: 0.70,
        notas: vec![
            Nota::nova(Lente::Morfonomia, 0, 0.0),
            Nota::nova(Lente::Fluxonomia, 10_000, 0.25),
            Nota::nova(Lente::Crononomia, 22_000, 0.50),
            Nota::nova(Lente::Escalonomia, 35_000, 0.80),
            Nota::nova(Lente::Crisonomia, 50_000, 1.10),
        ],
    }
}

fn barramento_treinado() -> BarramentoStdp {
    let cfg = ConfigStdp::default();
    let mut barramento = BarramentoStdp::novo(0.25);
    for ep in 0..5u64 {
        for e in cascata(ep * 200_000) {
            barramento.publica(e, &cfg);
        }
    }
    barramento
}

fn cadeia() -> Vec<(Lente, Lente)> {
    vec![
        (Lente::Morfonomia, Lente::Fluxonomia),
        (Lente::Fluxonomia, Lente::Crononomia),
        (Lente::Crononomia, Lente::Escalonomia),
        (Lente::Escalonomia, Lente::Crisonomia),
    ]
}

#[test]
fn ciclo_completo_s1_s2_s1_sela_o_motor() {
    let mut grafo = hipergrafo(true);
    let barramento = barramento_treinado();

    // S1: conflito na antifase.
    let antes = grafo
        .propaga(&pulso(), 3, 0.65)
        .expect("propagacao inicial");
    assert!(antes.estado[3].modulo() < 1e-9);
    assert!(
        antes.dissonantes.contains(&3),
        "dissonantes {:?}",
        antes.dissonantes
    );

    // S1 de novo, sem plasticidade, continua travado.
    let repetido = grafo.propaga(&pulso(), 3, 0.65).expect("repeticao");
    assert!(repetido.estado[3].modulo() < 1e-9);

    // S2: veredito e plasticidade.
    let veredito = VereditoS2 {
        alvo: 3,
        fasor: Complexo::polar(0.8, antes.estado[1].fase()),
        polaridades: vec![(1, 1.0), (2, 1.0)],
    };
    let commit = consolida(
        &mut grafo,
        &pulso(),
        3,
        0.65,
        &veredito,
        &ConfigPlasticidade::default(),
    )
    .expect("commit plastico");
    assert!(commit.ganho_depois > commit.ganho_antes);

    // S1 pos-cicatriz: a mesma entrada volta a chegar ao alvo.
    let depois = grafo.propaga(&pulso(), 3, 0.65).expect("propagacao final");
    assert!(
        depois.estado[3].modulo() > 0.04,
        "amplitude {}",
        depois.estado[3].modulo()
    );
    assert!((depois.norma2 - 1.0).abs() < 1e-9);

    // Portao global.
    let cenario = Cenario {
        grafo: &grafo,
        pulso: pulso(),
        barramento: &barramento,
        cadeia: cadeia(),
        saltos: 3,
        dt: 0.65,
        assimetria_minima: 0.05,
    };
    let relatorio = verifica(&cenario);
    assert!(relatorio.passou(), "falhas: {:?}", relatorio.falhas());
    assert_eq!(relatorio.resultados.len(), 6);
}

#[test]
fn portao_reprova_matriz_invertida() {
    let grafo = hipergrafo(false);
    let mut barramento = barramento_treinado();
    let a = indice(Lente::Fluxonomia);
    let b = indice(Lente::Crononomia);
    barramento.condutancia[a][b] = 0.05;
    barramento.condutancia[b][a] = 0.80;
    let cenario = Cenario {
        grafo: &grafo,
        pulso: pulso(),
        barramento: &barramento,
        cadeia: cadeia(),
        saltos: 3,
        dt: 0.65,
        assimetria_minima: 0.05,
    };
    let relatorio = verifica(&cenario);
    assert!(!relatorio.passou());
    let falhas: Vec<&str> = relatorio.falhas().iter().map(|f| f.portao).collect();
    assert!(falhas.contains(&"G6 causalidade"));
}

#[test]
fn palvra_reconhecida_no_fluxo_real_e_nao_no_embaralhado() {
    let barramento = barramento_treinado();
    let palavra = palavra_cascata();
    let achada = palavra
        .casa(&barramento.fluxo)
        .expect("cascata reconhecida");
    assert!(achada.ressonancia > 0.85, "R = {}", achada.ressonancia);

    // Mesmas amplitudes e fases, intervalos comprimidos: nao e a palavra.
    let comprimido: Vec<Espicula> = barramento.fluxo[..5]
        .iter()
        .zip([0u64, 2_000, 4_000, 6_000, 8_000])
        .map(|(e, t)| Espicula {
            t_us: 3_000_000 + t,
            ..*e
        })
        .collect();
    assert!(palavra.casa(&comprimido).is_none());
}

#[test]
fn espicula_tem_tamanho_logico_estavel() {
    // 8 (lente) + 4 (vertice) + 8 (instante) + 8 (amplitude) + 8 (fase) + 8 (selo) = 44 bytes
    // contando o enum como u8 com repr explicito; o contrato e nao carregar texto.
    assert_eq!(std::mem::size_of::<Espicula>(), 40);
}
