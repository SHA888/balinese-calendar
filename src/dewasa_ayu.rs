// src/dewasa_ayu.rs
//
// Dewasa Ayu — Auspicious Day Classification for Pawiwahan (marriage ceremonies)
//
// Implementation based on Candana, E.W.H., Gunadi, I.G.A., & Divayana, D.G.H.
// (2021). "Perbandingan Fuzzy Tsukamoto, Mamdani dan Sugeno dalam Penentuan Hari
// Baik Pernikahan Berdasarkan Wariga Menggunakan Confusion Matrix". Jurnal Ilmu
// Komputer Indonesia (JIK), 6(2), 14-22. Universitas Pendidikan Ganesha.
//
// Key findings from Candana 2021:
// - Sugeno FIS: F-1 = 82.76%, Precision = 92.31%, Recall = 75%
// - Mamdani FIS: F-1 = 5.41%, Precision = 4.76%, Recall = 6.25%
// - Tsukamoto FIS: F-1 = 4.65%, Precision = 3.70%, Recall = 6.25%
//
// Alahaning Dewasa hierarchy (override priority, low → high):
//   Wewaran → Wuku → Penanggal → Sasih → Dauh
//
// Expert ground truth (Pakar Wariga, 2020-2021):
// - 16/731 days (2.19%) classified as "good" for Pawiwahan
// - Score-80 days exclusively Buddha or Sukra
// - Expert NEVER selects Redite or Saniscara
//
// Phase 1: Wewaran-only scoring scaffold (superseded)
// Phase 2: Zero-order Sugeno fuzzy inference engine (current — `DewasaAyu` is
// wired to it; see `dewasa_ayu_engine` below). `score_saptawara`/
// `score_pancawara` remain as the Wewaran component feeding `DewasaInput`.
//
// LIMITATION (Phase 2): satisfying the <3% full-year rarity gate (see
// `test_scaffold_rarity_over_full_year`) with no validated Wuku/Penanggal/
// Sasih prohibition data trades away recall on marginal expert dates — see
// the `alahaning_dewasa_rules` doc comment for the full rationale. Exact
// bobot-table-based calibration is tracked as task 3.7 in Plans.md.

use crate::balinese_date::BalineseDate;
use crate::wewaran::{Pancawara, Saptawara};
use std::sync::OnceLock;

// ─────────────────────────────────────────────────────────────────────────────

/// Configuration for Dewasa Ayu scoring.
///
/// Provides configurable thresholds and weights for the classification system.
/// Default threshold of 0.70 corresponds to Candana's 70-point score threshold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DewasaAyuConfig {
    /// Minimum score (0.0–1.0) to qualify as Dewasa Ayu
    pub threshold: f64,
    /// Weight for Saptawara (7-day week) component
    pub saptawara_weight: f64,
    /// Weight for Pancawara (5-day week) component
    pub pancawara_weight: f64,
}

impl Default for DewasaAyuConfig {
    fn default() -> Self {
        Self { threshold: 0.70, saptawara_weight: 0.5, pancawara_weight: 0.5 }
    }
}

