use crate::render::Style;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub colors: Colors,
    pub bar: Bar,
    pub rows: Rows,
    pub labels: Labels,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Colors {
    pub fill: u8,
    pub track: u8,
    pub dim: u8,
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
    pub current: String,
    pub weekly: String,
}

impl Default for Colors {
    fn default() -> Self {
        Self { fill: 173, track: 240, dim: 245 }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self { width: 12, filled: "█".into(), empty: "█".into() }
    }
}

impl Default for Rows {
    fn default() -> Self {
        Self { context: true, current: true, weekly: true }
    }
}

impl Default for Labels {
    fn default() -> Self {
        Self { current: "Current".into(), weekly: "Weekly".into() }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors::default(),
            bar: Bar::default(),
            rows: Rows::default(),
            labels: Labels::default(),
        }
    }
}

impl Config {
    pub fn from_toml(s: &str) -> Config {
        toml::from_str(s).unwrap_or_default()
    }

    pub fn style(&self) -> Style {
        Style {
            fill: self.colors.fill,
            track: self.colors.track,
            dim: self.colors.dim,
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
    fn defaults_match_original() {
        let c = Config::default();
        assert_eq!(c.colors.fill, 173);
        assert_eq!(c.colors.track, 240);
        assert_eq!(c.colors.dim, 245);
        assert_eq!(c.bar.width, 12);
        assert_eq!(c.bar.filled, "█");
        assert_eq!(c.bar.empty, "█");
        assert!(c.rows.context && c.rows.current && c.rows.weekly);
        assert_eq!(c.labels.current, "Current");
        assert_eq!(c.labels.weekly, "Weekly");
    }

    #[test]
    fn parses_partial_override() {
        let c = Config::from_toml("[colors]\nfill = 99\n[bar]\nempty = \"░\"\n");
        assert_eq!(c.colors.fill, 99);
        assert_eq!(c.colors.track, 240); // untouched default
        assert_eq!(c.bar.empty, "░");
        assert_eq!(c.bar.width, 12); // untouched default
    }

    #[test]
    fn malformed_toml_yields_default() {
        let c = Config::from_toml("this is not toml =========");
        assert_eq!(c.colors.fill, 173);
    }

    #[test]
    fn style_maps_fields() {
        let c = Config::default();
        let s = c.style();
        assert_eq!(s.fill, 173);
        assert_eq!(s.width, 12);
        assert_eq!(s.filled, "█");
    }
}
