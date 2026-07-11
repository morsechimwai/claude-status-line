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
    /// Named color preset: "orange" (default), "blue", "green", "purple", "mono".
    pub preset: String,
    /// Overrides for the preset (256-color index). `None` -> use the preset.
    pub fill: Option<u8>,
    pub track: Option<u8>,
    pub dim: Option<u8>,
}

/// (fill, track, dim) for a named preset. Unknown names fall back to orange.
fn preset_colors(name: &str) -> (u8, u8, u8) {
    match name.trim().to_lowercase().as_str() {
        "blue" => (68, 17, 245),    // Claude usage-panel blue on navy
        "green" => (71, 22, 245),
        "purple" => (140, 53, 245),
        "mono" => (245, 238, 245),
        _ => (173, 240, 245), // orange (Claude brand) — default + fallback
    }
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
    /// Plan/tier label shown next to the model name. If set, it is used
    /// verbatim (e.g. "Max (20x)"). If empty and `plan_auto` is true, it is
    /// auto-detected from `~/.claude.json`.
    pub plan: String,
    /// Auto-detect the plan from `~/.claude.json` when `plan` is empty.
    pub plan_auto: bool,
}

impl Default for Colors {
    fn default() -> Self {
        Self { preset: "orange".into(), fill: None, track: None, dim: None }
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
        Self { model_header: true, plan: String::new(), plan_auto: true }
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
        let (p_fill, p_track, p_dim) = preset_colors(&self.colors.preset);
        Style {
            fill: self.colors.fill.unwrap_or(p_fill),
            track: self.colors.track.unwrap_or(p_track),
            dim: self.colors.dim.unwrap_or(p_dim),
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
        assert_eq!(c.colors.preset, "orange");
        let s = c.style();
        assert_eq!(s.fill, 173); // Claude-brand orange by default
        assert_eq!(s.track, 240);
        assert_eq!(s.dim, 245);
        assert_eq!(c.bar.width, 12);
        assert_eq!(c.bar.filled, "█");
        assert_eq!(c.bar.empty, "░");
        assert!(c.bar.braille); // hi-res braille bar on by default
        assert!(c.rows.context && c.rows.current && c.rows.weekly);
        assert_eq!(c.labels.context, "Context");
        assert_eq!(c.labels.current, "5h");
        assert_eq!(c.labels.weekly, "7d");
        assert!(c.layout.model_header);
        assert!(c.layout.plan_auto);
    }

    #[test]
    fn preset_and_override() {
        // preset picks a whole scheme...
        let blue = Config::from_toml("[colors]\npreset = \"blue\"\n").style();
        assert_eq!(blue.fill, 68);
        assert_eq!(blue.track, 17);
        // ...and a raw override wins over the preset for that channel only.
        let c = Config::from_toml("[colors]\npreset = \"blue\"\nfill = 99\n");
        let s = c.style();
        assert_eq!(s.fill, 99); // overridden
        assert_eq!(s.track, 17); // still the blue preset's track
        // an unknown preset falls back to orange.
        let unknown = Config::from_toml("[colors]\npreset = \"chartreuse\"\n").style();
        assert_eq!(unknown.fill, 173);
        assert_eq!(unknown.track, 240);
    }

    #[test]
    fn parses_partial_override() {
        let c = Config::from_toml("[bar]\nbraille = false\n[layout]\nmodel_header = false\n");
        assert_eq!(c.style().fill, 173); // untouched -> orange default
        assert!(!c.bar.braille);
        assert_eq!(c.bar.width, 12); // untouched default
        assert!(!c.layout.model_header);
    }

    #[test]
    fn malformed_toml_yields_default() {
        let c = Config::from_toml("this is not toml =========");
        assert_eq!(c.style().fill, 173);
    }

    #[test]
    fn style_maps_fields() {
        let s = Config::default().style();
        assert_eq!(s.fill, 173);
        assert_eq!(s.width, 12);
        assert_eq!(s.empty, "░");
        assert!(s.braille);
    }
}
