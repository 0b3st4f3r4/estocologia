//! Integration test of the engine: the three layers in the same S1 -> S2 -> S1 cycle.

use stochology_hypergraph::Lens;
use stochology_wave::complex::Complex;
use stochology_wave::gate::{Scenario, verify};
use stochology_wave::magnetic::{Incidence, MagneticHyperedge, MagneticHypergraph};
use stochology_wave::plasticity::{PlasticityConfig, VerdictS2, consolidate};
use stochology_wave::spike::{Note, Spike, StdpBus, StdpConfig, Word, index};

fn hypergraph(destructive: bool) -> MagneticHypergraph {
    MagneticHypergraph::new(
        vec!["V0_WORLD", "V1_THESIS", "V2_ANTITHESIS", "V3_AGENT"],
        vec![
            MagneticHyperedge::new(
                "H0_DIFFUSION",
                1.0,
                vec![
                    Incidence::new(0, 1.0, 0.0),
                    Incidence::new(1, 1.0, 0.0),
                    Incidence::new(2, 1.0, 0.0),
                ],
            ),
            MagneticHyperedge::new(
                "H1_THESIS_PATH",
                1.0,
                vec![Incidence::new(1, 1.0, 0.0), Incidence::new(3, 1.0, 0.0)],
            ),
            MagneticHyperedge::new(
                "H2_ANTITHESIS_PATH",
                1.0,
                vec![
                    Incidence::new(2, 1.0, 0.0),
                    Incidence::new(3, 1.0, if destructive { 1.0 } else { 0.0 }),
                ],
            ),
        ],
        0.5,
    )
}

fn pulse() -> Vec<Complex> {
    vec![
        Complex::polar(1.0, 0.0),
        Complex::ZERO,
        Complex::ZERO,
        Complex::ZERO,
    ]
}

fn cascade(base: u64) -> Vec<Spike> {
    let lenses = [
        Lens::Morphonomy,
        Lens::Fluxonomy,
        Lens::Chrononomy,
        Lens::Scalonomy,
        Lens::Crisonomy,
    ];
    let times = [0u64, 10_200, 21_800, 35_100, 49_900];
    let phases = [0.10, 0.35, 0.60, 0.90, 1.20];
    let amps = [0.90, 0.88, 0.85, 0.92, 0.96];
    (0..5)
        .map(|k| {
            Spike::new(
                lenses[k],
                10 + k as u32,
                base + times[k],
                amps[k],
                phases[k],
                0xA000 + base + k as u64,
            )
        })
        .collect()
}

fn cascade_word() -> Word {
    Word {
        id: "W_CASCADE",
        meaning: "deformation, bottleneck, drift, amplification, and rupture",
        tau_us: 8_000.0,
        threshold: 0.70,
        notes: vec![
            Note::new(Lens::Morphonomy, 0, 0.0),
            Note::new(Lens::Fluxonomy, 10_000, 0.25),
            Note::new(Lens::Chrononomy, 22_000, 0.50),
            Note::new(Lens::Scalonomy, 35_000, 0.80),
            Note::new(Lens::Crisonomy, 50_000, 1.10),
        ],
    }
}

fn trained_bus() -> StdpBus {
    let cfg = StdpConfig::default();
    let mut bus = StdpBus::new(0.25);
    for ep in 0..5u64 {
        for e in cascade(ep * 200_000) {
            bus.publish(e, &cfg);
        }
    }
    bus
}

fn chain() -> Vec<(Lens, Lens)> {
    vec![
        (Lens::Morphonomy, Lens::Fluxonomy),
        (Lens::Fluxonomy, Lens::Chrononomy),
        (Lens::Chrononomy, Lens::Scalonomy),
        (Lens::Scalonomy, Lens::Crisonomy),
    ]
}

#[test]
fn full_s1_s2_s1_cycle_seals_the_engine() {
    let mut graph = hypergraph(true);
    let bus = trained_bus();

    // S1: conflict in the antiphase.
    let before = graph
        .propagate(&pulse(), 3, 0.65)
        .expect("initial propagation");
    assert!(before.state[3].modulus() < 1e-9);
    assert!(
        before.dissonant.contains(&3),
        "dissonant {:?}",
        before.dissonant
    );

    // S1 again, without plasticity, stays locked.
    let repeated = graph.propagate(&pulse(), 3, 0.65).expect("repetition");
    assert!(repeated.state[3].modulus() < 1e-9);

    // S2: verdict and plasticity.
    let verdict = VerdictS2 {
        target: 3,
        phasor: Complex::polar(0.8, before.state[1].phase()),
        polarities: vec![(1, 1.0), (2, 1.0)],
    };
    let commit = consolidate(
        &mut graph,
        &pulse(),
        3,
        0.65,
        &verdict,
        &PlasticityConfig::default(),
    )
    .expect("plastic commit");
    assert!(commit.gain_after > commit.gain_before);

    // S1 after the scar: the same input arrives at the target again.
    let after = graph
        .propagate(&pulse(), 3, 0.65)
        .expect("final propagation");
    assert!(
        after.state[3].modulus() > 0.04,
        "amplitude {}",
        after.state[3].modulus()
    );
    assert!((after.norm2 - 1.0).abs() < 1e-9);

    // Global gate.
    let scenario = Scenario {
        graph: &graph,
        pulse: pulse(),
        bus: &bus,
        chain: chain(),
        steps: 3,
        dt: 0.65,
        min_asymmetry: 0.05,
    };
    let report = verify(&scenario);
    assert!(report.passed(), "failures: {:?}", report.failures());
    assert_eq!(report.results.len(), 6);
}

#[test]
fn gate_fails_inverted_matrix() {
    let graph = hypergraph(false);
    let mut bus = trained_bus();
    let a = index(Lens::Fluxonomy);
    let b = index(Lens::Chrononomy);
    bus.conductance[a][b] = 0.05;
    bus.conductance[b][a] = 0.80;
    let scenario = Scenario {
        graph: &graph,
        pulse: pulse(),
        bus: &bus,
        chain: chain(),
        steps: 3,
        dt: 0.65,
        min_asymmetry: 0.05,
    };
    let report = verify(&scenario);
    assert!(!report.passed());
    let failures: Vec<&str> = report.failures().iter().map(|f| f.gate).collect();
    assert!(failures.contains(&"G6 causality"));
}

#[test]
fn word_recognized_in_the_real_stream_and_not_in_the_shuffled_one() {
    let bus = trained_bus();
    let word = cascade_word();
    let found = word.matches(&bus.stream).expect("cascade recognized");
    assert!(found.resonance > 0.85, "R = {}", found.resonance);

    // Same amplitudes and phases, compressed intervals: it is not the word.
    let compressed: Vec<Spike> = bus.stream[..5]
        .iter()
        .zip([0u64, 2_000, 4_000, 6_000, 8_000])
        .map(|(e, t)| Spike {
            t_us: 3_000_000 + t,
            ..*e
        })
        .collect();
    assert!(word.matches(&compressed).is_none());
}

#[test]
fn spike_has_stable_logical_size() {
    // 8 (lens) + 4 (vertex) + 8 (instant) + 8 (amplitude) + 8 (phase) + 8 (seal) = 44 bytes
    // counting the enum as a u8 with explicit repr; the contract is not to carry text.
    assert_eq!(std::mem::size_of::<Spike>(), 40);
}