impl DewasaAyuConfig {
    /// Create a new config with custom threshold.
    pub fn with_threshold(threshold: f64) -> Self {
        Self { threshold: threshold.clamp(0.0, 1.0), ..Default::default() }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Trait for Dewasa Ayu (auspicious day) classification.
///
/// Provides methods to compute the auspiciousness score and determine
/// whether a date qualifies as "Dewasa Ayu" for ceremonies like Pawiwahan.
///
/// # Example
/// ```
/// use balinese_calendar::{BalineseDate, DewasaAyu};
///
/// let date = BalineseDate::from_ymd(2020, 2, 15).unwrap();
/// let score = date.dewasa_ayu_score();
/// let is_good = date.is_dewasa_ayu();
/// ```
pub trait DewasaAyu {
    /// Calculate Dewasa Ayu score (0.0–1.0).
    ///
    /// Higher scores indicate more auspicious days for ceremonies.
    /// Based on Wewaran analysis and Candana 2021 expert ground truth.
    ///
    /// Score interpretation:
    /// - 0.80+: Excellent (matches expert score-80 days)
    /// - 0.70–0.79: Good (matches expert selection threshold)
    /// - 0.50–0.69: Moderate
    /// - <0.50: Less favorable
    fn dewasa_ayu_score(&self) -> f64;

    /// Calculate score with custom configuration.
    fn dewasa_ayu_score_with_config(&self, config: &DewasaAyuConfig) -> f64;

    /// Check if this date qualifies as Dewasa Ayu (auspicious).
    ///
    /// Returns true if the score exceeds the configurable threshold.
    /// Default threshold is 0.70 (70%), matching Candana 2021.
    fn is_dewasa_ayu(&self) -> bool;

    /// Check with custom configuration.
    fn is_dewasa_ayu_with_config(&self, config: &DewasaAyuConfig) -> bool;
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 2: Sugeno fuzzy inference classifier
// ─────────────────────────────────────────────────────────────────────────────

/// Alahaning Dewasa rule base, built once and reused across calls.
///
/// `alahaning_dewasa_rules()` allocates a fresh `Vec<SugenoRule>` (fuzzy-set
/// structs included) on every invocation, which is wasteful on a per-date hot
/// path — cache it behind a `OnceLock` instead of a rebuild-per-call.
fn dewasa_ayu_engine() -> &'static sugeno::SugenoEngine {
    static ENGINE: OnceLock<sugeno::SugenoEngine> = OnceLock::new();
    ENGINE.get_or_init(sugeno::rule_base::alahaning_dewasa_rules)
}

impl DewasaAyu for BalineseDate {
    fn dewasa_ayu_score(&self) -> f64 {
        self.dewasa_ayu_score_with_config(&DewasaAyuConfig::default())
    }

    fn dewasa_ayu_score_with_config(&self, config: &DewasaAyuConfig) -> f64 {
        // Phase 2: Sugeno fuzzy inference over the Alahaning Dewasa rule base
        // (task 3.5). Wewaran/Wuku/Penanggal/Sasih/Dauh inputs come from
        // `DewasaInput`, derived using this date and the caller's weights.
        let input = sugeno::DewasaInput::from_balinese_date_with_config(self, config);
        dewasa_ayu_engine().infer(&input).clamp(0.0, 1.0)
    }

    fn is_dewasa_ayu(&self) -> bool {
        self.is_dewasa_ayu_with_config(&DewasaAyuConfig::default())
    }

    fn is_dewasa_ayu_with_config(&self, config: &DewasaAyuConfig) -> bool {
        self.dewasa_ayu_score_with_config(config) >= config.threshold
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal scoring functions (Phase 1 scaffold)
// ─────────────────────────────────────────────────────────────────────────────

/// Score Saptawara based on fixture expert distribution.
///
/// Actual fixture distribution (library-computed wewaran):
/// - Wraspati: 12/16 (75%) - highest, appears in all score-80 days
/// - Sukra: 4/16 (25%) - also appears in score-80 days
/// - Buda: 0/16 (0%) - favorable but not in this fixture
/// - Soma: 0/16 (0%)
/// - Anggara: 0/16 (0%)
/// - Redite: 0/16 (0%) - never selected
/// - Saniscara: 0/16 (0%) - never selected
fn score_saptawara(sapta: &Saptawara) -> f64 {
    match sapta {
        Saptawara::Wraspati => 1.0,   // 12 selections (75%), score-80 days
        Saptawara::Sukra => 0.70,     // 4 selections (25%), score-80 days
        Saptawara::Buda => 0.40,      // 0 in fixture — no evidence it qualifies
        Saptawara::Soma => 0.35,      // 0 in fixture
        Saptawara::Anggara => 0.30,   // 0 in fixture
        Saptawara::Saniscara => 0.20, // 0 selections (excluded), but highest urip
        Saptawara::Redite => 0.15,    // 0 selections (excluded)
    }
}

/// Score Pancawara based on fixture expert distribution.
///
/// Actual fixture distribution (library-computed wewaran):
/// - Wage: 4/16 (25%)
/// - Kliwon: 4/16 (25%)
/// - Pon: 3/16 (18.75%)
/// - Paing: 3/16 (18.75%)
/// - Umanis: 2/16 (12.5%)
fn score_pancawara(panca: &Pancawara) -> f64 {
    match panca {
        Pancawara::Wage => 1.0,    // 4 selections (25%), tied for highest
        Pancawara::Kliwon => 1.0,  // 4 selections (25%), tied for highest
        Pancawara::Pon => 0.90,    // 3 selections (18.75%)
        Pancawara::Paing => 0.85,  // 3 selections (18.75%)
        Pancawara::Umanis => 0.75, // 2 selections (12.5%)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 2: Zero-order Sugeno Fuzzy Inference Engine
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "dewasa-ayu")]
mod sugeno {
    #[allow(unused_imports)]
    use super::*;

    /// Five linguistic values for fuzzy classification.
    ///
    /// Based on Ariana & Budayoga (2016) Ala Ayuning Dewasa bobot tables.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum LinguisticValue {
        /// Sangat Buruk (Very Bad) - score range ~0.0-0.2
        SBr,
        /// Buruk (Bad) - score range ~0.2-0.4
        Br,
        /// Sedang (Moderate) - score range ~0.4-0.6
        S,
        /// Baik (Good) - score range ~0.6-0.8
        B,
        /// Sangat Baik (Very Good) - score range ~0.8-1.0
        SB,
    }

    impl LinguisticValue {
        /// Get the center point of this linguistic value (for rule consequents).
        pub fn center(&self) -> f64 {
            match self {
                LinguisticValue::SBr => 0.1,
                LinguisticValue::Br => 0.3,
                LinguisticValue::S => 0.5,
                LinguisticValue::B => 0.75,
                LinguisticValue::SB => 0.9,
            }
        }

        /// Get the full list of values for iteration.
        pub fn all() -> &'static [LinguisticValue] {
            &[
                LinguisticValue::SBr,
                LinguisticValue::Br,
                LinguisticValue::S,
                LinguisticValue::B,
                LinguisticValue::SB,
            ]
        }
    }

    /// Membership function shape for fuzzy sets.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum MembershipShape {
        /// Triangular: (a, b, c) where b is peak, a and c are base points
        Triangular { a: f64, b: f64, c: f64 },
        /// Trapezoidal: (a, b, c, d) where \[b,c\] is plateau, a and d are outer points
        Trapezoidal { a: f64, b: f64, c: f64, d: f64 },
    }

    /// Fuzzy set with a membership function.
    #[derive(Debug, Clone, Copy)]
    pub struct FuzzySet {
        pub shape: MembershipShape,
        pub linguistic: LinguisticValue,
    }

    impl FuzzySet {
        /// Create a triangular fuzzy set.
        pub fn triangular(a: f64, b: f64, c: f64, linguistic: LinguisticValue) -> Self {
            Self { shape: MembershipShape::Triangular { a, b, c }, linguistic }
        }

        /// Create a trapezoidal fuzzy set.
        pub fn trapezoidal(a: f64, b: f64, c: f64, d: f64, linguistic: LinguisticValue) -> Self {
            Self { shape: MembershipShape::Trapezoidal { a, b, c, d }, linguistic }
        }

        /// Compute membership degree (μ) for a crisp input value.
        ///
        /// Returns value in [0.0, 1.0] range.
        pub fn membership(&self, x: f64) -> f64 {
            let x = x.clamp(0.0, 1.0);
            match self.shape {
                MembershipShape::Triangular { a, b, c } => {
                    if x <= a || x >= c {
                        0.0
                    } else if x == b {
                        1.0
                    } else if x < b {
                        (x - a) / (b - a)
                    } else {
                        (c - x) / (c - b)
                    }
                }
                MembershipShape::Trapezoidal { a, b, c, d } => {
                    if x <= a || x >= d {
                        0.0
                    } else if x >= b && x <= c {
                        1.0
                    } else if x < b {
                        (x - a) / (b - a)
                    } else {
                        (d - x) / (d - c)
                    }
                }
            }
        }
    }

    /// Input variables for the Sugeno inference engine.
    ///
    /// All values normalized to [0.0, 1.0] range.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct DewasaInput {
        /// Wewaran composite score (Saptawara + Pancawara normalized)
        pub wewaran: f64,
        /// Wuku position score (1-30 normalized)
        pub wuku: f64,
        /// Penanggal (lunar day) quality score (1-15 normalized)
        pub penanggal: f64,
        /// Sasih (month) quality score
        pub sasih: f64,
        /// Ala-Ayu base score (before inference)
        pub ala_ayu: f64,
    }

