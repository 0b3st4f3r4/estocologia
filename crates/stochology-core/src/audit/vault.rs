//! Vault outside of time: declared split of the series into **train** and **vault**, with a single
//! opening.
//!
//! Declared in Chapters 18 and 20 ("the vault outside of time", "the vault is the password") and in
//! the inventory of proposals not yet demanded: in Rust the chained ledger and the Merkle root
//! existed, and the vault was described and not a component. This module closes the gap with the
//! hard rule of the project:
//!
//! > the future partition stays under the exclusive custody of the lifecycle agent; no modelling
//! > agent reads it before the pre-registration; it is opened **a single time**.
//!
//! **The rule this module proves:** erring after the opening requires a **new vault**, never an
//! adjustment.
//!
//! * The accepted opening enters the ledger with a hash, and the content of the event binds the
//!   pre-registration (id, cut, hash of the specification, n of items, agent): changing any of these
//!   fields outside the ledger makes `verify()` fail.
//! * The second opening returns an error **and is recorded** (`REFUSAL_CARD`), because the
//!   temptation to look twice is also part of the record: hiding the attempt would be rewriting the
//!   trail.
//! * `correct()` after the opening is refused in the same way, with
//!   `VaultError::NewVaultNeeded`. The only legitimate fix is `successor()`: a new id and a
//!   new pre-registration. That is what H3 did three times (2023 flights, Toronto prisons
//!   2020-2021, Rhode Island 2013-2015).
//!
//! Determinism: no clock, no network, no random seed; the temporal cut is an integer stamp
//! supplied by the caller.

use super::{Ledger, sha256_hex};

/// Card of the accepted opening event.
pub const OPENING_CARD: &str = "cofre.abertura";
/// Card of the event that records the refusal (second opening, late correction).
pub const REFUSAL_CARD: &str = "cofre.recusa";

/// The rule of the project, in one line, for whoever reads the code before the test.
pub const VAULT_RULE: &str = "vault opened a single time; after the opening the fix is a new vault, never an adjustment on the same data";

/// Where the cut between train and vault falls, declared before opening.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cut {
    /// Cut by position: train = [0, cut), vault = [cut, n).
    Index(usize),
    /// Temporal cut: train = t < cut, vault = t >= cut (t_disp, as in H3).
    Time(i64),
}

impl Cut {
    pub fn label(&self) -> String {
        match self {
            Cut::Index(i) => format!("index:{}", i),
            Cut::Time(t) => format!("time:{}", t),
        }
    }
}

/// What blocks the split or the opening.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultError {
    /// Series without a single unit.
    EmptySeries,
    /// One of the sides would come out empty: there is no train without history and no vault without future.
    DegenerateCut { cut: Cut, n: usize },
    /// Temporal series out of order: the cut by time would stop being a cut.
    UnorderedSeries {
        position: usize,
        previous: i64,
        current: i64,
    },
    /// Vault without content under custody.
    EmptyVault,
    /// Opening without an identified agent: the custody belongs to someone.
    AgentNotDeclared,
    /// Correction requested before opening.
    NotOpened,
    /// Second opening of the same vault: refused and recorded.
    AlreadyOpened { opening: u64, hash: String },
    /// Correction after the opening: it requires a new vault.
    NewVaultNeeded {
        vault: String,
        opening: u64,
        hash: String,
    },
    /// Successor with the same id as the vault already opened.
    VaultReused { id: String },
    /// Successor with the same pre-registration: it would be reopening the same data.
    RepeatedPreRegistration { vault: String },
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::EmptySeries => write!(f, "empty series: there is nothing to split"),
            VaultError::DegenerateCut { cut, n } => {
                write!(f, "degenerate cut {} over a series of {}", cut.label(), n)
            }
            VaultError::UnorderedSeries {
                position,
                previous,
                current,
            } => write!(
                f,
                "series out of order at position {}: {} after {}",
                position, current, previous
            ),
            VaultError::EmptyVault => write!(f, "empty vault: nothing under custody"),
            VaultError::AgentNotDeclared => write!(f, "opening without a declared agent"),
            VaultError::NotOpened => write!(f, "vault still closed: there is nothing to correct"),
            VaultError::AlreadyOpened { opening, hash } => write!(
                f,
                "second opening refused: the first one was event {} (hash {})",
                opening, hash
            ),
            VaultError::NewVaultNeeded {
                vault,
                opening,
                hash,
            } => write!(
                f,
                "correction after opening {} of vault {} (hash {}): requires a new vault",
                opening, vault, hash
            ),
            VaultError::VaultReused { id } => {
                write!(
                    f,
                    "vault {} was already opened: the successor needs a new id",
                    id
                )
            }
            VaultError::RepeatedPreRegistration { vault } => write!(
                f,
                "vault {} repeats the pre-registration of the predecessor: it would be the same data",
                vault
            ),
        }
    }
}

