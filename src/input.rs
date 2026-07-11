use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Input {
    #[serde(default)]
    pub model: Model,
    #[serde(default)]
    pub context_window: ContextWindow,
    #[serde(default)]
    pub rate_limits: RateLimits,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    #[serde(default = "default_model")]
    pub display_name: String,
}

fn default_model() -> String {
    "Claude".to_string()
}

impl Default for Model {
    fn default() -> Self {
        Self { display_name: default_model() }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct ContextWindow {
    #[serde(default)]
    pub used_percentage: f64,
    #[serde(default)]
    pub context_window_size: f64,
    #[serde(default)]
    pub total_input_tokens: f64,
    #[serde(default)]
    pub total_output_tokens: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct RateLimits {
    pub five_hour: Option<Window>,
    pub seven_day: Option<Window>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Window {
    #[serde(default)]
    pub used_percentage: Option<f64>,
    #[serde(default)]
    pub resets_at: Option<i64>,
}

pub fn parse(s: &str) -> Input {
    serde_json::from_str(s).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = r#"{
        "model": {"display_name": "Opus 4.8"},
        "context_window": {
            "used_percentage": 12.5,
            "context_window_size": 1000000,
            "total_input_tokens": 100000,
            "total_output_tokens": 25000
        },
        "rate_limits": {
            "five_hour": {"used_percentage": 42.0, "resets_at": 1705320000},
            "seven_day": {"used_percentage": 18.0, "resets_at": 1705320000}
        }
    }"#;

    #[test]
    fn parses_full() {
        let i = parse(FULL);
        assert_eq!(i.model.display_name, "Opus 4.8");
        assert_eq!(i.context_window.context_window_size, 1_000_000.0);
        assert_eq!(i.context_window.total_input_tokens, 100_000.0);
        let f = i.rate_limits.five_hour.unwrap();
        assert_eq!(f.used_percentage, Some(42.0));
        assert_eq!(f.resets_at, Some(1_705_320_000));
    }

    #[test]
    fn missing_rate_limits_defaults() {
        let i = parse(r#"{"model": {"display_name": "X"}}"#);
        assert_eq!(i.model.display_name, "X");
        assert!(i.rate_limits.five_hour.is_none());
        assert!(i.rate_limits.seven_day.is_none());
    }

    #[test]
    fn null_percentage_is_none_not_error() {
        let i = parse(r#"{"rate_limits":{"five_hour":{"used_percentage":null,"resets_at":123}}}"#);
        let f = i.rate_limits.five_hour.unwrap();
        assert_eq!(f.used_percentage, None);
        assert_eq!(f.resets_at, Some(123));
    }

    #[test]
    fn malformed_returns_default() {
        let i = parse("not json at all");
        assert_eq!(i.model.display_name, "Claude");
        assert_eq!(i.context_window.context_window_size, 0.0);
    }
}
