//! `[patterns]` section (FT-070, ADR-050) — pattern body completeness check.

use serde::{Deserialize, Serialize};

/// `[patterns]` section in `product.toml`. Mirrors `[features]` (W030).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternsConfig {
    /// Top-level H2 headings every live pattern body must contain.
    #[serde(rename = "body-sections", default = "default_body_sections")]
    pub body_sections: Vec<String>,
    /// Severity of body-section warnings — `warning` (default) or `error`.
    #[serde(rename = "body-severity", default)]
    pub body_severity: PatternBodySeverity,
}

impl Default for PatternsConfig {
    fn default() -> Self {
        Self {
            body_sections: default_body_sections(),
            body_severity: PatternBodySeverity::default(),
        }
    }
}

fn default_body_sections() -> Vec<String> {
    vec![
        "When to use".into(),
        "Prerequisites".into(),
        "The pattern".into(),
        "Anti-patterns".into(),
        "Worked example".into(),
    ]
}

/// `[patterns].body-severity` — `warning` by default.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PatternBodySeverity {
    #[default]
    Warning,
    Error,
}
