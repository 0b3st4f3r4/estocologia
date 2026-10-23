//! Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$) and Synthetic Canary Column (§4.1).
//!
//! Prevents temporal leakage ("time travel"): every feature table must store both the physical
//! event time $t_{\text{ev}}$ and the immutable database availability timestamp $t_{\text{disp}} \ge t_{\text{ev}}$.
//! Queries evaluated at decision cut-off $\tau$ are strictly filtered by $t_{\text{disp}} \le \tau$.

/// Bitemporal feature record with a synthetic canary column whose value is a function of future outcome $Y$,
/// stamped with future availability $t_{\text{disp, canary}} > \tau$.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitemporalRecord {
    pub row_id: usize,
    pub t_event: u64,
    pub t_availability: u64,
    pub feature_x: u8,
    pub outcome_y: u8,
    pub canary_column: u8,
    pub t_availability_canary: u64,
}

/// Audit result of querying a bitemporal feature store at decision timestamp $\tau$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitemporalClockAudit {
    pub cutoff_tau: u64,
    pub bitemporal_filter_enabled: bool,
    pub rows_visible: usize,
    /// Empirical mutual information $I(\text{Canary}; Y)$ in nats among visible features.
    pub canary_mutual_information_nats: f64,
    /// Fires when $I(\text{Canary}; Y) > 0$ or any row with $t_{\text{disp}} > \tau$ leaks into the query.
    pub temporal_intrusion_alarm: bool,
}

/// Queries `records` at decision cut-off `cutoff_tau`.
/// When `enforce_bitemporal_filter` is `true`, only records with $t_{\text{disp}} \le \tau$ are admitted,
/// and any column whose $t_{\text{disp, col}} > \tau$ is masked to zero.
pub fn query_with_bitemporal_clock(
    records: &[BitemporalRecord],
    cutoff_tau: u64,
    enforce_bitemporal_filter: bool,
) -> BitemporalClockAudit {
    let mut visible_canary = Vec::new();
    let mut visible_y = Vec::new();
    let mut leaked_future_timestamp = false;

    for r in records {
        assert!(
            r.t_availability >= r.t_event,
            "physical law violation: t_availability cannot precede t_event"
        );
        if enforce_bitemporal_filter {
            if r.t_availability <= cutoff_tau {
                let canary_val = if r.t_availability_canary <= cutoff_tau {
                    r.canary_column
                } else {
                    0
                };
                visible_canary.push(canary_val);
                visible_y.push(r.outcome_y);
            }
        } else {
            // Mutated query filtering only by t_event <= tau (ignoring t_availability)
            if r.t_event <= cutoff_tau {
                if r.t_availability > cutoff_tau || r.t_availability_canary > cutoff_tau {
                    leaked_future_timestamp = true;
                }
                visible_canary.push(r.canary_column);
                visible_y.push(r.outcome_y);
            }
        }
    }

    let mi = binary_mutual_information_nats(&visible_canary, &visible_y);
    let alarm = leaked_future_timestamp || mi > 1e-9;

    BitemporalClockAudit {
        cutoff_tau,
        bitemporal_filter_enabled: enforce_bitemporal_filter,
        rows_visible: visible_y.len(),
        canary_mutual_information_nats: mi,
        temporal_intrusion_alarm: alarm,
    }
}

fn binary_mutual_information_nats(x: &[u8], y: &[u8]) -> f64 {
    if x.is_empty() || x.len() != y.len() {
        return 0.0;
    }
    let n = x.len() as f64;
    let mut counts = [[0.0; 2]; 2];
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let a = (xi != 0) as usize;
        let b = (yi != 0) as usize;
        counts[a][b] += 1.0;
    }
    let px = [
        (counts[0][0] + counts[0][1]) / n,
        (counts[1][0] + counts[1][1]) / n,
    ];
    let py = [
        (counts[0][0] + counts[1][0]) / n,
        (counts[0][1] + counts[1][1]) / n,
    ];
    let mut mi = 0.0;
    for a in 0..2 {
        for b in 0..2 {
            let pxy = counts[a][b] / n;
            if pxy > 0.0 && px[a] > 0.0 && py[b] > 0.0 {
                mi += pxy * (pxy / (px[a] * py[b])).ln();
            }
        }
    }
    mi.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitemporal_clock_zeros_canary_and_mutation_triggers_alarm() {
        let records = [
            BitemporalRecord {
                row_id: 1,
                t_event: 10,
                t_availability: 12,
                feature_x: 0,
                outcome_y: 0,
                canary_column: 0,
                t_availability_canary: 999,
            },
            BitemporalRecord {
                row_id: 2,
                t_event: 11,
                t_availability: 14,
                feature_x: 1,
                outcome_y: 1,
                canary_column: 1,
                t_availability_canary: 999,
            },
        ];

        let safe = query_with_bitemporal_clock(&records, 50, true);
        assert!(!safe.temporal_intrusion_alarm);
        assert_eq!(safe.canary_mutual_information_nats, 0.0);

        let mutated = query_with_bitemporal_clock(&records, 50, false);
        assert!(mutated.temporal_intrusion_alarm);
        assert!(mutated.canary_mutual_information_nats > 0.5);
    }
}
