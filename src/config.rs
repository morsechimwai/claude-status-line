use crate::render::Style;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub colors: Colors,
    pub thresholds: Thresholds,
    pub bar: Bar,
    pub rows: Rows,
    pub labels: Labels,
    pub layout: Layout,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Colors {
    pub track: u8,
    pub dim: u8,
    pub good: u8,
    pub warn: u8,
    pub crit: u8,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Thresholds {
    pub warn_at: u8,
    pub crit_at: u8,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Bar {
    pub width: usize,
    pub filled: String,
    pub empty: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Rows {
    pub context: bool,
    pub current: bool,
    pub weekly: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Labels {
    pub context: String,
    pub current: String,
    pub weekly: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Layout {
    pub model_header: bool,
}

impl Default for Colors {
    fn default() -> Self {
        Self { track: 240, dim: 245, good: 71, warn: 179, crit: 167 }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self { warn_at: 50, crit_at: 80 }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self { width: 12, filled: "█".into(), empty: "░".into() }
    }
}

impl Default for Rows {
    fn default() -> Self {
        Self { context: true, current: true, weekly: true }
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self { context: "Context".into(), current: "5h".into(), weekly: "7d".into() }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self { model_header: true }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors::default(),
            thresholds: Thresholds::default(),
            bar: Bar::default(),
            rows: Rows::default(),
            labels: Labels::default(),
            layout: Layout::default(),
        }
    }
}

impl Config {
    pub fn from_toml(s: &str) -> Config {
        toml::from_str(s).unwrap_or_default()
    }

    pub fn style(&self) -> Style {
        Style {
            track: self.colors.track,
            dim: self.colors.dim,
            good: self.colors.good,
            warn: self.colors.warn,
            crit: self.colors.crit,
            warn_at: self.thresholds.warn_at,
            crit_at: self.thresholds.crit_at,
            width: self.bar.width,
            filled: self.bar.filled.clone(),
            empty: self.bar.empty.clone(),
        }
    }

    pub fn load() -> Config {
        match config_path().and_then(|p| std::fs::read_to_string(p).ok()) {
            Some(s) => Config::from_toml(&s),
            None => Config::default(),
        }
    }
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))?;
    Some(base.join("ccstatus").join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_redesign() {
        let c = Config::default();
        assert_eq!(c.colors.track, 240);
        assert_eq!(c.colors.dim, 245);
        assert_eq!(c.colors.good, 71);
        assert_eq!(c.colors.warn, 179);
        assert_eq!(c.colors.crit, 167);
        assert_eq!(c.thresholds.warn_at, 50);
        assert_eq!(c.thresholds.crit_at, 80);
        assert_eq!(c.bar.width, 12);
        assert_eq!(c.bar.filled, "█");
        assert_eq!(c.bar.empty, "░"); // light track is the new default
        assert!(c.rows.context && c.rows.current && c.rows.weekly);
        assert_eq!(c.labels.context, "Context");
        assert_eq!(c.labels.current, "5h");
        assert_eq!(c.labels.weekly, "7d");
        assert!(c.layout.model_header);
    }

    #[test]
    fn parses_partial_override() {
        let c = Config::from_toml(
            "[colors]\ngood = 99\n[thresholds]\ncrit_at = 90\n[bar]\nempty = \"█\"\n[layout]\nmodel_header = false\n",
        );
        assert_eq!(c.colors.good, 99);
        assert_eq!(c.colors.track, 240); // untouched default
        assert_eq!(c.thresholds.crit_at, 90);
        assert_eq!(c.thresholds.warn_at, 50); // untouched default
        assert_eq!(c.bar.empty, "█");
        assert!(!c.layout.model_header);
    }

    #[test]
    fn malformed_toml_yields_default() {
        let c = Config::from_toml("this is not toml =========");
        assert_eq!(c.colors.good, 71);
    }

    #[test]
    fn style_maps_fields() {
        let c = Config::default();
        let s = c.style();
        assert_eq!(s.good, 71);
        assert_eq!(s.crit, 167);
        assert_eq!(s.warn_at, 50);
        assert_eq!(s.width, 12);
        assert_eq!(s.empty, "░");
    }
}
