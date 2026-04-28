//! `[features]` section (FT-055, ADR-047) — feature body completeness check.

use serde::{Deserialize, Serialize};

/// `[features]` section in `product.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    /// Top-level H2 headings every non-stub feature body must contain.
    #[serde(rename = "required-sections", default = "default_required_sections")]
    pub required_sections: Vec<String>,
    /// H3 headings required under `## Functional Specification`.
    #[serde(rename = "functional-spec-subsections", default = "default_fs_subsections")]
    pub functional_spec_subsections: Vec<String>,
    /// Features with `phase < required-from-phase` are exempt from W030.
    #[serde(rename = "required-from-phase", default = "default_required_from_phase")]
    pub required_from_phase: u32,
    /// Severity of W030 — `warning` (default) or `error`.
    #[serde(rename = "completeness-severity", default)]
    pub completeness_severity: CompletenessSeverity,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            required_sections: default_required_sections(),
            functional_spec_subsections: default_fs_subsections(),
            required_from_phase: default_required_from_phase(),
            completeness_severity: CompletenessSeverity::default(),
        }
    }
}

fn default_required_sections() -> Vec<String> {
    vec![
        "Description".into(),
        "Functional Specification".into(),
        "Out of scope".into(),
    ]
}

fn default_fs_subsections() -> Vec<String> {
    vec![
        "Inputs".into(),
        "Outputs".into(),
        "State".into(),
        "Behaviour".into(),
        "Invariants".into(),
        "Error handling".into(),
        "Boundaries".into(),
    ]
}

fn default_required_from_phase() -> u32 { 1 }

/// `[features].completeness-severity` (FT-055, ADR-047) — `warning` by default.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompletenessSeverity {
    #[default]
    Warning,
    Error,
}
