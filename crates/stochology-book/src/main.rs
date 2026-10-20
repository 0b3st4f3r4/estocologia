//! Instrument of the book laboratories.
//!
//! Usage: ~cargo run --release --bin labs -- --chapter 4~
//!        ~cargo run --release --bin labs -- --all~

use stochology_book::{banco, chain, ensaio, escuta, fila, genetica, jev, ponte, segunda};
use stochology_book::{
    ch01, ch02, ch03, ch04, ch05, ch06, ch07, ch08, ch09, ch10, ch11, ch12, ch13, ch14, ch15, ch16,
    ch17, ch18, ch19, ch20, ch21,
};
use stochology_core::resolution::{R_HORIZON_DAYS, Resolution, measure as measure_loss};
use stochology_core::s1::{Category, S1};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut chapter = 0usize;
    let mut all = false;
    let mut resolution = false;
    let mut bank = false;
    let mut system1 = false;
    let mut system2 = false;
    let mut pretest_flag = false;
    let mut answers: Option<String> = None;
    let mut elicitation: Option<String> = None;
    let mut threshold: Option<f64> = None;
    let mut scale: Option<String> = None;
    let mut answer_key: Option<String> = None;
    let mut queue_flag = false;
    let mut genetics_flag = false;
    let mut streams_flag = false;
    let mut eprocess_flag = false;
    let mut mc_error_flag = false;
    let mut propagate_flag = false;
    let mut passk_flag = false;
    let mut oracles_flag = false;
    let mut multiverse_flag = false;
    let mut stacking_flag = false;
    let mut discrepancy_flag = false;
    let mut seed_spread_flag = false;
    let mut listening_flag = false;
    let mut chain_flag = false;
    let mut bridge_flag = false;
    let mut schema_flag = false;
    let mut second_opinion_flag = false;
    let mut jev_flag = false;
    let mut measure_flag = false;
    let mut rehearsal_shelf = false;
    let mut rehearsal_listening = false;
    let mut rehearsal_bridge = false;
    let mut help = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--chapter" => {
                if i + 1 < args.len() {
                    chapter = args[i + 1].parse().unwrap_or(0);
                    i += 1;
                }
            }
            "--all" => all = true,
            "--resolution" => resolution = true,
            "--bank" => bank = true,
            "--s1" => system1 = true,
            "--s2" => system2 = true,
            "--pretest" => pretest_flag = true,
            "--answers" => {
                if i + 1 < args.len() {
                    answers = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--scale" => {
                if i + 1 < args.len() {
                    scale = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--threshold" => {
                if i + 1 < args.len() {
                    threshold = args[i + 1].replace(',', ".").parse::<f64>().ok();
                    i += 1;
                }
            }
            "--elicitation" => {
                if i + 1 < args.len() {
                    elicitation = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--answer-key" => {
                if i + 1 < args.len() {
                    answer_key = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--queue" => queue_flag = true,
            "--genetics" => genetics_flag = true,
            "--streams" => streams_flag = true,
            "--eprocess" => eprocess_flag = true,
            "--mc-error" => mc_error_flag = true,
            "--propagate" => propagate_flag = true,
            "--passk" => passk_flag = true,
            "--oracles" => oracles_flag = true,
            "--multiverse" => multiverse_flag = true,
            "--stacking" => stacking_flag = true,
            "--discrepancy" => discrepancy_flag = true,
            "--seed-spread" => seed_spread_flag = true,
            "--listening" => listening_flag = true,
            "--h3" => chain_flag = true,
            "--bridge" => bridge_flag = true,
            "--schema" => schema_flag = true,
            "--second-opinion" => second_opinion_flag = true,
            "--jev" => jev_flag = true,
            "--measure" => measure_flag = true,
            "--rehearsal-shelf" => rehearsal_shelf = true,
            "--rehearsal-listening" => rehearsal_listening = true,
            "--rehearsal-bridge" => rehearsal_bridge = true,
            "--help" => help = true,
            other => {
                eprintln!("unknown flag: {other} (usage: labs --chapter N (1..21) or --all)");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    if help {
        println!(
            "usage: labs [--chapter N | --all | --bank | --queue | --resolution | --genetics | --streams | --mc-error | --propagate | --passk | --multiverse | --stacking | --discrepancy | --seed-spread"
        );
        println!(
            "            | --listening | --bridge | --schema | --second-opinion | --jev | --oracles"
        );
        println!("            | --h3 | --s1 | --s2 | --measure | --pretest | --elicitation Q5");
        println!("            | --rehearsal-shelf | --rehearsal-listening | --rehearsal-bridge]");
        return;
    }
    if resolution {
        r_horizon();
        return;
    }
    if bank {
        real_h0();
        return;
    }
    if system1 {
        run_system1();
        return;
    }
    if system2 {
        run_system2();
        return;
    }
    if pretest_flag {
        panel_pretest(answers.as_deref(), answer_key.as_deref());
        return;
    }
    if let Some(input) = elicitation.as_deref() {
        elicit(input, threshold, scale.as_deref());
        return;
    }
    if queue_flag {
        fila::run();
        return;
    }
    if genetics_flag {
        genetica::run();
        return;
    }
    if streams_flag {
        stochology_book::streams::run();
        return;
    }
    if seed_spread_flag {
        stochology_book::seed_spread::run();
        return;
    }
    if discrepancy_flag {
        stochology_book::discrepancy::run();
        return;
    }
    if stacking_flag {
        stochology_book::stacking::run();
        return;
    }
    if multiverse_flag {
        stochology_book::multiverse::run();
        return;
    }
    if oracles_flag {
        stochology_book::oracles::run();
        return;
    }
    if passk_flag {
        stochology_book::passk::run();
        return;
    }
    if propagate_flag {
        stochology_book::propagate::run();
        return;
    }
    if mc_error_flag {
        stochology_book::mc_error::run();
        return;
    }
    if eprocess_flag {
        stochology_book::eprocess::run();
        return;
    }
    if listening_flag {
        escuta::run(escuta::REGISTRY);
        return;
    }
    if chain_flag {
        chain::run(chain::REGISTRY, chain::LOOP_RESULTS, chain::LOOP_POSTHOC);
        return;
    }
    if bridge_flag {
        ponte::run(ponte::REGISTRY, ponte::JUDGE_NOTES);
        return;
    }
    if schema_flag {
        contract();
        return;
    }
    if second_opinion_flag {
        segunda::run();
        return;
    }
    if jev_flag && measure_flag {
        // The measure as a card: it is what the registry receives, without hand typing.
        print!("{}", jev::measure_card(&jev::measure()));
        return;
    } else if jev_flag {
        jev::run();
        return;
    }
    if rehearsal_shelf {
        match ensaio::shelf() {
            Ok(text) => print!("{text}"),
            Err(e) => {
                eprintln!("panel rehearsal did not run: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if rehearsal_listening {
        match ensaio::listening() {
            Ok(text) => print!("{text}"),
            Err(e) => {
                eprintln!("listening rehearsal did not run: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if rehearsal_bridge {
        match ensaio::bridge() {
            Ok(text) => print!("{text}"),
            Err(e) => {
                eprintln!("bridge rehearsal did not run: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if all {
        for n in 1usize..=21 {
            run(n);
        }
        return;
    }
    run(chapter);
}

/// System 1: deterministic triage of cards, with the closed vocabulary.
fn run_system1() {
    println!("== System 1 :: deterministic triage of cards ==");
    let s1 = S1::new();
    println!(
        "suffix tree: {} suffixes indexed over {} declared themes",
        s1.indexed_suffixes(),
        Category::ALL.len()
    );
    let samples = [
        "waiting time in the queue at the health station",
        "causal effect of the treatment on income",
        "heavy tail of a rare extreme event",
        "calibration of the platt score with brier",
        "fairness across subgroups and who pays for the error",
        "xyzzy plugh abracadabra",
    ];
    for text in samples {
        let t = s1.triage(text);
        println!(
            "{} | probability {:.4} | sum {:.4} | outside [0.01;0.99] {}",
            t.line(),
            t.probability,
            t.sum(),
            t.out_of_range()
        );
    }
}

/// SHELF elicitation: the fit of the prior by the quartiles and the controls of the protocol.
///
/// Usage: `labs --elicitation q05,q25,q50,q75,q95`
fn elicit(input: &str, threshold: Option<f64>, scale: Option<&str>) {
    use stochology_horizons::h3::shelf::{
        ANCHOR_TOLERANCE, EXPECTED_ANCHOR, MIN_WIDTH_RATIO, elicited_width_90, fit_normal,
        implied_width_90, overconfidence_ok, p_above, width_ratio_90,
    };

    let values: Vec<f64> = input
        .split(',')
        .filter_map(|t| t.trim().replace(',', ".").parse::<f64>().ok())
        .collect();
    if values.len() != 5 {
        println!(
            "five quantiles are needed in the order q05,q25,q50,q75,q95; {} came",
            values.len()
        );
        return;
    }
    let [q05, q25, q50, q75, q95] = [values[0], values[1], values[2], values[3], values[4]];
    println!("== SHELF elicitation :: fit by the quartiles and controls ==");
    if scale == Some("beta") {
        // frequency domain in (0, 1): the fit is Beta, and the protocol declares it
        use stochology_horizons::h3::beta::{beta_quantile, fit_beta};
        println!(
            "declared scale: Beta (frequency in (0, 1)) | elicited quantiles: q05 {q05} | q25 {q25} | q50 {q50} | q75 {q75} | q95 {q95}"
        );
        let p_beta = fit_beta(q25, q50, q75);
        println!(
            "Beta fitted to the three central quartiles: a = {:.6} | b = {:.6} | maximum error in the quartiles {:.3e}",
            p_beta.a, p_beta.b, p_beta.max_quartile_error
        );
        println!(
            "quantiles it implies: q05 {:.4} | q25 {:.4} | q50 {:.4} | q75 {:.4} | q95 {:.4}",
            beta_quantile(0.05, p_beta.a, p_beta.b),
            beta_quantile(0.25, p_beta.a, p_beta.b),
            beta_quantile(0.50, p_beta.a, p_beta.b),
            beta_quantile(0.75, p_beta.a, p_beta.b),
            beta_quantile(0.95, p_beta.a, p_beta.b)
        );
        let elicited = q95 - q05;
        let implied =
            beta_quantile(0.95, p_beta.a, p_beta.b) - beta_quantile(0.05, p_beta.a, p_beta.b);
        let ratio = if implied > 0.0 {
            elicited / implied
        } else {
            f64::NAN
        };
        println!(
            "90% width: elicited {:.4} | implied by the Beta itself {:.4} | ratio {:.4} (declared minimum {:.2}) -> {}",
            elicited,
            implied,
            ratio,
            MIN_WIDTH_RATIO,
            if overconfidence_ok(ratio) {
                "ok"
            } else {
                "OVERCONFIDENCE"
            }
        );
        let at_median =
            1.0 - stochology_horizons::h3::beta::incomplete_beta(p_beta.a, p_beta.b, q50);
        println!(
            "anchor: P(theta > median) by the Beta itself = {:.4} | expected {:.2} +- {:.2}",
            at_median, EXPECTED_ANCHOR, ANCHOR_TOLERANCE
        );
        if let Some(s) = threshold {
            println!(
                "decision threshold at {s}: P(theta > {s}) by the Beta = {:.4}",
                1.0 - stochology_horizons::h3::beta::incomplete_beta(p_beta.a, p_beta.b, s)
            );
        }
        return;
    }
    if scale == Some("log") {
        // binomial domain in odds ratio: the quantiles become logs before the fit
        let logs: Vec<f64> = values.iter().map(|v| v.ln()).collect();
        println!(
            "declared scale: natural log (odds ratio) | elicited quantiles: q05 {q05} | q25 {q25} | q50 {q50} | q75 {q75} | q95 {q95}"
        );
        println!(
            "in the log: q05 {:.6} | q25 {:.6} | q50 {:.6} | q75 {:.6} | q95 {:.6}",
            logs[0], logs[1], logs[2], logs[3], logs[4]
        );
        let p_log = fit_normal(logs[1], logs[2], logs[3]);
        println!(
            "Normal fitted in the log: mean {:.6} | sd {:.6} | maximum error in the quartiles {:.3e}",
            p_log.mean, p_log.sd, p_log.max_quartile_error
        );
        let elicited = elicited_width_90(logs[0], logs[4]);
        let implied = implied_width_90(p_log.sd);
        let ratio = width_ratio_90(elicited, p_log.sd);
        println!(
            "90% width in the log: elicited {:.4} | implied by the Normal itself {:.4} | ratio {:.4} (declared minimum {:.2}) -> {}",
            elicited,
            implied,
            ratio,
            MIN_WIDTH_RATIO,
            if overconfidence_ok(ratio) {
                "ok"
            } else {
                "OVERCONFIDENCE"
            }
        );
        println!(
            "in the log 0 is odds ratio 1 (equal treatment): P(OR > 1) in the prior itself = {:.4}",
            p_above(0.0, p_log.mean, p_log.sd)
        );
        if let Some(s) = threshold {
            println!(
                "decision threshold at {s} (odds ratio): P(OR > {s}) in the prior itself = {:.4}",
                p_above(s.ln(), p_log.mean, p_log.sd)
            );
        }
        let median = p_above(logs[2], p_log.mean, p_log.sd);
        println!(
            "anchor with S equal to the specialist's own median: P(OR > {q50}) = {:.4} (expected {:.2} +- {:.2})",
            median, EXPECTED_ANCHOR, ANCHOR_TOLERANCE
        );
        return;
    }
    println!("elicited quantiles: q05 {q05} | q25 {q25} | q50 {q50} | q75 {q75} | q95 {q95}");
    let prior = fit_normal(q25, q50, q75);
    println!(
        "Normal fitted by least squares on q25/q50/q75: mean {:.6} | sd {:.6} | maximum error in the quartiles {:.3e}",
        prior.mean, prior.sd, prior.max_quartile_error
    );
    let elicited = elicited_width_90(q05, q95);
    let implied = implied_width_90(prior.sd);
    let ratio = width_ratio_90(elicited, prior.sd);
    println!(
        "90% width: elicited {:.4} | implied by the Normal itself {:.4} | ratio {:.4} (declared minimum {:.2}) -> {}",
        elicited,
        implied,
        ratio,
        MIN_WIDTH_RATIO,
        if overconfidence_ok(ratio) {
            "ok"
        } else {
            "OVERCONFIDENCE"
        }
    );
    println!(
        "anchor with S equal to the specialist's own median: expected {:.2} +- {:.2} in P(theta > S)",
        EXPECTED_ANCHOR, ANCHOR_TOLERANCE
    );
    if let Some(s) = threshold {
        println!(
            "decision threshold at {s}: P(theta > {s}) in the prior itself = {:.4} | P(theta > S) with S at the median = {:.4}",
            p_above(s, prior.mean, prior.sd),
            p_above(prior.mean, prior.mean, prior.sd)
        );
    }
}

/// Calibration pre-test of the SHELF panel: the declared null of the protocol design.
fn panel_pretest(answers: Option<&str>, answer_key: Option<&str>) {
    use stochology_horizons::h3::pretest::{
        ADJUSTED_BUCKETS, ADJUSTED_MAX_ECE, PROTOCOL_BUCKETS, PROTOCOL_MAX_ECE, evaluate_adjusted,
        evaluate_with, invalid_probabilities, null_with_gate,
    };

    if let (Some(answers), Some(answer_key)) = (answers, answer_key) {
        let confidence: Vec<f64> = answers
            .split(',')
            .filter_map(|t| t.trim().parse::<f64>().ok())
            .collect();
        let hit: Vec<bool> = answer_key
            .split(',')
            .map(|t| {
                let t = t.trim();
                t.eq_ignore_ascii_case("yes") || t.eq_ignore_ascii_case("sim")
            })
            .collect();
        if confidence.len() != hit.len() || confidence.is_empty() {
            println!(
                "answers with {} values and answer key with {}: I do not score",
                confidence.len(),
                hit.len()
            );
            return;
        }
        let invalid = invalid_probabilities(&confidence);
        if !invalid.is_empty() {
            println!(
                "p outside (0, 1) in items {:?}: the protocol refuses exact 0 and 1",
                invalid
            );
        }
        println!("== SHELF pre-test :: scoring one specialist ==");
        println!("items: {} | distinct p: {}", confidence.len(), {
            let mut v = confidence.clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            v.dedup();
            v.len()
        });
        // declaration 3 of point 12: criterion 3 holds by the binomial test, and the gate in force is the adjusted one
        let adjusted = evaluate_adjusted(&confidence, &hit);
        println!(
            "Brier {:.6} | pool climatology {:.6} | skill {:.6} | REL {:.6} RES {:.6} UNC {:.6}",
            adjusted.brier,
            adjusted.brier_climate,
            adjusted.skill,
            adjusted.murphy.rel,
            adjusted.murphy.res,
            adjusted.murphy.unc
        );
        for bucket in &adjusted.buckets {
            println!(
                "  bucket {} | n {} | mean forecast {:.4} | observed rate {:.4} | deviation {:+.4}",
                bucket.index,
                bucket.n,
                bucket.forecast_mean,
                bucket.observed_rate,
                bucket.deviation()
            );
        }
        let tested_buckets =
            stochology_horizons::h3::pretest::buckets_by_binomial_test(&adjusted.buckets);
        for t in &tested_buckets {
            println!(
                "  bucket {} by the binomial test: {} hits of {} with forecast {:.4} | two-sided p {:.4} | Bonferroni p {:.4} -> {}",
                t.index,
                t.hits,
                t.n,
                t.forecast_mean,
                t.two_sided_p,
                t.p_with_bonferroni,
                if t.is_systematic(0.05) {
                    "SYSTEMATIC"
                } else {
                    "ok"
                }
            );
        }
        let by_written_threshold =
            evaluate_with(&confidence, &hit, ADJUSTED_BUCKETS, ADJUSTED_MAX_ECE);
        let by_fixed_threshold = by_written_threshold
            .rejections
            .iter()
            .filter(|x| {
                matches!(
                    x,
                    stochology_horizons::h3::pretest::Rejection::SystematicBucketByFixedThreshold { .. }
                )
            })
            .count();
        println!(
            "criterion 3, the two rulers: by the binomial test (in force, declaration 3) {} | by the fixed threshold of 0.25 (as written) {}",
            if tested_buckets.iter().any(|t| t.is_systematic(0.05)) {
                "rejected"
            } else {
                "ok"
            },
            if by_fixed_threshold > 0 {
                "rejected"
            } else {
                "ok"
            }
        );
        println!(
            "observed curve (the diagnostic that bites): {}",
            if stochology_horizons::h3::pretest::observed_curve_is_weakly_monotone(
                &adjusted.buckets
            ) {
                "weakly monotone"
            } else {
                "NOT monotone"
            }
        );
        for bucket in &adjusted.buckets {
            println!(
                "  item by item of bucket {}: see the specialist form",
                bucket.index
            );
        }
        println!(
            "adjusted gate, in force ({} buckets, ECE <= {}, criterion 3 by the binomial test): ECE {:.4} -> {}",
            ADJUSTED_BUCKETS,
            ADJUSTED_MAX_ECE,
            adjusted.ece,
            if adjusted.approved {
                "APPROVED"
            } else {
                "REJECTED"
            }
        );
        for r in &adjusted.rejections {
            println!("  rejection: {r:?}");
        }
        let written = evaluate_with(&confidence, &hit, PROTOCOL_BUCKETS, PROTOCOL_MAX_ECE);
        println!(
            "protocol gate as written ({} buckets, ECE <= {}): ECE {:.4} -> {}",
            PROTOCOL_BUCKETS,
            PROTOCOL_MAX_ECE,
            written.ece,
            if written.approved {
                "approved"
            } else {
                "rejected"
            }
        );
        return;
    }

    println!("== SHELF pre-test :: panel calibration ==");
    println!(
        "protocol design: 20 items, 10 buckets of equal mass; ECE threshold {}",
        PROTOCOL_MAX_ECE
    );
    println!(
        "measured limits of the design: criterion 3 (bucket with n >= 4) becomes vacuous, because 20 items in 10 buckets give 2 per bucket; criterion 4, as the protocol writes it, is inert: the buckets are cut by ordered p and the mean forecast of the upper one exceeds that of the lower one by construction"
    );
    for (items, buckets, threshold) in [
        (20usize, 5usize, ADJUSTED_MAX_ECE),
        (20, PROTOCOL_BUCKETS, PROTOCOL_MAX_ECE),
        (200, PROTOCOL_BUCKETS, PROTOCOL_MAX_ECE),
    ] {
        let null = null_with_gate(2026, 20_000, items, buckets, threshold);
        println!(
            "calibrated null, {items} items in {buckets} buckets, ECE threshold {threshold}: ECE p50 {:.4} | p90 {:.4} | p95 {:.4} | maximum {:.4} | skill p50 {:.4} | p10 {:.4} | Brier p50 {:.4}",
            null.ece_p50,
            null.ece_p90,
            null.ece_p95,
            null.max_ece,
            null.skill_p50,
            null.skill_p10,
            null.brier_p50
        );
        println!(
            "    verdict of the gate in the calibrated world: approved {:.4} | rejected by criterion 3 {:.4} | by corrected criterion 3 (binomial) {:.4} | by criterion 2 {:.4} | by some criterion {:.4}",
            null.fraction_passing,
            null.rejected_criterion_3,
            null.rejected_criterion_3_binomial,
            null.rejected_criterion_2,
            null.rejected_by_any
        );
    }
}

/// System 2: deliberation over the hypergraph, board belief, inference and the R axis.
fn run_system2() {
    use stochology_core::{Card, Layer, UncertaintyType};
    use stochology_core::{R_HORIZON_DAYS, measure_resolution_loss};
    use stochology_s2::{
        DECLARED_ORDER, HypergraphWorld, Lats, Metropolis, PanelBelief, action_destination, decide,
        evaluate_resolutions, greedy, learn, log_beta, sample_graph, stress_deliberative,
        stress_in_order,
    };

    println!("== System 2 :: deliberation over the hypergraph ==");
    let graph = sample_graph();
    let problems = graph.validate();
    println!(
        "declared graph: {} nodes, {} tree edges, {} graph edges, {} hyperedges | validation {}",
        graph.nodes.len(),
        graph.tree_edges.len(),
        graph.graph_edges.len(),
        graph.hyperedges.len(),
        if problems.is_empty() {
            "ok".to_string()
        } else {
            format!("{problems:?}")
        }
    );
    let world = HypergraphWorld::new(&graph);
    println!("nodes that no lens covers: {:?}", world.uncovered_nodes());
    let lats = Lats {
        depth: 2,
        budget: 40,
        exploration: 1.0,
        seed: 2026,
    };
    let greedy_decision = greedy(&world, "root.stochology").expect("the graph has exits");
    let search = world
        .search("root.stochology", &lats, 3)
        .expect("the search arrives");
    println!(
        "greedy: stops at {} (value {:.3}) | LATS: {} (value {:.3}, node without lens {})",
        action_destination(&greedy_decision.action),
        greedy_decision.value,
        search.path.join(" -> "),
        search.value,
        search.lens_free
    );

    let complete_card = |id: &str| {
        Card::new(id, Layer::Agent, UncertaintyType::Epistemic)
            .with_safeguards()
            .complete()
    };
    let mut panel = PanelBelief::new();
    let training: Vec<Card> = (0..4)
        .map(|i| {
            let mut f = complete_card(&format!("s2.treino.{i}"));
            f.trail_intact = false;
            f
        })
        .collect();
    learn(&mut panel, &training, 3.0);
    let top = panel.order()[0];
    println!(
        "board belief: most likely band {} with probability {:.4} | entropy {:.4}",
        top.name(),
        panel.probability(top),
        panel.entropy()
    );
    let mut target = complete_card("s2.alvo");
    target.trail_intact = false;
    let deliberated = stress_deliberative(&target, &panel);
    let declared = stress_in_order(&target, &DECLARED_ORDER);
    println!(
        "stress: deliberated stopped at {} with {} attack(s) | declared order stopped at {} with {} attacks | same rejection {}",
        deliberated.agent.name(),
        deliberated.attacks,
        declared.agent.name(),
        declared.attacks,
        deliberated.failed == declared.failed
    );

    let chain = Metropolis {
        step: 0.15,
        seed: 7,
    }
    .sample(log_beta(4.0, 8.0), 0.5, 200_000);
    println!(
        "inference: Beta(4, 8) chain mean {:.4} against the closed form {:.4} | acceptance {:.3}",
        chain.mean(20_000),
        1.0 / 3.0,
        chain.acceptance_rate()
    );

    let loss = measure_resolution_loss(1909, R_HORIZON_DAYS, 10, 5.0, 11.0);
    let decision = decide(&loss, 12.0);
    let out: Vec<String> = evaluate_resolutions(&loss, 12.0)
        .iter()
        .filter(|a| a.out_of_reach.is_some())
        .map(|a| a.resolution.code().to_string())
        .collect();
    println!(
        "R axis: loss of the aggregate {:.2} min against a margin of 12.00 min -> {} ({}) | out of measured reach: {}",
        loss.loss,
        decision.chosen.code(),
        decision.reason,
        out.join(", ")
    );
    let without_oscillation = measure_resolution_loss(1909, R_HORIZON_DAYS, 10, 8.0, 8.0);
    println!(
        "R axis without oscillation: loss {:.2} min -> {}",
        without_oscillation.loss,
        decide(&without_oscillation, 12.0).chosen.code()
    );
}

/// Real H0 in Rust: the bank call centre.
fn real_h0() {
    println!("== Real H0 :: the bank call centre, in Rust ==");
    let path = match std::env::var("ESTOCOLOGIA_DADOS") {
        Ok(base) => format!("{}/data/bank_calls.csv", base.trim_end_matches('/')),
        Err(_) => "data/bank_calls.csv".to_string(),
    };
    let m = banco::measure(&path, 1909, 400);
    println!(
        "training {} days (Mar-Jun) | vault {} days (Jul-Oct) | {:.0} calls in the vault",
        m.train_days, m.vault_days, m.vault_calls
    );
    println!(
        "roster of the mean profile: {} agents at the peak, {:.0} agent-hours per day",
        m.agents_at_peak, m.agents_per_day
    );
    println!(
        "service level on the mean profile = {:.4} | mean of the vault days = {:.4} | days below the goal of {:.0}% = {:.4}",
        m.level_on_mean_profile,
        m.mean_level_of_days,
        banco::GOAL * 100.0,
        m.fraction_days_below_goal
    );
    println!(
        "dispersion between days: Var/mean = {:.1} in the vault, {:.1} in the training window, {:.1} over all | deviation between days = {:.0} | Poisson would predict {:.0}",
        m.var_over_mean,
        m.var_over_mean_train,
        m.var_over_mean_all,
        m.sd_between_days,
        m.poisson_sd
    );
    println!(
        "halves null: per day = {:.4} | per call = {:.4}",
        m.halves_null_day, m.halves_null_call
    );
}

/// R horizon: what it costs to ask coarsely, with the same total resource.
fn r_horizon() {
    println!("== R horizon :: resolution of the question ==");
    println!("paired arms: same arrivals and same service bases; only the service rate changes");
    for (label, low, high) in [
        (
            "demand 5 or 11 (the station of Chapter 12)",
            5.0_f64,
            11.0_f64,
        ),
        ("demand 7 or 9 (light oscillation)", 7.0, 9.0),
        ("demand 8 or 8 (no oscillation)", 8.0, 8.0),
    ] {
        let p = measure_loss(1909, R_HORIZON_DAYS, 10, low, high);
        println!(
            "{}: mean capacity {:.1} | aggregate wait {:.2} min, fine wait {:.2} min, loss {:.2} min",
            label, p.aggregated_capacity, p.aggregated_wait, p.fine_wait, p.loss
        );
        println!(
            "   above 30 min: aggregated {:.4}, fine {:.4}",
            p.above30_aggregated, p.above30_fine
        );
    }
    let fineness: Vec<&str> = Resolution::ALL.iter().map(|r| r.code()).collect();
    println!("resolution ladder: {}", fineness.join(" -> "));

    // ---- prediction 3: the oscillation threshold
    println!(
        "\n-- prediction 3: the oscillation threshold (failed day: more than 10% above 30 min; threshold: more than 1/4 of the days) --"
    );
    let amplitudes = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
    let calibrated = stochology_core::resolution::calibrated_scale(1909, 200, 10, 8.0, 0.10);
    let erlang =
        stochology_core::resolution::oscillation_threshold(1909, 200, 10, 8.0, &amplitudes);
    println!(
        "  {} days per amplitude | 10 h per day | specification 1: the Erlang plan of the average day, {} agents",
        erlang.points[0].days, erlang.points[0].agents_r0
    );
    println!(
        "  {:>10} {:>12} {:>14} {:>12} {:>10}",
        "amplitude", "failed days", "fraction", "loss (min)", "above30 R0"
    );
    for p in &erlang.points {
        println!(
            "  {:>10.1} {:>12} {:>14.3} {:>12.2} {:>10.4}",
            p.amplitude, p.failed_days, p.failed_day_fraction, p.loss, p.above30_r0
        );
    }
    match erlang.threshold {
        Some(a) => println!(
            "  threshold: amplitude {:.1} | resolution loss at the threshold {:.2} min | monotone: {}",
            a,
            erlang.loss_at_threshold.unwrap_or(f64::NAN),
            erlang.monotone
        ),
        None => println!(
            "  threshold: NOT FOUND on the grid (prediction 3 does not hold here) | monotone: {}",
            erlang.monotone
        ),
    }

    println!("\n-- prediction 3, specification 2: the R0 scale calibrated on flat demand --");
    let l = stochology_core::resolution::oscillation_threshold_with_plan(
        1909,
        200,
        10,
        8.0,
        &amplitudes,
        calibrated,
    );
    println!(
        "  the calibrated scale has {} agents (the flat one fails on at most 10% of the days)",
        calibrated
    );
    println!(
        "  {:>10} {:>12} {:>14} {:>12} {:>10}",
        "amplitude", "failed days", "fraction", "loss (min)", "above30 R0"
    );
    for p in &l.points {
        println!(
            "  {:>10.1} {:>12} {:>14.3} {:>12.2} {:>10.4}",
            p.amplitude, p.failed_days, p.failed_day_fraction, p.loss, p.above30_r0
        );
    }
    match l.threshold {
        Some(a) => println!(
            "  threshold: amplitude {:.1} | resolution loss at the threshold {:.2} min | monotone: {}",
            a,
            l.loss_at_threshold.unwrap_or(f64::NAN),
            l.monotone
        ),
        None => println!(
            "  threshold: NOT FOUND on the grid (prediction 3 does not hold here) | monotone: {}",
            l.monotone
        ),
    }

    // ---- prediction 5: the transfer
    println!(
        "\n-- prediction 5: the transfer (surviving R1 implies surviving R0; the converse does not) --"
    );
    let profiles = [
        (8.0, 0.5),
        (8.0, 1.0),
        (8.0, 1.5),
        (8.0, 2.0),
        (8.0, 2.5),
        (8.0, 3.0),
    ];
    let t = stochology_core::resolution::transfer(1909, 200, 10, &profiles);
    println!(
        "  {} profiles x 200 days | day threshold {:.2} | violations of the implication: {} | counterexamples: {} | aggregate conservation checked: {}",
        t.profiles, t.threshold, t.violations, t.counterexamples, t.conservation_checked
    );
    println!(
        "  reading: no profile survives R1 and fails at R0; {} profiles pass at R0 and fail on some day",
        t.counterexamples
    );
    let l_threshold = l.threshold;

    // ---- the R axis, chapter by chapter
    println!(
        "\n-- the R axis, chapter by chapter (the table is in book/r-axis.tsv; the reading is computed here) --"
    );
    let profiles: [(&str, f64, f64, &str); 3] = [
        ("posto", 5.0, 11.0, "demand from 5 to 11"),
        ("posto-leve", 7.0, 9.0, "demand from 7 to 9"),
        ("plano", 8.0, 8.0, "flat demand"),
    ];
    let table = std::fs::read_to_string("book/r-axis.tsv").unwrap_or_default();
    // The table is frozen and sealed (empirical/p18-eixo-r) in the chapter numbering of 2026-10-16; the chapters were
    // reordered on 2026-10-19, so each row is printed under its current number, in current order.
    let mut rows: Vec<(usize, String)> = Vec::new();
    for line in table.lines() {
        let l = line.trim_end();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = l.split('\t').collect();
        if fields.len() < 3 || fields[0] == "cap" {
            continue;
        }
        let (frozen, case, sentence) = (fields[0], fields[1], fields[2]);
        let cap = current_chapter(frozen.parse().unwrap_or(0));
        let reading = match profiles.iter().find(|(name, _, _, _)| *name == case) {
            Some((_, low, high, label)) => {
                let p = measure_loss(1909, R_HORIZON_DAYS, 10, *low, *high);
                format!("resolution loss {:.2} min ({})", p.loss, label)
            }
            None => match case {
                "previsoes" => match l_threshold {
                    Some(amplitude) => {
                        format!(
                            "oscillation threshold at amplitude {amplitude:.1} (calibrated plan)"
                        )
                    }
                    None => "oscillation threshold not found".to_string(),
                },
                _ => "no loss measured in this chapter".to_string(),
            },
        };
        rows.push((cap, format!("chapter {cap:>2} | {reading} | {sentence}")));
    }
    rows.sort_by_key(|(cap, _)| *cap);
    for (_, row) in rows {
        println!("{row}");
    }
}

/// The chapter number of today for a number frozen before the reordering of 2026-10-19 (Appendix E).
fn current_chapter(frozen: usize) -> usize {
    match frozen {
        12 => 15,
        13 => 16,
        14 => 17,
        15 => 18,
        16 => 12,
        17 => 13,
        18 => 14,
        n => n,
    }
}

fn contract() {
    println!("== The contract :: the card on file, the version and the migration chain ==");
    println!(
        "card version: {} | chain declared in crates/stochology-core/migracoes.tsv",
        stochology_core::schema::SCHEMA_VERSION.text()
    );
    for (i, step) in stochology_core::schema::DECLARED_CHAIN
        .description()
        .iter()
        .enumerate()
    {
        println!("  step {}: {}", i + 1, step);
    }
    println!(
        "  versions of the chain: {}",
        stochology_core::schema::DECLARED_CHAIN
            .versions()
            .iter()
            .map(|v| v.text())
            .collect::<Vec<_>>()
            .join(" -> ")
    );

    println!("\n-- demonstration: a version 1.0 document migrates up to the card version --");
    let mut card = stochology_core::Card::new(
        "lab.schema.1",
        stochology_core::Layer::Model,
        stochology_core::UncertaintyType::Epistemic,
    )
    .with_resolution(stochology_core::Resolution::R3Subquestion)
    .complete()
    .with_safeguards();
    card.definition = "contract demonstration card".into();
    card.decision_maker_question = "each day, does the service hold?".into();
    card.package_action = "size the roster of the day".into();

    let current = stochology_core::schema::text(&card);
    let mut old = String::from("versao = 1.0\n");
    for line in current.lines().skip(1) {
        if line.starts_with("resolucao ") || line.starts_with("priori_elicitada ") {
            continue;
        }
        old.push_str(line);
        old.push('\n');
    }
    println!("  the 1.0 document does not declare the two fields the chain adds");
    match stochology_core::schema::from_text(&old) {
        Ok(migrated) => {
            println!(
                "  migrated: resolution = {} (the declared addition value) | elicited_prior = {} | definition intact = {}",
                migrated.resolution.code(),
                migrated.elicited_prior,
                migrated.definition == card.definition
            );
            println!(
                "  the migrated card validated: id {} | layer {} | action \"{}\"",
                migrated.id,
                migrated.layer.code(),
                migrated.package_action
            );
        }
        Err(error) => println!("  REJECTED: {error:?}"),
    }
    println!(
        "  the card at the current version does not go through the chain: {}",
        stochology_core::schema::from_text(&current).is_ok()
    );

    println!(
        "\n-- the two fields of contract 1.3: the uncertainty budget and the dominant event --"
    );
    // The budget of the health post of Chapter 5 with no correlation between the rates (labs --propagate): the
    // arrivals carry 42% of the delta variance and the pace 58%. The dominant event of Bernoulli's ship in
    // Chapter 11: the loss of the ship, with probability 5%, is the only event that costs, so it carries all of it.
    let declared = card.clone().with_budget(
        "arrivals=0.42; pace=0.58; covariance=0.00",
        "the ship is lost; p=0.05; share=1.0",
    );
    match stochology_core::schema::from_text(&stochology_core::schema::text(&declared)) {
        Ok(read) => println!(
            "  declared and read back: budget \"{}\" | dominant event \"{}\"",
            read.uncertainty_budget.unwrap_or_default(),
            read.dominant_event.unwrap_or_default()
        ),
        Err(error) => println!("  REJECTED: {error}"),
    }
    let open = card
        .clone()
        .with_budget("arrivals=0.42", "the ship is lost; p=0.05; share=1.0");
    match stochology_core::schema::from_text(&stochology_core::schema::text(&open)) {
        Ok(_) => println!("  a budget that does not close: ACCEPTED (it should not be)"),
        Err(error) => println!("  a budget that does not close is refused: {error}"),
    }
    println!("\n-- the two fields of contract 1.4: the multiverse and the reliability --");
    // Contract 1.4. The multiverse of the health post (labs --multiverse): 18 specifications, 10 keep the promise as
    // the card of Chapter 5 does, and the variance between them by choice. The reliability of an agent in world B of
    // the laboratory of Chapter 16 (labs --passk): pass^1 0.5312 and pass^8 0.0700 in its first run.
    let multiverse = "specifications=18; agree=10; shares=service-cv:0.93, rate-law:0.00, correlation:0.06, interaction:0.02";
    let reliability = "k=8; pass1=0.5312; passk=0.0700";
    let declared = card
        .clone()
        .with_multiverse(multiverse)
        .with_reliability(reliability);
    match stochology_core::schema::from_text(&stochology_core::schema::text(&declared)) {
        Ok(read) => println!(
            "  declared and read back: multiverse \"{}\" | reliability \"{}\"",
            read.multiverse.unwrap_or_default(),
            read.reliability.unwrap_or_default()
        ),
        Err(error) => println!("  REJECTED: {error}"),
    }
    let unmeasured = card.clone().with_budget(
        "inputs=0.40; specification=0.60",
        "the ship is lost; p=0.05; share=1.0",
    );
    match stochology_core::schema::from_text(&stochology_core::schema::text(&unmeasured)) {
        Ok(_) => println!(
            "  a share for the specification without a multiverse: ACCEPTED (it should not be)"
        ),
        Err(error) => {
            println!("  a share for the specification without a multiverse is refused: {error}")
        }
    }
    let promising = card
        .clone()
        .with_reliability("k=8; pass1=0.5312; passk=0.6");
    match stochology_core::schema::from_text(&stochology_core::schema::text(&promising)) {
        Ok(_) => println!("  a pass^k above pass^1: ACCEPTED (it should not be)"),
        Err(error) => println!("  a pass^k above pass^1 is refused: {error}"),
    }
    println!("\n-- the three fields of contract 1.5: what happens between the specialists --");
    // The ship of Chapter 11: the decision specialist writes the rule, and p = 0.05 against C/L = 0.0529 implies not
    // to insure; the inference specialist writes the null model and logs two findings, one used.
    let conduct = card.clone().with_conduct(
        "modelo_nulo:inference, regra_de_decisao:decision",
        "logged=2; used=1; unused=a second break in the series: outside the window of the question",
        "p=0.05; C=1800; L=34000; act=do not protect",
    );
    match stochology_core::schema::from_text(&stochology_core::schema::text(&conduct)) {
        Ok(read) => println!(
            "  declared and read back: authorship \"{}\" | findings \"{}\" | decision rule \"{}\"",
            read.authorship.unwrap_or_default(),
            read.findings.unwrap_or_default(),
            read.decision_rule.unwrap_or_default()
        ),
        Err(error) => println!("  REJECTED: {error}"),
    }
    for (what, card) in [
        (
            "a field written outside its role",
            card.clone().with_conduct(
                "regra_de_decisao:inference",
                "logged=0; used=0",
                "p=0.05; C=1800; L=34000; act=do not protect",
            ),
        ),
        (
            "a finding withheld",
            card.clone().with_conduct(
                "regra_de_decisao:decision",
                "logged=3; used=1; unused=a second break in the series: outside the window of the question",
                "p=0.05; C=1800; L=34000; act=do not protect",
            ),
        ),
        (
            "an action that departs from its reasoning",
            card.clone().with_conduct(
                "regra_de_decisao:decision",
                "logged=0; used=0",
                "p=0.05; C=1800; L=34000; act=protect",
            ),
        ),
    ] {
        match stochology_core::schema::from_text(&stochology_core::schema::text(&card)) {
            Ok(_) => println!("  {what}: ACCEPTED (it should not be)"),
            Err(error) => println!("  {what} is refused: {error}"),
        }
    }
}

fn run(cap: usize) {
    match cap {
        1 => ch01(),
        2 => ch02(),
        3 => ch03(),
        4 => ch04(),
        5 => ch05::run(),
        6 => ch06::run(),
        7 => ch07::run(),
        8 => ch08::run(),
        9 => ch09::run(),
        10 => ch10::run(),
        11 => ch11::run(),
        12 => ch12::run(),
        13 => ch13::run(),
        14 => ch14::run(),
        15 => ch15::run(),
        16 => ch16::run(),
        17 => ch17::run(),
        18 => ch18::run(),
        19 => ch19::run(),
        20 => ch20::run(),
        21 => ch21::run(),
        _ => {
            eprintln!("usage: labs --chapter N (1..21) or --all");
            std::process::exit(1);
        }
    }
}

fn ch01() {
    println!("== Chapter 1 :: the invisible target ==");
    let n = 10_000;
    println!(
        "two machines with the seed 1713 agree in {} of {} draws: {}",
        n,
        n,
        ch01::two_machines_agree(1713, n)
    );
    println!(
        "draws coinciding with the next seed: {}",
        ch01::matches_with_another_seed(1713, n)
    );
    for seed in [1713u64, 1909, 2026] {
        println!(
            "seed {}: frequency of heads in 100 thousand tosses = {:.4}",
            seed,
            ch01::frequency_of_heads(seed, 100_000)
        );
    }
}

fn ch02() {
    println!("== Chapter 2 :: the fallacy lives on the bridge ==");
    let false_positive = 1.0 / 1_000_000.0;
    for prevalence in [0.01_f64, 1.0 / 1_000.0, 1.0 / 1_000_000.0] {
        let p = ch02::prosecutor_fallacy(prevalence, 1.0, false_positive);
        println!(
            "prevalence {:.0e}: P(evidence|innocent)={:.8}  P(innocent|evidence)={:.4}  swap factor={:.1}",
            prevalence,
            p.likelihood,
            p.posterior_of_innocence,
            ch02::swap_factor(prevalence, 1.0, false_positive)
        );
    }

    // The gambler's fallacy: a streak says something about an urn and nothing about a coin.
    let coin = ch02::coin_after_streak(2026, 1_000_000, 5);
    println!(
        "coin, 1000000 sequences of 6 tosses: after 5 heads in a row ({} cases) the next toss is heads in {:.1}%; betting on tails at even odds gains {:+.3} per bet",
        coin.cases,
        100.0 * coin.repeats,
        2.0 * (1.0 - coin.repeats) - 1.0
    );
    let urn = ch02::urn_after_streak(2026, 1_000_000, 5, 10, 10);
    println!(
        "urn of 10 red and 10 blue balls, without replacement: after 5 reds in a row ({} cases) the next ball is red in {:.1}% (exact {:.1}%); betting on blue at even odds gains {:+.3} per bet",
        urn.cases,
        100.0 * urn.repeats,
        100.0 * ch02::urn_exact(10, 10, 5),
        2.0 * (1.0 - urn.repeats) - 1.0
    );
}

fn ch03() {
    println!("== Chapter 3 :: trees, odds and decibans ==");
    let a = ch03::tree(10_000, 0.01, 0.90, 0.09);
    println!(
        "mammography: {} sick, {} sick positives, {} healthy, {} false positives, {} positives",
        a.sick, a.true_positives, a.healthy, a.false_positives, a.positives
    );
    println!(
        "P(sick|positive) = {:.4}  (one in every {:.1} positives)",
        a.fraction_sick_among_positives(),
        a.one_in_every()
    );
    println!(
        "second test: independent = {:.1} dB, correlated (effective ratio 4.2) = {:.2} dB",
        ch03::decibans(10.0),
        ch03::decibans(4.2)
    );
    println!(
        "posterior with two positives: independent = {:.4}, correlated = {:.4}",
        ch03::posterior_by_odds(0.01, &[10.0, 10.0]),
        ch03::posterior_by_odds(0.01, &[10.0, 4.2])
    );
    // The book and the world: coherence removes the first and leaves the second.
    let (rain, dry, q) = (0.60, 0.50, 0.25);
    let (cr, cd) = ch03::make_coherent(rain, dry);
    println!(
        "Dutch book: prices {rain:.2} + {dry:.2} -> certain profit of the bookmaker = {:.4}",
        ch03::dutch_book_profit(rain, dry)
    );
    println!(
        "made coherent: {cr:.4} + {cd:.4} -> certain profit = {:.4}",
        ch03::dutch_book_profit(cr, cd)
    );
    println!(
        "against a world that rains {q:.2} of the time: edge of who knows it = {:.4} with the original rain price, {:.4} after coherence, {:.4} if calibrated",
        ch03::edge_over_the_agent(rain, q),
        ch03::edge_over_the_agent(cr, q),
        ch03::edge_over_the_agent(q, q)
    );
}

fn ch04() {
    println!("== Chapter 4 :: the coin whose bias changes ==");
    let c = ch04::moving_coin(2026, 6_400, 200, 0.99);
    println!(
        "bias climbs from {:.1} to {:.1} in {} steps, {} repetitions, discount {}",
        ch04::bias_at(0, c.steps),
        ch04::bias_at(c.steps, c.steps),
        c.steps,
        c.repetitions,
        c.discount
    );
    println!(
        "coverage of the 95% interval: fixed model = {:.4}, model with forgetting = {:.4}",
        c.static_coverage, c.discounted_coverage
    );
    println!(
        "width of the fixed interval at step 100 = {:.4}, at step {} = {:.4}",
        c.static_width_100, c.steps, c.static_width_final
    );
    println!(
        "width of the interval with forgetting at step {} = {:.4} (ratio {:.1}x the final fixed one)",
        c.steps,
        c.discounted_width_final,
        c.discounted_width_final / c.static_width_final
    );

    // The law of total variance and the simulated respondent.
    let exact = ch04::variance_split(&ch04::SURVEY_SHARES, &ch04::SURVEY_YES);
    let person = ch04::simulated_split(2026, 200_000, ch04::Respondent::Person);
    let simulated = ch04::simulated_split(2026, 200_000, ch04::Respondent::MostCommon);
    println!(
        "survey of 5 groups, closed form: total variance {:.4} = inside the groups {:.4} + between the groups {:.4}; yes rate {:.4}",
        exact.total, exact.within, exact.between, exact.yes_rate
    );
    println!(
        "200000 people drawn from the groups: inside {:.4} ({:.1}% of the total), yes rate {:.4}",
        person.within,
        100.0 * person.within / person.total,
        person.yes_rate
    );
    println!(
        "200000 simulated respondents answering the most common answer of their group: inside {:.4} ({:.1}% of the total), yes rate {:.4}",
        simulated.within,
        100.0 * simulated.within / simulated.total,
        simulated.yes_rate
    );
}