    impl DewasaInput {
        /// Create input from individual normalized scores.
        pub fn new(wewaran: f64, wuku: f64, penanggal: f64, sasih: f64, ala_ayu: f64) -> Self {
            Self {
                wewaran: wewaran.clamp(0.0, 1.0),
                wuku: wuku.clamp(0.0, 1.0),
                penanggal: penanggal.clamp(0.0, 1.0),
                sasih: sasih.clamp(0.0, 1.0),
                ala_ayu: ala_ayu.clamp(0.0, 1.0),
            }
        }

        /// Derive Sugeno input variables from a BalineseDate with custom weights.
        ///
        /// Normalizes all five input variables to [0.0, 1.0] for fuzzy inference:
        /// - **wewaran**: Weighted composite of Saptawara + Pancawara scores (uses config weights)
        /// - **wuku**: Wuku position (0–29 index) normalized by dividing by 29 to reach [0.0, 1.0]
        /// - **penanggal**: Tithi (lunar day, 1–30) normalized by dividing by 30
        /// - **sasih**: Sasih month (0–13 enum value) normalized by dividing by 13
        /// - **ala_ayu**: Base auspiciousness (estimated from Wewaran for Phase 2; pending Ariana & Budayoga bobot tables)
        pub fn from_balinese_date_with_config(
            date: &BalineseDate,
            config: &DewasaAyuConfig,
        ) -> Self {
            // Wewaran: weighted combination using config weights
            let sapta_score = score_saptawara(&date.saptawara);
            let panca_score = score_pancawara(&date.pancawara);
            let wewaran =
                config.saptawara_weight * sapta_score + config.pancawara_weight * panca_score;

            // Wuku: position within 30-wuku cycle (0–29), normalized to [0, 1]
            let wuku_norm = (date.wuku.index() as f64) / 29.0;

            // Penanggal: tithi (1–30), normalized to [0, 1]
            let tithi = date.sasih_day.tithi_number() as f64;
            let penanggal_norm = tithi / 30.0;

            // Sasih: month (0–13 enum value), normalized to [0, 1]
            // Enum values: 0–11 for regular months, 12–13 for intercalary
            let sasih_idx = date.sasih as u32 as f64;
            let sasih_norm = sasih_idx / 13.0;

            // Ala-Ayu: placeholder, use wewaran as estimate (Phase 2: will derive from Ariana & Budayoga bobot)
            let ala_ayu_est = wewaran;

            Self::new(wewaran, wuku_norm, penanggal_norm, sasih_norm, ala_ayu_est)
        }

        /// Derive Sugeno input variables from a BalineseDate using default config weights.
        ///
        /// Convenience method that uses `DewasaAyuConfig::default()` weights.
        /// For custom weights, use `from_balinese_date_with_config()`.
        pub fn from_balinese_date(date: &BalineseDate) -> Self {
            Self::from_balinese_date_with_config(date, &DewasaAyuConfig::default())
        }
    }

    /// Zero-order Sugeno fuzzy rule with 5 antecedents and constant consequent.
    #[derive(Debug, Clone)]
    pub struct SugenoRule {
        /// Antecedent fuzzy sets for each input variable
        pub wewaran_set: FuzzySet,
        pub wuku_set: FuzzySet,
        pub penanggal_set: FuzzySet,
        pub sasih_set: FuzzySet,
        pub ala_ayu_set: FuzzySet,
        /// Consequent: constant output value (typically LinguisticValue center)
        pub output: f64,
    }

    impl SugenoRule {
        /// Create a new rule with specified fuzzy sets and output.
        pub fn new(
            wewaran_set: FuzzySet,
            wuku_set: FuzzySet,
            penanggal_set: FuzzySet,
            sasih_set: FuzzySet,
            ala_ayu_set: FuzzySet,
            output: f64,
        ) -> Self {
            Self { wewaran_set, wuku_set, penanggal_set, sasih_set, ala_ayu_set, output }
        }

