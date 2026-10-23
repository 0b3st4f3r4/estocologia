//! Single-Opening Temporal Vault (`TemporalVault`) with SHA-256 Seal (§4.1).
//!
//! Protects the out-of-time test set from adaptive overfitting:
//! 1. The vault is sealed with a cryptographic SHA-256 digest of its outcomes;
//! 2. It can be opened **at most once** (`open_once()`);
//! 3. Any second attempt to open the vault is rejected with [`VaultError::IllegalReopeningAttempt`];
//! 4. If the evaluated model fails the pre-registered Brier threshold, the project is automatically
//!    downgraded to [`VaultState::DowngradedToExploratory`].

use stochology_core::sha256_hex;
use stochology_lenses::decision::brier_score;

/// State of a [`TemporalVault`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultState {
    Sealed,
    OpenedApproved,
    DowngradedToExploratory,
}

/// Errors raised when interacting with a [`TemporalVault`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultError {
    /// Attempted to reopen a vault that has already been opened once.
    IllegalReopeningAttempt,
    /// Dimension mismatch between predictions and sealed vault outcomes.
    DimensionMismatch,
}

/// Result of the single permitted opening of a [`TemporalVault`].
#[derive(Debug, Clone, PartialEq)]
pub struct VaultOpeningReport {
    pub vault_id: String,
    pub sha256_seal: String,
    pub evaluated_brier_score: f64,
    pub max_allowed_brier: f64,
    pub final_state: VaultState,
}

/// Cryptographically sealed Single-Opening Temporal Vault (§4.1).
#[derive(Debug, Clone)]
pub struct TemporalVault {
    pub vault_id: String,
    pub sha256_seal: String,
    outcomes_y: Vec<f64>,
    state: VaultState,
}

impl TemporalVault {
    /// Seals a new out-of-time test set inside a `TemporalVault`.
    pub fn seal(vault_id: impl Into<String>, outcomes_y: Vec<f64>) -> Self {
        assert!(!outcomes_y.is_empty(), "vault outcomes cannot be empty");
        let id = vault_id.into();
        let payload = format!("{id}:{outcomes_y:?}");
        let seal = sha256_hex(payload.as_bytes());
        Self {
            vault_id: id,
            sha256_seal: seal,
            outcomes_y,
            state: VaultState::Sealed,
        }
    }

    #[inline]
    pub const fn state(&self) -> VaultState {
        self.state
    }

    /// Opens the vault **exactly once** to evaluate `forecasts_p` against `max_allowed_brier`.
    /// Any subsequent call returns `Err(VaultError::IllegalReopeningAttempt)`.
    pub fn open_once(
        &mut self,
        forecasts_p: &[f64],
        max_allowed_brier: f64,
    ) -> Result<VaultOpeningReport, VaultError> {
        if self.state != VaultState::Sealed {
            return Err(VaultError::IllegalReopeningAttempt);
        }
        if forecasts_p.len() != self.outcomes_y.len() {
            return Err(VaultError::DimensionMismatch);
        }

        let bs = brier_score(forecasts_p, &self.outcomes_y);
        self.state = if bs <= max_allowed_brier {
            VaultState::OpenedApproved
        } else {
            VaultState::DowngradedToExploratory
        };

        Ok(VaultOpeningReport {
            vault_id: self.vault_id.clone(),
            sha256_seal: self.sha256_seal.clone(),
            evaluated_brier_score: bs,
            max_allowed_brier,
            final_state: self.state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vault_allows_only_one_opening_and_downgrades_on_failure() {
        let mut v1 = TemporalVault::seal("vault_pass", vec![0.0, 1.0, 1.0, 0.0]);
        let r1 = v1
            .open_once(&[0.05, 0.95, 0.90, 0.10], 0.05)
            .expect("first opening must succeed");
        assert_eq!(r1.final_state, VaultState::OpenedApproved);
        assert_eq!(
            v1.open_once(&[0.05, 0.95, 0.90, 0.10], 0.05),
            Err(VaultError::IllegalReopeningAttempt)
        );

        let mut v2 = TemporalVault::seal("vault_fail", vec![0.0, 1.0]);
        let r2 = v2
            .open_once(&[0.90, 0.10], 0.05)
            .expect("first opening must return report");
        assert_eq!(r2.final_state, VaultState::DowngradedToExploratory);
    }
}

