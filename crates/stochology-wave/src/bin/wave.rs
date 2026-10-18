//! CLI of the wave engine.
//!
//! Usage:
//!   cargo run --release --bin wave                    # full demonstration
//!   cargo run --release --bin wave -- --check         # the six gates, exits 1 if any fails
//!   cargo run --release --bin wave -- --defect mutation
//!   cargo run --release --bin wave -- --defect inversion

use stochology_hypergraph::Lens;
use stochology_wave::complex::Complex;
use stochology_wave::gate::{Scenario, verify};
use stochology_wave::magnetic::{Incidence, MagneticHyperedge, MagneticHypergraph, verify_chain};
use stochology_wave::phasor::{Coupling, Thresholds, superpose};
use stochology_wave::plasticity::{PlasticityConfig, VerdictS2, consolidate};
use stochology_wave::spike::{Note, Spike, StdpBus, StdpConfig, Word};

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

fn chain() -> Vec<(Lens, Lens)> {
    vec![
        (Lens::Morphonomy, Lens::Fluxonomy),
        (Lens::Fluxonomy, Lens::Chrononomy),
        (Lens::Chrononomy, Lens::Scalonomy),
        (Lens::Scalonomy, Lens::Crisonomy),
    ]
}

fn vocabulary() -> Vec<Word> {
    vec![
        Word {
            id: "W_CASCADE",
            meaning: "deformation, bottleneck, cycle drift, amplification, and rupture",
            tau_us: 8_000.0,
            threshold: 0.70,
            notes: vec![
                Note::new(Lens::Morphonomy, 0, 0.0),
                Note::new(Lens::Fluxonomy, 10_000, 0.25),
                Note::new(Lens::Chrononomy, 22_000, 0.50),
                Note::new(Lens::Scalonomy, 35_000, 0.80),
                Note::new(Lens::Crisonomy, 50_000, 1.10),
            ],
        },
        Word {
            id: "W_BRAKE",
            meaning: "abrupt phase inversion between flux and form",
            tau_us: 5_000.0,
            threshold: 0.70,
            notes: vec![
                Note::new(Lens::Fluxonomy, 0, 0.0),
                Note::new(Lens::Morphonomy, 6_000, std::f64::consts::PI),
            ],
        },
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

fn train_bus() -> StdpBus {
    let cfg = StdpConfig::default();
    let mut bus = StdpBus::new(0.25);
    for ep in 0..5u64 {
        for e in cascade(ep * 200_000) {
            bus.publish(e, &cfg);
        }
    }
    bus
}

fn show_superposition() {
    println!("=== 1. LOCAL SUPERPOSITION (Phi of a hyperedge) ===");
    let cases = [
        (
            "in phase",
            vec![
                Coupling::new(0.80, 0.10, 1.0, 0.0),
                Coupling::new(0.75, 0.15, 1.0, 0.0),
            ],
        ),
        (
            "antiphase",
            vec![
                Coupling::new(0.90, 0.0, 1.0, 0.0),
                Coupling::new(0.85, std::f64::consts::PI * 0.96, 1.0, 0.0),
            ],
        ),
    ];
    for (name, cs) in cases {
        let field = superpose(&cs, &Thresholds::default());
        println!(
            "  {name:9} | |Psi| = {:.4} | E = {:.4} | classical E = {:.4} | interference = {:+.4} | C = {:.4} | {}",
            field.phasor.modulus(),
            field.energy,
            field.classical_energy,
            field.interference,
            field.coherence,
            field.regime.name()
        );
    }
}

fn show_propagation(destructive: &MagneticHypergraph, constructive: &MagneticHypergraph) {
    println!("\n=== 2. MULTI-STEP PROPAGATION (Cayley, 3 steps, dt = 0.65) ===");
    for (name, g) in [("constructive", constructive), ("destructive", destructive)] {
        match g.propagate(&pulse(), 3, 0.65) {
            Ok(p) => {
                let amps: Vec<String> = p
                    .state
                    .iter()
                    .enumerate()
                    .map(|(i, c)| format!("V{i}:{:.4}@{:+.2}", c.modulus(), c.phase()))
                    .collect();
                println!(
                    "  {name:12} | norm^2 = {:.10} | frustration = {:.4} | states [{}]",
                    p.norm2,
                    p.frustration,
                    amps.join(" ")
                );
                println!(
                    "  {:12} | incoming at V3 = {:.4} | coherence V3 = {:.4} | dissonant {:?}",
                    "", p.incoming[3], p.local_coherence[3], p.dissonant
                );
            }
            Err(e) => println!("  {name}: propagation failed: {e}"),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("--demo");
    let defect = if mode == "--defect" {
        args.get(1).map(String::as_str)
    } else {
        None
    };

    let constructive = hypergraph(false);
    let mut destructive = hypergraph(true);
    let bus = train_bus();

    if mode == "--demo" {
        println!("=== WAVE ENGINE OF STOCHOLOGY ===");
        show_superposition();
        show_propagation(&destructive, &constructive);

        println!("\n=== 3. KURAMOTO-OJA PLASTICITY (System 2 heals the antiphase) ===");
        let before = destructive
            .propagate(&pulse(), 3, 0.65)
            .expect("initial propagation");
        let verdict = VerdictS2 {
            target: 3,
            phasor: Complex::polar(0.8, before.state[1].phase()),
            polarities: vec![(1, 1.0), (2, 1.0)],
        };
        match consolidate(
            &mut destructive,
            &pulse(),
            3,
            0.65,
            &verdict,
            &PlasticityConfig::default(),
        ) {
            Ok(c) => println!(
                "  commit accepted | amplitude V3 {:.4} -> {:.4} | gain {:.4} -> {:.4} | frustration {:.4} -> {:.4} | seal 0x{:016x} -> 0x{:016x}",
                c.amplitude_before,
                c.amplitude_after,
                c.gain_before,
                c.gain_after,
                c.frustration_before,
                c.frustration_after,
                c.seal_before,
                c.seal_after
            ),
            Err(r) => println!(
                "  commit refused: {} (gain {:.4} -> {:.4})",
                r.reason, r.gain_before, r.gain_after
            ),
        }

        println!("\n=== 4. SPIKES: VOCABULARY AND STDP ===");
        for word in vocabulary() {
            match word.matches(&bus.stream) {
                Some(p) => println!(
                    "  word {} | R(W) = {:.4} | phase {:.4} | seal 0x{:016x}",
                    p.id, p.resonance, p.resulting_phase, p.seal
                ),
                None => println!("  word {} does not appear in the stream", word.id),
            }
        }
        for (from, to) in chain() {
            println!(
                "  {:11} -> {:11} | eta forward {:.4} | reverse {:.4} | asymmetry {:+.4} | alpha {:+.4}",
                from.name(),
                to.name(),
                bus.conductance[stochology_wave::spike::index(from)]
                    [stochology_wave::spike::index(to)],
                bus.conductance[stochology_wave::spike::index(to)]
                    [stochology_wave::spike::index(from)],
                bus.asymmetry(from, to),
                bus.alpha[stochology_wave::spike::index(from)][stochology_wave::spike::index(to)]
            );
        }
    }

    let scenario = Scenario {
        graph: &destructive,
        pulse: pulse(),
        bus: &bus,
        chain: chain(),
        steps: 3,
        dt: 0.65,
        min_asymmetry: 0.05,
    };

    if mode == "--demo" {
        println!("\n=== 5. GATE ===");
    }
    let report = verify(&scenario);
    for r in &report.results {
        println!(
            "  [{}] {} | {}",
            if r.passed { "OK" } else { "FAILED" },
            r.gate,
            r.detail
        );
    }

    let mut code = if report.passed() { 0 } else { 1 };

    match (mode, defect) {
        ("--defect", Some("mutation")) => {
            println!("\n[NEGATIVE PROOF] tampering 0.001 in the last state of the chain");
            let mut steps = destructive
                .propagate_audited(&pulse(), 3, 0.65)
                .expect("audited");
            if let Some(s) = steps.last_mut() {
                s.state[3].re += 0.001;
            }
            match verify_chain(&steps) {
                Ok(()) => {
                    println!("  the chain accepted the mutation: gate broken");
                    code = 1;
                }
                Err(e) => {
                    println!("  chain rejected: {e}");
                    code = 1;
                }
            }
        }
        ("--defect", Some("inversion")) => {
            println!("\n[NEGATIVE PROOF] inverting one causal link of the synaptic matrix");
            let mut corrupted = bus.clone();
            let a = stochology_wave::spike::index(Lens::Fluxonomy);
            let b = stochology_wave::spike::index(Lens::Chrononomy);
            corrupted.conductance[a][b] = 0.05;
            corrupted.conductance[b][a] = 0.80;
            let corrupted_scenario = Scenario {
                graph: &destructive,
                pulse: pulse(),
                bus: &corrupted,
                chain: chain(),
                steps: 3,
                dt: 0.65,
                min_asymmetry: 0.05,
            };
            let rel = verify(&corrupted_scenario);
            if rel.passed() {
                println!("  the gate accepted the anticausal inversion: gate broken");
            } else {
                for f in rel.failures() {
                    println!("  {}", f.detail);
                }
            }
            code = 1;
        }
        ("--defect", Some(other)) => {
            println!("unknown defect: {other} (use mutation or inversion)");
            code = 1;
        }
        ("--defect", None) => {
            println!("--defect requires a name: mutation or inversion");
            code = 1;
        }
        _ => {}
    }

    if mode == "--demo" {
        println!("\nreport seal: 0x{:016x}", report.seal);
    }

    if code != 0 {
        std::process::exit(code);
    }
}