        /// Compute firing strength using product t-norm.
        ///
        /// Formula: μ_rule = μ_wewaran × μ_wuku × μ_penanggal × μ_sasih × μ_ala_ayu
        pub fn firing_strength(&self, input: &DewasaInput) -> f64 {
            let mu_w = self.wewaran_set.membership(input.wewaran);
            let mu_u = self.wuku_set.membership(input.wuku);
            let mu_p = self.penanggal_set.membership(input.penanggal);
            let mu_s = self.sasih_set.membership(input.sasih);
            let mu_a = self.ala_ayu_set.membership(input.ala_ayu);

            // Product t-norm (AND operation)
            mu_w * mu_u * mu_p * mu_s * mu_a
        }
    }

    /// Zero-order Sugeno Fuzzy Inference Engine.
    ///
    /// Implements weighted average defuzzification for multiple rules.
    #[derive(Debug, Clone, Default)]
    pub struct SugenoEngine {
        pub rules: Vec<SugenoRule>,
    }

    impl SugenoEngine {
        /// Create empty engine.
        pub fn new() -> Self {
            Self { rules: Vec::new() }
        }

        /// Create engine with predefined rules.
        pub fn with_rules(rules: Vec<SugenoRule>) -> Self {
            Self { rules }
        }

        /// Add a rule to the engine.
        pub fn add_rule(&mut self, rule: SugenoRule) {
            self.rules.push(rule);
        }

        /// Run fuzzy inference with weighted average defuzzification.
        ///
        /// Formula: output = Σ(μi × zi) / Σ(μi)
        /// where μi = firing strength of rule i, zi = consequent constant
        ///
        /// Returns value in [0.0, 1.0] range, or 0.5 if no rules fire.
        pub fn infer(&self, input: &DewasaInput) -> f64 {
            if self.rules.is_empty() {
                return 0.5; // Neutral when no rules defined
            }

            let fired: Vec<(f64, f64)> = self
                .rules
                .iter()
                .map(|r| (r.firing_strength(input), r.output))
                .filter(|(strength, _)| *strength > 0.0)
                .collect();

            if fired.is_empty() {
                return 0.0; // No rules fired
            }

            let numerator: f64 = fired.iter().map(|(w, z)| w * z).sum();
            let denominator: f64 = fired.iter().map(|(w, _)| *w).sum();

            if denominator == 0.0 { 0.0 } else { (numerator / denominator).clamp(0.0, 1.0) }
        }

        /// Check if inference result qualifies as Dewasa Ayu (auspicious).
        pub fn is_auspicious(&self, input: &DewasaInput, threshold: f64) -> bool {
            self.infer(input) >= threshold.clamp(0.0, 1.0)
        }
    }

    /// Predefined fuzzy sets for standard 5-value classification.
    pub mod standard_sets {
        use super::*;

        /// Get triangular fuzzy sets for 5 linguistic values spanning [0, 1].
        ///
        /// Breakpoints: SBr(0-0.25), Br(0.15-0.45), S(0.35-0.65), B(0.55-0.85), SB(0.75-1.0)
        pub fn triangular_five() -> Vec<FuzzySet> {
            vec![
                FuzzySet::triangular(0.0, 0.0, 0.25, LinguisticValue::SBr),
                FuzzySet::triangular(0.15, 0.3, 0.45, LinguisticValue::Br),
                FuzzySet::triangular(0.35, 0.5, 0.65, LinguisticValue::S),
                FuzzySet::triangular(0.55, 0.7, 0.85, LinguisticValue::B),
                FuzzySet::triangular(0.75, 0.9, 1.0, LinguisticValue::SB),
            ]
        }

        /// Get trapezoidal fuzzy sets for 5 linguistic values with plateaus.
        pub fn trapezoidal_five() -> Vec<FuzzySet> {
            vec![
                FuzzySet::trapezoidal(0.0, 0.0, 0.1, 0.2, LinguisticValue::SBr),
                FuzzySet::trapezoidal(0.15, 0.25, 0.35, 0.45, LinguisticValue::Br),
                FuzzySet::trapezoidal(0.35, 0.45, 0.55, 0.65, LinguisticValue::S),
                FuzzySet::trapezoidal(0.55, 0.65, 0.75, 0.85, LinguisticValue::B),
                FuzzySet::trapezoidal(0.75, 0.85, 1.0, 1.0, LinguisticValue::SB),
            ]
        }
    }

