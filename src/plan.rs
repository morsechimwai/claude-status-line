//! Best-effort auto-detection of the Claude subscription plan/tier.
//!
//! The status-line JSON does not carry the plan, but Claude Code records the
//! account's rate-limit tier in `~/.claude.json` (e.g.
//! `oauthAccount.organizationRateLimitTier = "default_claude_max_20x"`). We read
//! only that one string field and map it to a short label like `Max (20x)`.
//! Everything here is best-effort: any failure yields `None` and the plan is
//! simply hidden.

/// Detect the plan label from `~/.claude.json`, or `None` if unavailable.
pub fn detect() -> Option<String> {
    let path = dirs::home_dir()?.join(".claude.json");
    let text = std::fs::read_to_string(path).ok()?;
    for key in ["organizationRateLimitTier", "userRateLimitTier", "seatTier"] {
        if let Some(raw) = json_string_value(&text, key) {
            if let Some(label) = pretty_tier(&raw) {
                return Some(label);
            }
        }
    }
    None
}

/// Extract the first non-empty JSON string value for `key` from `text` via a
/// lightweight scan (avoids parsing the whole large account file). Skips `null`
/// or non-string occurrences and keeps looking.
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
                if !val.is_empty() {
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
        return Some(format!("Max ({rest})")); // "20x" -> "Max (20x)"
    }
    let label = match s {
        "max" => "Max".to_string(),
        "pro" => "Pro".to_string(),
        "free" => "Free".to_string(),
        "team" => "Team".to_string(),
        "enterprise" => "Enterprise".to_string(),
        other => {
            let spaced = other.replace('_', " ");
            let mut chars = spaced.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => return None,
            }
        }
    };
    Some(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_tier_strings() {
        assert_eq!(pretty_tier("default_claude_max_20x").as_deref(), Some("Max (20x)"));
        assert_eq!(pretty_tier("default_claude_max_5x").as_deref(), Some("Max (5x)"));
        assert_eq!(pretty_tier("claude_pro").as_deref(), Some("Pro"));
        assert_eq!(pretty_tier("default_claude_pro").as_deref(), Some("Pro"));
        assert_eq!(pretty_tier("team").as_deref(), Some("Team"));
        assert_eq!(pretty_tier("").as_deref(), None);
    }

    #[test]
    fn extracts_json_string_skipping_null() {
        let json = r#"{ "userRateLimitTier": null, "oauthAccount": { "organizationRateLimitTier": "default_claude_max_20x" } }"#;
        assert_eq!(
            json_string_value(json, "organizationRateLimitTier").as_deref(),
            Some("default_claude_max_20x")
        );
        // a key present only as null returns None.
        assert_eq!(json_string_value(json, "userRateLimitTier"), None);
        // a missing key returns None.
        assert_eq!(json_string_value(json, "nope"), None);
    }
}
