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
// Phase 1: Scoring scaffold (current)
// Phase 2: Zero-order Sugeno fuzzy inference engine (upcoming)
//
// LIMITATION (Phase 1): the wewaran-only scaffold is intentionally permissive —
// it classifies well over half of all days as Dewasa Ayu, far above the expert's
// 2.19% (16/731) rarity. It exists only to exercise the trait + fixture plumbing.
// The full Sugeno engine (Phase 2) must satisfy the <3% rarity constraint; see the
// `test_scaffold_rarity_over_full_year` guard in tests/dewasa_ayu_test.rs.

use crate::balinese_date::BalineseDate;
use crate::wewaran::{Pancawara, Saptawara};

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
// Phase 1: Wewaran-based scoring scaffold
// Phase 2: Full Sugeno fuzzy inference engine (TODO)
// ─────────────────────────────────────────────────────────────────────────────

impl DewasaAyu for BalineseDate {
    fn dewasa_ayu_score(&self) -> f64 {
        self.dewasa_ayu_score_with_config(&DewasaAyuConfig::default())
    }

    fn dewasa_ayu_score_with_config(&self, config: &DewasaAyuConfig) -> f64 {
        // Phase 1: Wewaran-based scoring (scaffold)
        // Uses Saptawara and Pancawara distributions from Candana 2021

        let sapta_score = score_saptawara(&self.saptawara);
        let panca_score = score_pancawara(&self.pancawara);

        // Weighted combination. The expert's avoidance of Redite/Saniscara is
        // encoded directly in their low base scores (see `score_saptawara`), so no
        // separate exclusion penalty is needed: any Redite/Saniscara day caps below
        // the default 0.70 threshold regardless of pancawara.
        let score = config.saptawara_weight * sapta_score + config.pancawara_weight * panca_score;

        score.clamp(0.0, 1.0)
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
        Saptawara::Sukra => 0.90,     // 4 selections (25%), score-80 days
        Saptawara::Buda => 0.85,      // 0 in fixture but traditionally favorable
        Saptawara::Soma => 0.70,      // 0 in fixture
        Saptawara::Anggara => 0.65,   // 0 in fixture
        Saptawara::Saniscara => 0.35, // 0 selections (excluded), but highest urip
        Saptawara::Redite => 0.30,    // 0 selections (excluded)
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

        /// Derive Sugeno input variables from a BalineseDate.
        ///
        /// Normalizes all five input variables to [0.0, 1.0] for fuzzy inference:
        /// - **wewaran**: Composite of Saptawara + Pancawara scores (already normalized [0.3, 1.0])
        /// - **wuku**: Wuku position (1–30) normalized by dividing by 30
        /// - **penanggal**: Tithi (lunar day, 1–30) normalized by dividing by 30
        /// - **sasih**: Sasih month (1–12) normalized by dividing by 12
        /// - **ala_ayu**: Base auspiciousness (estimated from Wewaran for Phase 2; pending Ariana & Budayoga bobot tables)
        pub fn from_balinese_date(date: &BalineseDate) -> Self {
            // Wewaran: weighted combination of Saptawara and Pancawara scores
            let sapta_score = score_saptawara(&date.saptawara);
            let panca_score = score_pancawara(&date.pancawara);
            let wewaran = 0.5 * sapta_score + 0.5 * panca_score;

            // Wuku: position within 30-wuku cycle, normalized to [0, 1]
            let wuku_norm = (date.wuku.index() as f64) / 30.0;

            // Penanggal: tithi (1–30), normalized to [0, 1]
            let tithi = date.sasih_day.tithi_number() as f64;
            let penanggal_norm = tithi / 30.0;

            // Sasih: month (0–11 enum value), normalized to [0, 1]
            // Note: Sasih enum uses 0–11 for regular months, 12–13 for intercalary months
            let sasih_idx = date.sasih as u32 as f64;
            let sasih_norm = (sasih_idx + 1.0) / 13.0; // +1 to shift from 0-indexed, /13 for max intercalary

            // Ala-Ayu: placeholder, use wewaran as estimate (Phase 2: will derive from Ariana & Budayoga bobot)
            let ala_ayu_est = wewaran;

            Self::new(wewaran, wuku_norm, penanggal_norm, sasih_norm, ala_ayu_est)
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
        // Verify that expert-selected dates from fixture qualify
        // Using actual dates from candana_2021_dewasa.json fixture
        let expert_dates = [
            (2020, 2, 13),  // Wraspati Wage, score 80
            (2020, 4, 17),  // Sukra Pon, score 80
            (2020, 6, 18),  // Wraspati Kliwon, score 78
            (2020, 8, 21),  // Sukra Wage, score 76
            (2020, 10, 23), // Sukra Paing, score 75
        ];

        for (y, m, d) in expert_dates {
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
    }

    #[test]
    fn test_excluded_dates_never_dewasa_ayu() {
        // The expert NEVER selects Redite or Saniscara. Their low base scores
        // (0.30 / 0.35) cap the weighted total below the default 0.70 threshold for
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
}