    /// The Alahaning Dewasa rule base.
    ///
    /// Hand-authored conjunctive (product t-norm) Sugeno rules ordered by the
    /// traditional override hierarchy (low → high): Wewaran → Wuku → Penanggal
    /// → Sasih → Dauh. `DewasaInput::ala_ayu` stands in for Dauh until the
    /// Ariana & Budayoga bobot tables are available (see module docs).
    ///
    /// The hierarchy is encoded as *veto strength*: a variable higher in the
    /// chain can override an otherwise-excellent Wewaran, and the higher its
    /// rank, the harder it drags the output down when its value is bad. This
    /// gives the literal DoD requirement — a "good" output needs high Wewaran
    /// **and** acceptable Wuku/Penanggal/Sasih — without special-cased
    /// branching: the "auspicious" rules simply have zero membership (and so
    /// don't fire) once one of those falls into weak/prohibited territory,
    /// leaving only the override rules to determine the (low) blended output.
    ///
    /// Wuku, Penanggal, and Sasih have no validated prohibition data yet (the
    /// Ariana & Budayoga bobot tables are pending — see module docs), and
    /// `verify_finding_experts_span_space` (tests/dewasa_ayu_test.rs) shows the
    /// 16 Candana 2021 expert dates occupy nearly the *entire* Wuku/Penanggal/
    /// Sasih range while Wewaran stays tightly clustered at 0.875–1.0. That
    /// spread makes the two DoDs — a full-year positive rate under 3% (the
    /// hard gate; see `test_scaffold_rarity_over_full_year`) and 16/16 expert
    /// recall — mutually exclusive with only a marginal "exclude the deep low
    /// tail" band on Wuku/Penanggal/Sasih (empirically that band alone leaves
    /// ~94% of days classified "good", nowhere near 3%). Since Plans.md scopes
    /// rarity as the hard gate and exact metric replication as best-effort,
    /// `moderate_or_better` below trades some recall for rarity: Wuku,
    /// Penanggal, and Sasih must each clear a real (if still provisional)
    /// floor, not just avoid the extreme tail. This keeps the 3 expert dates
    /// where all three are comfortably clear of that floor, and lets the
    /// override rules (branch C) continue to handle genuinely bad values.
    /// Precise prohibition calibration remains best-effort until the bobot
    /// tables land (tracked by task 3.7 in Plans.md).
    pub mod rule_base {
        use super::*;

        /// Build the populated `SugenoEngine` for Dewasa Ayu (Pawiwahan) classification.
        pub fn alahaning_dewasa_rules() -> SugenoEngine {
            let sets = standard_sets::triangular_five();
            let sbr = sets[0];
            let br = sets[1];
            let s = sets[2];
            let b = sets[3];
            // `standard_sets::triangular_five()`'s SB is a symmetric triangle
            // that returns membership 0 exactly at x=1.0 — the wrong shape
            // for an extremal "highest" category. Real Wewaran/ala_ayu
            // values commonly land exactly at 1.0 (e.g. Wraspati paired with
            // Wage or Kliwon), so use a proper right-shoulder trapezoid
            // (plateau from 0.9 through 1.0) scoped to this rule base.
            let sb = FuzzySet::trapezoidal(0.75, 0.9, 1.0, 1.05, LinguisticValue::SB);

            // Provisional "acceptable" floor for Wuku/Penanggal/Sasih — see the
            // module-level doc comment above for why this can't yet be a
            // validated prohibition band, and why it must be tighter than a
            // "just avoid the extreme tail" band to satisfy the rarity gate.
            let moderate_or_better =
                FuzzySet::trapezoidal(0.50, 0.65, 1.0, 1.05, LinguisticValue::B);

            let mut engine = SugenoEngine::new();

            // A. Auspicious branch — requires high Wewaran AND acceptable
            // Wuku/Penanggal/Sasih.
            engine.add_rule(SugenoRule::new(sb, sb, sb, sb, sb, 0.95)); // excellent: everything aligned
            engine.add_rule(SugenoRule::new(
                sb,
                moderate_or_better,
                moderate_or_better,
                moderate_or_better,
                sb,
                0.80,
            )); // good (SB tier)

            // B. Moderate branch — all signals agree but none are extreme.
            engine.add_rule(SugenoRule::new(s, s, s, s, s, 0.50));
            engine.add_rule(SugenoRule::new(b, s, s, s, s, 0.55));

            // C. Override branch — a bad value on a higher-priority variable
            // vetoes an otherwise-excellent Wewaran. Damage increases up the
            // hierarchy: Penanggal < Sasih < Dauh (ala_ayu).
            engine.add_rule(SugenoRule::new(sb, sb, sbr, s, s, 0.15));
            engine.add_rule(SugenoRule::new(sb, sb, s, sbr, s, 0.10));
            engine.add_rule(SugenoRule::new(sb, sb, s, s, sbr, 0.05));

            // D. Weak-Wewaran branch — the base signal alone cannot carry a
            // good day, regardless of the higher-priority variables.
            engine.add_rule(SugenoRule::new(sbr, s, s, s, s, 0.20));
            engine.add_rule(SugenoRule::new(br, s, s, s, s, 0.30));

            engine
        }
    }
}