impl std::error::Error for VaultError {}

/// Series cut into two sides that add up to the whole series and do not share a unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partition<T> {
    pub train: Vec<T>,
    pub vault: Vec<T>,
    pub cut: Cut,
    pub n: usize,
}

impl<T> Partition<T> {
    pub fn n_train(&self) -> usize {
        self.train.len()
    }

    pub fn n_vault(&self) -> usize {
        self.vault.len()
    }

    /// The two sides add up to the whole series, with no unit lost.
    pub fn is_whole(&self) -> bool {
        self.train.len() + self.vault.len() == self.n
    }
}

impl<T: PartialEq> Partition<T> {
    /// Some unit appears on both sides.
    pub fn overlaps(&self) -> bool {
        self.train.iter().any(|t| self.vault.contains(t))
    }

    /// The declared split: whole and without overlap.
    pub fn is_clean(&self) -> bool {
        self.is_whole() && !self.overlaps()
    }
}

/// Splits by position: train = [0, cut), vault = [cut, n).
pub fn split_by_index<T: Clone>(series: &[T], cut: usize) -> Result<Partition<T>, VaultError> {
    if series.is_empty() {
        return Err(VaultError::EmptySeries);
    }
    if cut == 0 || cut >= series.len() {
        return Err(VaultError::DegenerateCut {
            cut: Cut::Index(cut),
            n: series.len(),
        });
    }
    Ok(Partition {
        train: series[..cut].to_vec(),
        vault: series[cut..].to_vec(),
        cut: Cut::Index(cut),
        n: series.len(),
    })
}

/// Splits by time: train = stamp < cut, vault = stamp >= cut.
///
/// The series has to come ordered by the stamp; out of order, the temporal cut is not a cut and the
/// operation is refused.
pub fn split_by_time<T: Clone, F: Fn(&T) -> i64>(
    series: &[T],
    cut: i64,
    timestamp: F,
) -> Result<Partition<T>, VaultError> {
    if series.is_empty() {
        return Err(VaultError::EmptySeries);
    }
    let mut previous: Option<i64> = None;
    for (position, item) in series.iter().enumerate() {
        let current = timestamp(item);
        if let Some(p) = previous
            && current < p
        {
            return Err(VaultError::UnorderedSeries {
                position,
                previous: p,
                current,
            });
        }
        previous = Some(current);
    }
    let first_of_vault = series
        .iter()
        .position(|x| timestamp(x) >= cut)
        .unwrap_or(series.len());
    if first_of_vault == 0 || first_of_vault >= series.len() {
        return Err(VaultError::DegenerateCut {
            cut: Cut::Time(cut),
            n: series.len(),
        });
    }
    Ok(Partition {
        train: series[..first_of_vault].to_vec(),
        vault: series[first_of_vault..].to_vec(),
        cut: Cut::Time(cut),
        n: series.len(),
    })
}

/// Declares vault and train at once: the partition and the vault side already under custody.
pub fn declare_vault<T: Clone>(
    id: &str,
    specification: &str,
    series: &[T],
    cut: usize,
) -> Result<(Partition<T>, Vault<T>), VaultError> {
    let partition = split_by_index(series, cut)?;
    let vault = Vault::new(id, Cut::Index(cut), specification, partition.vault.clone())?;
    Ok((partition, vault))
}

