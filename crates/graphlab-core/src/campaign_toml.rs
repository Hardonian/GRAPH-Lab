//! Campaign TOML parsing (file format for `campaigns/*.toml`).
//!
//! Schema (v1):
//! ```toml
//! name = "hadwiger-n6-exhaustive"
//! predicate = "hadwiger-lower-bound-v1"
//! mode = "exhaustive-unlabeled"   # | "exhaustive-labeled" | "sampled"
//! n_min = 0
//! n_max = 6
//! # seed = 42
//! # samples = 100000
//! # p = 0.5
//! threads = 48
//! max_witnesses = 0
//! ```

use crate::campaign::Campaign;
use crate::search::DomainMode;

#[derive(Debug, thiserror::Error)]
pub enum CampaignError {
    #[error("toml parse: {0}")]
    Toml(String),
    #[error("invalid field: {0}")]
    Invalid(String),
}

impl Campaign {
    pub fn from_toml_str(s: &str) -> Result<Self, CampaignError> {
        let v: toml::Value =
            toml::from_str(s).map_err(|e: toml::de::Error| CampaignError::Toml(e.to_string()))?;
        let t = v
            .as_table()
            .ok_or_else(|| CampaignError::Toml("not a table".into()))?;
        let get_str = |k: &str| -> Result<String, CampaignError> {
            t.get(k)
                .and_then(|x| x.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| CampaignError::Invalid(format!("missing string field {k}")))
        };
        let get_usize = |k: &str| -> Result<usize, CampaignError> {
            t.get(k)
                .and_then(|x| x.as_integer())
                .and_then(|x| usize::try_from(x).ok())
                .ok_or_else(|| CampaignError::Invalid(format!("missing int field {k}")))
        };
        let opt_u64 = |k: &str| t.get(k).and_then(|x| x.as_integer()).map(|x| x as u64);
        let opt_usize = |k: &str| t.get(k).and_then(|x| x.as_integer()).map(|x| x as usize);
        let opt_f64 = |k: &str| t.get(k).and_then(|x| x.as_float());
        let mode = match get_str("mode")?.as_str() {
            "exhaustive-unlabeled" => DomainMode::ExhaustiveUnlabeled,
            "exhaustive-labeled" => DomainMode::ExhaustiveLabeled,
            "sampled" => DomainMode::Sampled,
            other => return Err(CampaignError::Invalid(format!("unknown mode {other}"))),
        };
        let c = Campaign {
            name: get_str("name")?,
            predicate: get_str("predicate")?,
            mode,
            n_min: get_usize("n_min")?,
            n_max: get_usize("n_max")?,
            seed: opt_u64("seed"),
            samples: opt_usize("samples"),
            p: opt_f64("p"),
            threads: opt_usize("threads"),
            max_witnesses: opt_usize("max_witnesses").unwrap_or(0),
        };
        if matches!(c.mode, DomainMode::Sampled)
            && (c.seed.is_none() || c.samples.is_none() || c.p.is_none())
        {
            return Err(CampaignError::Invalid(
                "sampled mode requires seed, samples, p".into(),
            ));
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_baseline_campaign() {
        let s = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../campaigns/baseline.toml"
        ))
        .unwrap();
        let c = Campaign::from_toml_str(&s).unwrap();
        assert_eq!(c.mode, DomainMode::ExhaustiveUnlabeled);
        assert!(c.n_max <= 6);
    }

    #[test]
    fn rejects_bad_mode_and_missing_fields() {
        assert!(Campaign::from_toml_str(
            "name=\"x\" predicate=\"p\" mode=\"nope\" n_min=0 n_max=1"
        )
        .is_err());
        assert!(Campaign::from_toml_str("name=\"x\"").is_err());
        assert!(Campaign::from_toml_str(
            "name=\"x\" predicate=\"p\" mode=\"sampled\" n_min=2 n_max=3"
        )
        .is_err());
    }
}
