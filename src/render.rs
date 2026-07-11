use chrono::{DateTime, Datelike, TimeZone};

pub fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}m", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}k", n / 1_000)
    } else {
        n.to_string()
    }
}

pub fn fmt_reset<Tz: TimeZone>(reset: Option<DateTime<Tz>>, now: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    let Some(r) = reset else {
        return "--".to_string();
    };
    let same_day = r.year() == now.year() && r.ordinal() == now.ordinal();
    if same_day {
        r.format("%-I:%M %p").to_string()
    } else {
        r.format("%b %-d, %-I:%M %p").to_string()
    }
}

/// Human-readable remaining duration for a positive number of seconds.
/// `4d 6h` / `2h 15m` / `45m` / `<1m`. Callers pass only positive durations.
pub fn fmt_countdown(secs: i64) -> String {
    if secs < 60 {
        "<1m".to_string()
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86_400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d {}h", secs / 86_400, (secs % 86_400) / 3600)
    }
}

pub struct Style {
    pub track: u8,
    pub dim: u8,
    pub good: u8,
    pub warn: u8,
    pub crit: u8,
    pub warn_at: u8,
    pub crit_at: u8,
    pub width: usize,
    pub filled: String,
    pub empty: String,
}

/// Threshold color for a bar at `pct`: good below `warn_at`, warn up to
/// `crit_at`, crit at or above `crit_at`.
pub fn fill_color(pct: u8, s: &Style) -> u8 {
    if pct >= s.crit_at {
        s.crit
    } else if pct >= s.warn_at {
        s.warn
    } else {
        s.good
    }
}

pub fn filled_cells(pct: u8, width: usize) -> usize {
    (pct.min(100) as usize * width + 50) / 100
}

pub fn bar(pct: u8, s: &Style) -> String {
    let f = filled_cells(pct, s.width);
    let e = s.width - f;
    let mut out = format!("\x1b[38;5;{}m", fill_color(pct, s));
    for _ in 0..f {
        out.push_str(&s.filled);
    }
    out.push_str(&format!("\x1b[38;5;{}m", s.track));
    for _ in 0..e {
        out.push_str(&s.empty);
    }
    out.push_str("\x1b[0m");
    out
}

/// Bold model-name header line.
pub fn header(model: &str) -> String {
    format!("\x1b[1m{model}\x1b[0m")
}

/// One aligned gauge row: bold label (padded to 8), bar, right-aligned
/// percentage, then the value — columns line up across rows.
pub fn row(label: &str, pct: Option<u8>, value: &str, s: &Style) -> String {
    let dim = format!("\x1b[38;5;{}m", s.dim);
    let (pdisp, bar_pct) = match pct {
        None => (format!("{:>4}", "--"), 0u8),
        Some(p) => (format!("{:>3}%", p), p),
    };
    format!(
        "\x1b[1m{label:<8}\x1b[0m {barstr}  {dim}{pdisp}\x1b[0m  {dim}{value}\x1b[0m",
        barstr = bar(bar_pct, s),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_tokens() {
        assert_eq!(fmt_tokens(0), "0");
        assert_eq!(fmt_tokens(999), "999");
        assert_eq!(fmt_tokens(1_000), "1k");
        assert_eq!(fmt_tokens(15_500), "15k");
        assert_eq!(fmt_tokens(1_000_000), "1.0m");
        assert_eq!(fmt_tokens(1_500_000), "1.5m");
    }

    #[test]
    fn formats_reset() {
        use chrono::{TimeZone, Utc};
        let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();

        let same = Utc.with_ymd_and_hms(2024, 1, 15, 15, 30, 0).unwrap();
        assert_eq!(fmt_reset(Some(same), &now), "3:30 PM");

        let other = Utc.with_ymd_and_hms(2024, 1, 18, 15, 30, 0).unwrap();
        assert_eq!(fmt_reset(Some(other), &now), "Jan 18, 3:30 PM");

        assert_eq!(fmt_reset::<Utc>(None, &now), "--");
    }

    #[test]
    fn formats_countdown() {
        assert_eq!(fmt_countdown(30), "<1m");
        assert_eq!(fmt_countdown(45 * 60), "45m");
        assert_eq!(fmt_countdown(2 * 3600 + 15 * 60), "2h 15m");
        assert_eq!(fmt_countdown(4 * 86_400 + 6 * 3600), "4d 6h");
        assert_eq!(fmt_countdown(60), "1m");
        assert_eq!(fmt_countdown(3600), "1h 0m");
    }

    fn test_style() -> Style {
        Style {
            track: 240,
            dim: 245,
            good: 71,
            warn: 179,
            crit: 167,
            warn_at: 50,
            crit_at: 80,
            width: 12,
            filled: "#".into(),
            empty: ".".into(),
        }
    }

    #[test]
    fn threshold_fill_color() {
        let s = test_style();
        assert_eq!(fill_color(10, &s), 71); // good
        assert_eq!(fill_color(49, &s), 71);
        assert_eq!(fill_color(50, &s), 179); // warn at cutoff
        assert_eq!(fill_color(79, &s), 179);
        assert_eq!(fill_color(80, &s), 167); // crit at cutoff
        assert_eq!(fill_color(100, &s), 167);
    }

    #[test]
    fn computes_filled_cells() {
        assert_eq!(filled_cells(0, 12), 0);
        assert_eq!(filled_cells(100, 12), 12);
        assert_eq!(filled_cells(50, 12), 6);
        assert_eq!(filled_cells(99, 12), 12);
        assert_eq!(filled_cells(4, 12), 0);
        assert_eq!(filled_cells(8, 12), 1);
        assert_eq!(filled_cells(200, 12), 12);
    }

    #[test]
    fn bar_uses_threshold_color() {
        let s = test_style();
        let good = bar(30, &s);
        assert!(good.starts_with("\x1b[38;5;71m")); // green fill
        let crit = bar(90, &s);
        assert!(crit.starts_with("\x1b[38;5;167m")); // red fill
        assert_eq!(bar(50, &s).matches('#').count(), 6);
        assert_eq!(bar(50, &s).matches('.').count(), 6);
        assert!(crit.ends_with("\x1b[0m"));
    }

    #[test]
    fn header_is_bold() {
        assert_eq!(header("Opus 4.8"), "\x1b[1mOpus 4.8\x1b[0m");
    }

    #[test]
    fn row_with_pct() {
        let s = test_style();
        let r = row("5h", Some(42), "resets in 2h 15m", &s);
        assert!(r.contains("\x1b[1m5h      \x1b[0m")); // bold label padded to 8
        assert!(r.contains(" 42%")); // right-aligned in width 3 + %
        assert!(r.contains("resets in 2h 15m"));
    }

    #[test]
    fn row_without_pct() {
        let s = test_style();
        let r = row("7d", None, "--", &s);
        assert!(r.contains("  --"));
        assert!(!r.contains('%'));
    }
}
