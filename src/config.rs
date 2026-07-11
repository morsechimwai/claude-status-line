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
    pub layout: Layout,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Colors {
    pub track: u8,
    pub dim: u8,
    pub fill: u8,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Bar {
    pub width: usize,
    pub filled: String,
    pub empty: String,
    pub braille: bool,
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
    /// Optional plan/tier label shown next to the model name (e.g. "Max (20x)").
    /// The status-line JSON does not expose the plan, so you set it here.
    pub plan: String,
}

impl Default for Colors {
    fn default() -> Self {
        // Match the official Claude Code usage panel: medium blue fill on a
        // dark navy track.
        Self { track: 17, dim: 245, fill: 68 }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self { width: 12, filled: "█".into(), empty: "░".into(), braille: true }
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
        Self { model_header: true, plan: String::new() }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors::default(),
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
            fill: self.colors.fill,
            width: self.bar.width,
            filled: self.bar.filled.clone(),
            empty: self.bar.empty.clone(),
            braille: self.bar.braille,
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
        assert_eq!(c.colors.dim, 245);
        assert_eq!(c.colors.fill, 68); // Claude usage-panel blue
        assert_eq!(c.colors.track, 17); // dark navy track
        assert_eq!(c.bar.width, 12);
        assert_eq!(c.bar.filled, "█");
        assert_eq!(c.bar.empty, "░");
        assert!(c.bar.braille); // hi-res braille bar on by default
        assert!(c.rows.context && c.rows.current && c.rows.weekly);
        assert_eq!(c.labels.context, "Context");
        assert_eq!(c.labels.current, "5h");
        assert_eq!(c.labels.weekly, "7d");
        assert!(c.layout.model_header);
    }

    #[test]
    fn parses_partial_override() {
        let c = Config::from_toml(
            "[colors]\nfill = 99\n[bar]\nbraille = false\n[layout]\nmodel_header = false\n",
        );
        assert_eq!(c.colors.fill, 99);
        assert_eq!(c.colors.track, 17); // untouched default
        assert!(!c.bar.braille);
        assert_eq!(c.bar.width, 12); // untouched default
        assert!(!c.layout.model_header);
    }

    #[test]
    fn malformed_toml_yields_default() {
        let c = Config::from_toml("this is not toml =========");
        assert_eq!(c.colors.fill, 68);
    }

    #[test]
    fn style_maps_fields() {
        let c = Config::default();
        let s = c.style();
        assert_eq!(s.fill, 68);
        assert_eq!(s.width, 12);
        assert_eq!(s.empty, "░");
        assert!(s.braille);
    }
}
