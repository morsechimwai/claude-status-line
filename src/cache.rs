use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CachedWindow {
    pub used_percentage: f64,
    pub resets_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CachedUsage {
    pub five_hour: Option<CachedWindow>,
    pub seven_day: Option<CachedWindow>,
}

fn cache_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".cache")))?;
    Some(base.join("ccstatus").join("usage.json"))
}

pub fn load_from(path: &Path) -> Option<CachedUsage> {
    let s = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&s).ok()
}

pub fn store_to(path: &Path, u: &CachedUsage) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(s) = serde_json::to_string(u) {
        let _ = std::fs::write(path, s);
    }
}

pub fn load() -> Option<CachedUsage> {
    load_from(&cache_path()?)
}

pub fn store(u: &CachedUsage) {
    if let Some(p) = cache_path() {
        store_to(&p, u);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("ccstatus_test_{name}")).join("usage.json")
    }

    #[test]
    fn round_trips() {
        let path = temp_file("round_trips");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
        let u = CachedUsage {
            five_hour: Some(CachedWindow { used_percentage: 42.0, resets_at: Some(123) }),
            seven_day: Some(CachedWindow { used_percentage: 18.0, resets_at: None }),
        };
        store_to(&path, &u);
        assert_eq!(load_from(&path), Some(u));
    }

    #[test]
    fn missing_file_is_none() {
        let path = temp_file("missing").join("nope.json");
        assert_eq!(load_from(&path), None);
    }

    #[test]
    fn corrupt_file_is_none() {
        let path = temp_file("corrupt");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "{ not json").unwrap();
        assert_eq!(load_from(&path), None);
    }
}