/// The reading returned by the single opening: the content leaves the vault once, and the hash stays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opening<T> {
    pub vault: String,
    /// Position of the opening event in the ledger.
    pub order: u64,
    pub hash: String,
    pub merkle_root: String,
    pub agent: String,
    pub cut: Cut,
    pub pre_registration_hash: String,
    pub items: Vec<T>,
}

impl<T> Opening<T> {
    pub fn n_items(&self) -> usize {
        self.items.len()
    }
}

/// Future partition under custody, opened a single time and audited in the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vault<T> {
    pub id: String,
    pub cut: Cut,
    /// SHA-256 hash of the specification frozen before any reading.
    pub pre_registration_hash: String,
    content: Vec<T>,
    agent: Option<String>,
    opened: bool,
    ledger: Ledger,
}

impl<T> Vault<T> {
    pub fn n_items(&self) -> usize {
        self.content.len()
    }

    pub fn is_open(&self) -> bool {
        self.opened
    }

    /// Accepted openings: it has to be 0 or 1, never 2.
    pub fn openings(&self) -> u64 {
        self.ledger
            .events
            .iter()
            .filter(|e| e.card_id == OPENING_CARD)
            .count() as u64
    }

    /// Recorded refusals: second opening and late correction.
    pub fn refusals(&self) -> u64 {
        self.ledger
            .events
            .iter()
            .filter(|e| e.card_id == REFUSAL_CARD)
            .count() as u64
    }

    pub fn ledger(&self) -> &Ledger {
        &self.ledger
    }

    pub fn merkle_root(&self) -> String {
        self.ledger.merkle_root()
    }

    fn first_opening(&self) -> Option<(u64, String)> {
        self.ledger
            .events
            .iter()
            .find(|e| e.card_id == OPENING_CARD)
            .map(|e| (e.index, e.current_hash.clone()))
    }

    /// Canonical content of the opening event: it binds the pre-registration to the ledger.
    fn opening_content(&self, agent: &str) -> String {
        format!(
            "opened|{}|{}|{}|{}|{}",
            self.id,
            self.cut.label(),
            self.pre_registration_hash,
            self.content.len(),
            agent
        )
    }

    /// Recomputes the chain and charges the vault itself: at most one accepted opening, and the
    /// declared state has to match the recorded event.
    pub fn verify(&self) -> bool {
        if !self.ledger.verify() {
            return false;
        }
        let mut accepted = self
            .ledger
            .events
            .iter()
            .filter(|e| e.card_id == OPENING_CARD);
        let first = accepted.next();
        if accepted.next().is_some() {
            return false;
        }
        match (first, self.opened) {
            (None, false) => true,
            (Some(event), true) => match &self.agent {
                Some(a) => event.content == self.opening_content(a),
                None => false,
            },
            _ => false,
        }
    }
}

impl<T: Clone> Vault<T> {
    /// Declares the vault: stores the content and freezes the pre-registration by its hash. Nothing
    /// has been read yet.
    pub fn new(
        id: &str,
        cut: Cut,
        specification: &str,
        content: Vec<T>,
    ) -> Result<Self, VaultError> {
        if content.is_empty() {
            return Err(VaultError::EmptyVault);
        }
        Ok(Vault {
            id: id.to_string(),
            cut,
            pre_registration_hash: sha256_hex(specification.as_bytes()),
            content,
            agent: None,
            opened: false,
            ledger: Ledger::new(id),
        })
    }