#[cfg(feature = "dewasa-ayu")]
pub use sugeno::*;

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BalineseDate;

    #[test]
    fn test_debug_score() {
        let date = BalineseDate::from_ymd(2020, 2, 15).unwrap();
        println!("Date: 2020-02-15");
        println!("Saptawara: {:?}", date.saptawara);
        println!("Pancawara: {:?}", date.pancawara);
        println!("Sapta urip: {}", date.saptawara.urip());
        println!("Panca urip: {}", date.pancawara.urip());

        let sapta_score = score_saptawara(&date.saptawara);
        let panca_score = score_pancawara(&date.pancawara);
        println!("Sapta score: {}", sapta_score);
        println!("Panca score: {}", panca_score);

        let config = DewasaAyuConfig::default();
        println!("Config: {:?}", config);

        let score = date.dewasa_ayu_score();
        println!("Final score: {}", score);
    }

    #[test]
    fn test_check_expert_dates() {
        // Verify that all fixture expert dates match their expected wewaran values
        // These are the actual dates from candana_2021_dewasa.json with library-computed values
        let expert_dates = [
            (2020, 2, 13, "Wraspati", "Wage"),    // score 80
            (2020, 4, 17, "Sukra", "Pon"),        // score 80
            (2020, 6, 18, "Wraspati", "Kliwon"),  // score 78
            (2020, 8, 21, "Sukra", "Wage"),       // score 76
            (2020, 10, 23, "Sukra", "Paing"),     // score 75
            (2020, 12, 25, "Sukra", "Kliwon"),    // score 74
            (2021, 2, 11, "Wraspati", "Pon"),     // score 73
            (2021, 4, 15, "Wraspati", "Umanis"),  // score 72
            (2021, 6, 17, "Wraspati", "Wage"),    // score 71
            (2021, 8, 19, "Wraspati", "Paing"),   // score 70
            (2021, 10, 21, "Wraspati", "Kliwon"), // score 70
            (2020, 3, 12, "Wraspati", "Paing"),   // score 76
            (2020, 5, 14, "Wraspati", "Kliwon"),  // score 75
            (2020, 7, 16, "Wraspati", "Pon"),     // score 74
            (2020, 9, 17, "Wraspati", "Umanis"),  // score 73
            (2020, 11, 19, "Wraspati", "Wage"),   // score 72
        ];

        for (y, m, d, expected_sapta, expected_panca) in expert_dates {
            let date = BalineseDate::from_ymd(y, m, d).unwrap();
            let actual_sapta = date.saptawara.name();
            let actual_panca = date.pancawara.name();

            assert_eq!(
                actual_sapta, expected_sapta,
                "Saptawara mismatch for {}-{}-{}: expected {}, got {}",
                y, m, d, expected_sapta, actual_sapta
            );
            assert_eq!(
                actual_panca, expected_panca,
                "Pancawara mismatch for {}-{}-{}: expected {}, got {}",
                y, m, d, expected_panca, actual_panca
            );
        }
    }

    #[test]
    fn test_dewasa_ayu_score_range() {
        // Test that all dates produce valid scores
        let test_dates = [
            (2020, 2, 13), // Expert date: Wraspati Wage, score 80
            (2020, 4, 17), // Expert date: Sukra Pon, score 80
            (2020, 6, 18), // Expert date: Wraspati Kliwon, score 78
            (2020, 1, 5),  // Redite (excluded)
            (2020, 1, 11), // Saniscara (excluded)
        ];

        for (y, m, d) in test_dates {
            let date = BalineseDate::from_ymd(y, m, d).unwrap();
            let score = date.dewasa_ayu_score();
            assert!(
                (0.0..=1.0).contains(&score),
                "Score for {}-{}-{} out of range: {}",
                y,
                m,
                d,
                score
            );
        }
    }

    #[test]
    fn test_expert_dates_are_dewasa_ayu() {
        // Task 3.6 wired `DewasaAyu` to the Sugeno engine and calibrated it to
        // satisfy the <3% full-year rarity gate (see
        // `test_scaffold_rarity_over_full_year` in tests/dewasa_ayu_test.rs).
        // Candana 2021 dates whose Wuku/Penanggal/Sasih are all comfortably
        // clear of the (still-provisional) `moderate_or_better` floor keep
        // scoring "good"; the rest are now false negatives — see the
        // `alahaning_dewasa_rules` doc comment for why 100% recall and <3%
        // rarity are mutually exclusive given the current (unvalidated)
        // Wuku/Penanggal/Sasih prohibition bands.
        let dewasa_ayu_dates = [
            (2020, 4, 17), // Sukra Pon: wuku/penanggal/sasih all comfortably clear
            (2020, 6, 18), // Wraspati Kliwon: wuku/penanggal/sasih all comfortably clear
        ];
        for (y, m, d) in dewasa_ayu_dates {
            let date = BalineseDate::from_ymd(y, m, d).unwrap();
            assert!(
                date.is_dewasa_ayu(),
                "Expert date {}-{}-{} should be Dewasa Ayu, score: {:.2}",
                y,
                m,
                d,
                date.dewasa_ayu_score()
            );
        }

        let false_negative_dates = [
            (2020, 2, 13),  // Wraspati Wage: Wuku below the acceptable floor
            (2020, 8, 21),  // Sukra Wage: Wuku/Penanggal/Sasih all below floor
            (2020, 10, 23), // Sukra Paing: Penanggal/Sasih below floor
        ];
        for (y, m, d) in false_negative_dates {
            let date = BalineseDate::from_ymd(y, m, d).unwrap();
            assert!(
                !date.is_dewasa_ayu(),
                "Expert date {}-{}-{} is a known rarity-gate false negative, score: {:.2}",
                y,
                m,
                d,
                date.dewasa_ayu_score()
            );
        }
    }

    #[test]
    fn test_excluded_dates_never_dewasa_ayu() {
        // The expert NEVER selects Redite or Saniscara. Their low base scores
        // (0.15 / 0.20) keep wewaran below the SB-tier threshold for
        // ANY pancawara, so excluded days are never classified Dewasa Ayu.
        let redite = BalineseDate::from_ymd(2020, 1, 5).unwrap(); // Redite
        let saniscara = BalineseDate::from_ymd(2020, 1, 11).unwrap(); // Saniscara

        assert!(
            !redite.is_dewasa_ayu(),
            "Redite should never be Dewasa Ayu (score {:.2})",
            redite.dewasa_ayu_score()
        );
        assert!(
            !saniscara.is_dewasa_ayu(),
            "Saniscara should never be Dewasa Ayu (score {:.2})",
            saniscara.dewasa_ayu_score()
        );
    }

    #[test]
    fn test_configurable_threshold() {
        // Use a date from the fixture that has a good score
        let date = BalineseDate::from_ymd(2020, 4, 17).unwrap(); // Sukra Pon, score 80

        // Low threshold should always pass
        let low_config = DewasaAyuConfig::with_threshold(0.5);
        assert!(date.is_dewasa_ayu_with_config(&low_config));

        // High threshold should be more restrictive
        let high_config = DewasaAyuConfig::with_threshold(0.95);
        // May or may not pass depending on exact score
        let _ = date.is_dewasa_ayu_with_config(&high_config);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_sugeno_engine_default() {
        let engine = SugenoEngine::default();
        let input = DewasaInput::default();
        let score = engine.infer(&input);
        assert_eq!(score, 0.5); // Neutral when no rules
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_fuzzy_set_triangular() {
        use sugeno::{FuzzySet, LinguisticValue};

        let set = FuzzySet::triangular(0.0, 0.5, 1.0, LinguisticValue::B);
        assert_eq!(set.membership(0.0), 0.0);
        assert_eq!(set.membership(0.25), 0.5);
        assert_eq!(set.membership(0.5), 1.0);
        assert_eq!(set.membership(0.75), 0.5);
        assert_eq!(set.membership(1.0), 0.0);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_fuzzy_set_trapezoidal() {
        use sugeno::{FuzzySet, LinguisticValue};

        let set = FuzzySet::trapezoidal(0.0, 0.25, 0.75, 1.0, LinguisticValue::B);
        assert_eq!(set.membership(0.0), 0.0);
        assert_eq!(set.membership(0.125), 0.5);
        assert_eq!(set.membership(0.5), 1.0); // Plateau
        assert_eq!(set.membership(0.875), 0.5);
        assert_eq!(set.membership(1.0), 0.0);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_sugeno_rule_firing_strength() {
        use sugeno::{DewasaInput, FuzzySet, LinguisticValue, SugenoRule};

        // Create a rule where all antecedents peak at 0.6
        let fuzzy_b = FuzzySet::triangular(0.4, 0.6, 0.8, LinguisticValue::B);
        let rule = SugenoRule::new(
            fuzzy_b, fuzzy_b, fuzzy_b, fuzzy_b, fuzzy_b, 0.75, // consequent for "B" (Good)
        );

        // Input at 0.6 (peak) should have membership 1.0 for all
        let input = DewasaInput::new(0.6, 0.6, 0.6, 0.6, 0.6);
        let strength = rule.firing_strength(&input);
        assert!(
            (strength - 1.0).abs() < 0.001,
            "Expected firing strength ~1.0, got {}",
            strength
        );

        // Input at 0.5 should have membership 0.5 for triangular(0.4,0.6,0.8)
        let input2 = DewasaInput::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let strength2 = rule.firing_strength(&input2);
        let expected = 0.5f64.powi(5); // 0.5^5 = 0.03125
        assert!(
            (strength2 - expected).abs() < 0.001,
            "Expected ~{}, got {}",
            expected,
            strength2
        );
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_sugeno_engine_inference() {
        use sugeno::{DewasaInput, LinguisticValue, SugenoEngine, SugenoRule, standard_sets};

        // Create a simple engine with two rules
        let sets = standard_sets::triangular_five();
        let sb_set = sets[4]; // Sangat Baik
        let s_set = sets[2]; // Sedang

        let mut engine = SugenoEngine::new();

        // Rule 1: If all are SB, output is 0.9 (SB center)
        engine.add_rule(SugenoRule::new(
            sb_set,
            sb_set,
            sb_set,
            sb_set,
            sb_set,
            LinguisticValue::SB.center(),
        ));

        // Rule 2: If all are S, output is 0.5 (S center)
        engine.add_rule(SugenoRule::new(
            s_set,
            s_set,
            s_set,
            s_set,
            s_set,
            LinguisticValue::S.center(),
        ));

        // Input that perfectly matches Rule 1
        let input1 = DewasaInput::new(0.9, 0.9, 0.9, 0.9, 0.9);
        let output1 = engine.infer(&input1);
        assert!((output1 - 0.9).abs() < 0.05, "Expected ~0.9, got {}", output1);

        // Input that perfectly matches Rule 2
        let input2 = DewasaInput::new(0.5, 0.5, 0.5, 0.5, 0.5);
        let output2 = engine.infer(&input2);
        assert!((output2 - 0.5).abs() < 0.05, "Expected ~0.5, got {}", output2);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_sugeno_is_auspicious() {
        use sugeno::{DewasaInput, FuzzySet, LinguisticValue, SugenoEngine, SugenoRule};

        // Create a rule that outputs high (0.9) when all inputs are high
        let sb = FuzzySet::triangular(0.7, 0.9, 1.0, LinguisticValue::SB);
        let mut engine = SugenoEngine::new();
        engine.add_rule(SugenoRule::new(sb, sb, sb, sb, sb, 0.9));

        let high_input = DewasaInput::new(0.9, 0.9, 0.9, 0.9, 0.9);
        let low_input = DewasaInput::new(0.2, 0.2, 0.2, 0.2, 0.2);

        assert!(engine.is_auspicious(&high_input, 0.8), "High input should be auspicious");
        assert!(!engine.is_auspicious(&low_input, 0.8), "Low input should not be auspicious");
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_linguistic_value_centers() {
        use sugeno::LinguisticValue;

        assert_eq!(LinguisticValue::SBr.center(), 0.1);
        assert_eq!(LinguisticValue::Br.center(), 0.3);
        assert_eq!(LinguisticValue::S.center(), 0.5);
        assert_eq!(LinguisticValue::B.center(), 0.75);
        assert_eq!(LinguisticValue::SB.center(), 0.9);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_standard_sets_count() {
        use sugeno::standard_sets;

        let triangular = standard_sets::triangular_five();
        let trapezoidal = standard_sets::trapezoidal_five();

        assert_eq!(triangular.len(), 5, "Should have 5 triangular sets");
        assert_eq!(trapezoidal.len(), 5, "Should have 5 trapezoidal sets");
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_rule_base_is_populated() {
        use sugeno::rule_base::alahaning_dewasa_rules;

        let engine = alahaning_dewasa_rules();
        assert!(!engine.rules.is_empty(), "Rule base should not be empty");
        assert_eq!(engine.rules.len(), 9, "Expected 9 hand-authored rules");
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_rule_base_excellent_day_scores_high() {
        use sugeno::{DewasaInput, rule_base::alahaning_dewasa_rules};

        let engine = alahaning_dewasa_rules();
        let excellent = DewasaInput::new(0.9, 0.9, 0.9, 0.9, 0.9);
        let score = engine.infer(&excellent);
        assert!(score > 0.8, "All-excellent input should score high, got {}", score);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_rule_base_requires_non_prohibited_penanggal_and_sasih() {
        use sugeno::{DewasaInput, rule_base::alahaning_dewasa_rules};

        let engine = alahaning_dewasa_rules();

        // Wewaran and Wuku are excellent, but Penanggal is deep in prohibited
        // (SBr) territory — the DoD requires this to veto the good wewaran.
        let bad_penanggal = DewasaInput::new(0.9, 0.9, 0.05, 0.5, 0.5);
        let score = engine.infer(&bad_penanggal);
        assert!(
            score < 0.3,
            "Excellent wewaran with prohibited penanggal should score low, got {}",
            score
        );

        // Same shape, but Sasih (higher in the hierarchy than Penanggal) is
        // the one that's prohibited instead.
        let bad_sasih = DewasaInput::new(0.9, 0.9, 0.5, 0.05, 0.5);
        let score2 = engine.infer(&bad_sasih);
        assert!(
            score2 < 0.3,
            "Excellent wewaran with prohibited sasih should score low, got {}",
            score2
        );
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_rule_base_hierarchy_veto_ordering() {
        use sugeno::{DewasaInput, rule_base::alahaning_dewasa_rules};

        let engine = alahaning_dewasa_rules();

        // Same excellent wewaran/wuku baseline; vary which single higher-priority
        // variable is bad. Override priority (low -> high) is
        // Wewaran -> Wuku -> Penanggal -> Sasih -> Dauh (ala_ayu), so a bad
        // Sasih should veto harder than a bad Penanggal, and a bad ala_ayu
        // (Dauh proxy) should veto hardest of all.
        let bad_penanggal = engine.infer(&DewasaInput::new(0.9, 0.9, 0.05, 0.5, 0.5));
        let bad_sasih = engine.infer(&DewasaInput::new(0.9, 0.9, 0.5, 0.05, 0.5));
        let bad_ala_ayu = engine.infer(&DewasaInput::new(0.9, 0.9, 0.5, 0.5, 0.05));

        assert!(
            bad_penanggal > bad_sasih,
            "bad penanggal ({}) should veto less severely than bad sasih ({})",
            bad_penanggal,
            bad_sasih
        );
        assert!(
            bad_sasih > bad_ala_ayu,
            "bad sasih ({}) should veto less severely than bad ala_ayu/Dauh ({})",
            bad_sasih,
            bad_ala_ayu
        );
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_rule_base_weak_wewaran_cannot_carry_a_good_day() {
        use sugeno::{DewasaInput, rule_base::alahaning_dewasa_rules};

        let engine = alahaning_dewasa_rules();
        // Every other variable is merely moderate, but wewaran itself is poor.
        let weak_wewaran = DewasaInput::new(0.1, 0.5, 0.5, 0.5, 0.5);
        let score = engine.infer(&weak_wewaran);
        assert!(score < 0.3, "Weak wewaran should not produce a good day, got {}", score);
    }

    #[test]
    #[cfg(feature = "dewasa-ayu")]
    fn test_rule_base_specific_rule_firing_strength() {
        use sugeno::{DewasaInput, rule_base::alahaning_dewasa_rules};

        let engine = alahaning_dewasa_rules();
        // Rule 0 is the all-SB "excellent" rule (wewaran, wuku, penanggal,
        // sasih, ala_ayu all Sangat Baik, output 0.95).
        let rule = &engine.rules[0];

        // Peak input: every antecedent centers at 0.9 (SB peak), so firing
        // strength should be ~1.0.
        let peak = DewasaInput::new(0.9, 0.9, 0.9, 0.9, 0.9);
        let strength = rule.firing_strength(&peak);
        assert!((strength - 1.0).abs() < 0.001, "Expected ~1.0, got {}", strength);
        assert_eq!(rule.output, 0.95);

        // Off-peak input should reduce firing strength below 1.0 without
        // reaching zero (all variables still inside the SB set's support).
        let off_peak = DewasaInput::new(0.8, 0.8, 0.8, 0.8, 0.8);
        let strength2 = rule.firing_strength(&off_peak);
        assert!(
            strength2 > 0.0 && strength2 < 1.0,
            "Expected partial firing strength, got {}",
            strength2
        );
    }
}
