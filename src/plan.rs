//! Best-effort auto-detection of the Claude subscription plan/tier.
//!
//! The status-line JSON does not carry the plan, but Claude Code records the
//! account's rate-limit tier in `~/.claude.json` (e.g.
//! `oauthAccount.organizationRateLimitTier = "default_claude_max_20x"`). We read
//! only that one string field and map it to a short label like `Max (20x)`.
//!
//! `~/.claude.json` can grow large (session/project history), and the status
//! line renders often, so the detected label is cached and only recomputed when
//! the account file's modification time changes. Everything here is best-effort:
//! any failure yields `None` and the plan is simply hidden.

use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

const TIER_KEYS: [&str; 3] = [
    "organizationRateLimitTier",
    "userRateLimitTier",
    "seatTier",
];

/// Detect the plan label, using a small mtime-keyed cache so `~/.claude.json` is
/// only read when it actually changes. Returns `None` if unavailable.
pub fn detect() -> Option<String> {
    let src = dirs::home_dir()?.join(".claude.json");
    let mtime = std::fs::metadata(&src)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs());

    // Serve from cache when the source file is unchanged.
    if let (Some(mt), Some((cached_mt, label))) = (mtime, read_cache()) {
        if cached_mt == mt {
            return if label.is_empty() { None } else { Some(label) };
        }
    }

    let label = detect_from_file(&src);
    if let Some(mt) = mtime {
        write_cache(mt, label.as_deref().unwrap_or(""));
    }
    label
}

/// Read `path`, extract the first usable tier field, and map it to a label.
/// Testable in isolation from the cache.
pub fn detect_from_file(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    for key in TIER_KEYS {
        if let Some(raw) = json_string_value(&text, key) {
            if let Some(label) = pretty_tier(&raw) {
                return Some(label);
            }
        }
    }
    None
}

/// Extract the first non-empty JSON string value for `key` from `text` via a
/// lightweight scan (avoids parsing the whole large account file). Skips `null`,
/// non-string, and backslash-escaped values, and keeps looking.
fn json_string_value(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let mut from = 0;
    while let Some(i) = text[from..].find(&needle) {
        let after = from + i + needle.len();
        from = after; // advance regardless, so we can retry on null values
        let rest = text[after..].trim_start();
        let Some(rest) = rest.strip_prefix(':') else {
            continue;
        };
        let rest = rest.trim_start();
        if let Some(rest) = rest.strip_prefix('"') {
            if let Some(end) = rest.find('"') {
                let val = &rest[..end];
                // Reject values that contain an escape — tier strings never do,
                // and a naive terminator scan would mis-slice them.
                if !val.is_empty() && !val.contains('\\') {
                    return Some(val.to_string());
                }
            }
        }
    }
    None
}

/// Map a raw tier string to a short display label, or `None` if it's empty.
/// `default_claude_max_20x` -> `Max (20x)`, `claude_pro` -> `Pro`, etc.
pub fn pretty_tier(raw: &str) -> Option<String> {
    let s = raw.trim().to_lowercase();
    if s.is_empty() {
        return None;
    }
    let s = s
        .strip_prefix("default_claude_")
        .or_else(|| s.strip_prefix("claude_"))
        .unwrap_or(&s);
    if let Some(rest) = s.strip_prefix("max_") {
        if !rest.is_empty() {
            return Some(format!("Max ({rest})")); // "20x" -> "Max (20x)"
        }
    }
    let label = match s {
        "max" => "Max".to_string(),
        "pro" => "Pro".to_string(),
        "free" => "Free".to_string(),
        "team" => "Team".to_string(),
        "enterprise" => "Enterprise".to_string(),
        other => {
            let spaced = other.trim_matches('_').replace('_', " ");
            let mut chars = spaced.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => return None,
            }
        }
    };
    Some(label)
}

fn cache_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".cache")))?;
    Some(base.join("ccstatus").join("plan"))
}

/// Cache format: line 1 = source mtime (secs), remainder = the label (may be empty).
fn read_cache() -> Option<(u64, String)> {
    let text = std::fs::read_to_string(cache_path()?).ok()?;
    let (first, rest) = text.split_once('\n')?;
    let mt = first.trim().parse::<u64>().ok()?;
    Some((mt, rest.trim_end_matches('\n').to_string()))
}

fn write_cache(mtime: u64, label: &str) {
    let Some(path) = cache_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, format!("{mtime}\n{label}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn maps_tier_strings() {
        assert_eq!(pretty_tier("default_claude_max_20x").as_deref(), Some("Max (20x)"));
        assert_eq!(pretty_tier("default_claude_max_5x").as_deref(), Some("Max (5x)"));
        assert_eq!(pretty_tier("claude_pro").as_deref(), Some("Pro"));
        assert_eq!(pretty_tier("default_claude_pro").as_deref(), Some("Pro"));
        assert_eq!(pretty_tier("team").as_deref(), Some("Team"));
        assert_eq!(pretty_tier("max_").as_deref(), Some("Max")); // empty suffix -> plain Max
        assert_eq!(pretty_tier("").as_deref(), None);
    }

    #[test]
    fn extracts_json_string_skipping_null_and_escapes() {
        let json = r#"{ "userRateLimitTier": null, "oauthAccount": { "organizationRateLimitTier": "default_claude_max_20x" } }"#;
        assert_eq!(
            json_string_value(json, "organizationRateLimitTier").as_deref(),
            Some("default_claude_max_20x")
        );
        assert_eq!(json_string_value(json, "userRateLimitTier"), None); // null
        assert_eq!(json_string_value(json, "nope"), None); // missing
        // an escaped-quote value is rejected rather than mis-sliced.
        let tricky = r#"{"organizationRateLimitTier": "a\"b", "userRateLimitTier": "claude_pro"}"#;
        assert_eq!(json_string_value(tricky, "organizationRateLimitTier"), None);
    }

    #[test]
    fn detect_from_file_reads_tier() {
        let dir = std::env::temp_dir().join("ccstatus_plan_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("claude.json");
        let mut f = std::fs::File::create(&path).unwrap();
        // first key is null, so detection falls through to the real value.
        write!(
            f,
            r#"{{"organizationRateLimitTier": null, "userRateLimitTier": "default_claude_max_5x"}}"#
        )
        .unwrap();
        assert_eq!(detect_from_file(&path).as_deref(), Some("Max (5x)"));

        assert_eq!(detect_from_file(&dir.join("missing.json")), None);
    }
}