    /// Opens the vault. On the second attempt it returns an error, records the refusal and delivers
    /// nothing new.
    pub fn open(&mut self, agent: &str) -> Result<Opening<T>, VaultError> {
        if agent.is_empty() {
            return Err(VaultError::AgentNotDeclared);
        }
        if self.opened {
            let (order, hash) = self.first_opening().unwrap_or((u64::MAX, "0".repeat(64)));
            self.ledger.record(
                REFUSAL_CARD,
                &format!("second_opening|{}|agent={}|first={}", self.id, agent, hash),
            );
            return Err(VaultError::AlreadyOpened {
                opening: order,
                hash,
            });
        }
        let content = self.opening_content(agent);
        let order = self.ledger.events.len() as u64;
        let hash = self.ledger.record(OPENING_CARD, &content);
        self.opened = true;
        self.agent = Some(agent.to_string());
        Ok(Opening {
            vault: self.id.clone(),
            order,
            hash,
            merkle_root: self.merkle_root(),
            agent: agent.to_string(),
            cut: self.cut.clone(),
            pre_registration_hash: self.pre_registration_hash.clone(),
            items: self.content.clone(),
        })
    }

    /// The hard rule, in code: once opened, an adjustment on the same data is refused and the
    /// attempt stays in the record. The legitimate fix is a new vault.
    pub fn correct(&mut self, agent: &str, adjustment: &str) -> Result<(), VaultError> {
        if !self.opened {
            return Err(VaultError::NotOpened);
        }
        let (order, hash) = self.first_opening().unwrap_or((u64::MAX, "0".repeat(64)));
        self.ledger.record(
            REFUSAL_CARD,
            &format!(
                "late_correction|{}|agent={}|adjustment={}|first={}",
                self.id, agent, adjustment, hash
            ),
        );
        Err(VaultError::NewVaultNeeded {
            vault: self.id.clone(),
            opening: order,
            hash,
        })
    }

    /// The successor: a new id and a new pre-registration, born closed. Failing here is the gate.
    pub fn successor(
        &self,
        id: &str,
        cut: Cut,
        specification: &str,
        content: Vec<T>,
    ) -> Result<Vault<T>, VaultError> {
        if id == self.id {
            return Err(VaultError::VaultReused { id: id.to_string() });
        }
        if sha256_hex(specification.as_bytes()) == self.pre_registration_hash {
            return Err(VaultError::RepeatedPreRegistration {
                vault: id.to_string(),
            });
        }
        Vault::new(id, cut, specification, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("d{:03}", i)).collect()
    }

    /// Frozen specification of the test vault, declared before any reading.
    const SPEC13: &str = "spec: flights Oct-Dec 2013, G2 declared before opening";

    fn test_vault() -> Vault<String> {
        let (_, vault) = declare_vault("h3.voos13", SPEC13, &series(10), 6)
            .expect("new vault with four units on the vault side");
        vault
    }

    #[test]
    fn splitting_by_index_and_by_time_do_not_overlap() {
        let s = series(10);
        let p = split_by_index(&s, 6).expect("valid cut");
        assert_eq!(p.n_train(), 6);
        assert_eq!(p.n_vault(), 4);
        assert_eq!(p.n, 10);
        assert!(p.is_clean());
        assert!(!p.overlaps());
        assert_eq!(p.cut, Cut::Index(6));
        assert_eq!(p.train[0], "d000");
        assert_eq!(p.vault[3], "d009");

        // temporal cut: the vault starts at t >= cut, like the t_disp of H3
        let items: Vec<(i64, String)> = s
            .iter()
            .enumerate()
            .map(|(i, x)| (i as i64 * 10, x.clone()))
            .collect();
        let q = split_by_time(&items, 30, |x| x.0).expect("valid cut");
        assert_eq!(q.n_train(), 3);
        assert_eq!(q.n_vault(), 7);
        assert!(q.is_clean());
        assert!(q.train.iter().all(|x| x.0 < 30));
        assert!(q.vault.iter().all(|x| x.0 >= 30));

        // the declared vault keeps exactly the vault side, and is born closed
        let (partition, vault) = declare_vault("h3.voos23", "spec: flights Oct-Dec 2023", &s, 6)
            .expect("valid declaration");
        assert_eq!(vault.n_items(), partition.n_vault());
        assert!(!vault.is_open());
        assert_eq!(vault.openings(), 0);
        assert_eq!(
            vault.pre_registration_hash,
            sha256_hex("spec: flights Oct-Dec 2023".as_bytes())
        );
    }

    #[test]
    fn degenerate_cut_empty_series_and_unordered_series_are_refused() {
        let s = series(5);
        assert_eq!(
            split_by_index(&s, 0),
            Err(VaultError::DegenerateCut {
                cut: Cut::Index(0),
                n: 5
            })
        );
        assert_eq!(
            split_by_index(&s, 5),
            Err(VaultError::DegenerateCut {
                cut: Cut::Index(5),
                n: 5
            })
        );
        let empty: &[String] = &[];
        assert_eq!(split_by_index(empty, 1), Err(VaultError::EmptySeries));

        let unordered = vec![(0i64, "a"), (10i64, "b"), (5i64, "c")];
        assert_eq!(
            split_by_time(&unordered, 10, |x| x.0),
            Err(VaultError::UnorderedSeries {
                position: 2,
                previous: 10,
                current: 5
            })
        );
        let without_vault = vec![(10i64, "a"), (20i64, "b")];
        assert_eq!(
            split_by_time(&without_vault, 30, |x| x.0),
            Err(VaultError::DegenerateCut {
                cut: Cut::Time(30),
                n: 2
            })
        );

        // a vault without content and a reused vault also fail
        assert_eq!(
            Vault::new("empty", Cut::Index(1), "spec", Vec::<String>::new()),
            Err(VaultError::EmptyVault)
        );
    }

    #[test]
    fn a_single_opening_and_the_second_failure_is_recorded() {
        let mut c = test_vault();
        assert!(!c.is_open());
        let a = c.open("F3").expect("first opening");
        assert_eq!(a.n_items(), 4);
        assert_eq!(a.items, vec!["d006", "d007", "d008", "d009"]);
        assert_eq!(a.order, 0);
        assert_eq!(a.vault, "h3.voos13");
        assert_eq!(a.hash, c.ledger().events[0].current_hash);
        assert_eq!(a.merkle_root, c.merkle_root());
        assert_eq!(c.openings(), 1);
        assert_eq!(c.refusals(), 0);
        assert!(c.is_open());
        assert!(c.verify());

        let root_before = c.merkle_root();
        match c.open("F3") {
            Err(VaultError::AlreadyOpened { opening, hash }) => {
                assert_eq!(opening, 0);
                assert_eq!(hash, a.hash);
            }
            other => panic!("the second opening should fail, got {:?}", other),
        }
        assert_eq!(c.openings(), 1, "never two accepted openings");
        assert_eq!(c.refusals(), 1);
        assert_ne!(
            c.merkle_root(),
            root_before,
            "the refusal enters the record"
        );
        assert!(c.verify());
        let last = c.ledger().events.last().expect("refusal recorded");
        assert_eq!(last.card_id, REFUSAL_CARD);
        assert!(last.content.contains("second_opening"));
        assert_eq!(last.index, 1);

        // without a declared agent the opening does not even start: nothing is recorded
        let mut fresh = test_vault();
        assert_eq!(fresh.open(""), Err(VaultError::AgentNotDeclared));
        assert!(!fresh.is_open());
        assert_eq!(fresh.ledger().events.len(), 0);
        assert!(fresh.verify());
    }

    #[test]
    fn tampering_with_the_record_invalidates_the_verification() {
        let mut c = test_vault();
        c.open("F3").expect("first opening");
        assert!(c.verify());

        // (i) content of the opening event rewritten: the chain falls
        let mut tampered = c.clone();
        tampered.ledger.events[0].content =
            "opened|h3.voos13|against_the_specification".to_string();
        assert!(!tampered.verify());

        // (ii) the refusal record is also audited: deleting the attempt brings everything down
        assert!(c.open("F3").is_err());
        assert!(c.verify());
        let mut tampered2 = c.clone();
        let last = tampered2.ledger.events.len() - 1;
        tampered2.ledger.events[last].content = "second_opening|never_happened".to_string();
        assert!(!tampered2.verify());

        // (iii) changing the pre-registration outside the ledger is caught too
        let mut tampered3 = c.clone();
        tampered3.pre_registration_hash =
            sha256_hex(b"pre-registration rewritten after the result");
        assert!(!tampered3.verify());

        // (iv) deleting the opening leaves the vault state lying
        let mut tampered4 = c.clone();
        tampered4.ledger.events[0].card_id = "cofre.outro".to_string();
        assert!(!tampered4.verify());

        // (v) and the intact vault keeps passing
        assert!(c.verify());
    }

    #[test]
    fn the_merkle_root_changes_when_the_content_changes() {
        let spec = "spec: flights Oct-Dec 2013";
        let mut a = Vault::new("vault-a", Cut::Index(6), spec, series(10)).expect("a");
        let mut b = Vault::new("vault-a", Cut::Index(6), spec, series(10)).expect("b");
        a.open("F3").expect("opening of a");
        b.open("F3").expect("opening of b");
        // same declaration, same content: reproducible root
        assert_eq!(a.merkle_root(), b.merkle_root());
        assert_eq!(a.merkle_root(), a.ledger().merkle_root());
        let hashes: Vec<String> = a
            .ledger()
            .events
            .iter()
            .map(|e| e.current_hash.clone())
            .collect();
        assert_eq!(crate::audit::merkle_root(&hashes), a.merkle_root());

        // extra content in the record (the refusal): the root changes
        assert!(b.open("F3").is_err());
        assert_ne!(a.merkle_root(), b.merkle_root());

        // another agent in the opening: different content, different root
        let mut d = Vault::new("vault-a", Cut::Index(6), spec, series(10)).expect("d");
        d.open("F1").expect("opening of d");
        assert_ne!(a.merkle_root(), d.merkle_root());

        // different content under custody: different root
        let mut e = Vault::new("vault-a", Cut::Index(6), spec, series(9)).expect("e");
        e.open("F3").expect("opening of e");
        assert_ne!(a.merkle_root(), e.merkle_root());
    }

    #[test]
    fn erring_after_the_opening_requires_a_new_vault_and_not_an_adjustment() {
        // the rule of the project, declared and charged here
        assert!(VAULT_RULE.contains("new vault"));

        let mut c = test_vault();
        // closed, there is nothing to correct
        assert_eq!(c.correct("F3", "delta=-0.027"), Err(VaultError::NotOpened));

        c.open("F3").expect("first opening");
        let root = c.merkle_root();
        // adjusting on the same data: refused, and the temptation stays in the record
        match c.correct("F3", "delta=-0.027") {
            Err(VaultError::NewVaultNeeded {
                vault,
                opening,
                hash,
            }) => {
                assert_eq!(vault, "h3.voos13");
                assert_eq!(opening, 0);
                assert_eq!(hash, c.ledger().events[0].current_hash);
            }
            other => panic!(
                "a post-opening adjustment should be refused, got {:?}",
                other
            ),
        }
        assert_eq!(c.refusals(), 1);
        assert_eq!(c.openings(), 1);
        assert!(c.verify());
        assert_ne!(c.merkle_root(), root);
        assert!(c.ledger().events[1].content.contains("late_correction"));

        // the only legitimate exit: a new vault, with a new id and a new pre-registration
        assert_eq!(
            c.successor("h3.voos13", Cut::Index(6), "new spec", series(10)),
            Err(VaultError::VaultReused {
                id: "h3.voos13".to_string()
            })
        );
        assert_eq!(
            c.successor("h3.voos23", Cut::Index(6), SPEC13, series(10)),
            Err(VaultError::RepeatedPreRegistration {
                vault: "h3.voos23".to_string()
            })
        );
        let mut successor = c
            .successor(
                "h3.voos23",
                Cut::Index(6),
                "spec: flights Oct-Dec 2023, same quarter of 2013",
                series(10),
            )
            .expect("new vault");
        assert!(!successor.is_open());
        assert_ne!(successor.pre_registration_hash, c.pre_registration_hash);
        assert_eq!(successor.openings(), 0);
        successor.open("F3").expect("opening of the successor");
        assert!(successor.verify());

        // and the record of the old vault was not rewritten
        assert_eq!(c.openings(), 1);
        assert_eq!(c.refusals(), 1);
        assert!(c.verify());
    }
}
